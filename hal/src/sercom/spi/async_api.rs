use crate::{
    pac::Interrupt,
    sercom::{
        spi::{Capability, DataWidth, Duplex, Error, Flags, Rx, Spi, Tx, ValidConfig},
        InterruptNumbers, Interrupts, Sercom,
    },
    typelevel::NoneT,
};
use core::task::Poll;
use cortex_m::interrupt::InterruptNumber;
use cortex_m_interrupt::NvicInterruptRegistration;
use num_traits::{AsPrimitive, PrimInt};

impl<C, A, S> Spi<C, A>
where
    C: ValidConfig<Sercom = S>,
    A: Capability,
    S: Sercom,
{
    /// Turn an [`Spi`] into a [`SpiFuture`]. In cases where the underlying
    /// [`Spi`] is [`Duplex`], reading words need to be accompanied with sending
    /// a no-op word. By default it is set to 0x00, but you can configure it
    /// by using the [`nop_word`](SpiFuture::nop_word) method.
    #[cfg(feature = "thumbv6")]
    #[inline]
    pub fn into_future<N, I>(self, interrupts: Interrupts<N, I>) -> SpiFuture<C, A, N>
    where
        C::Word: Copy,
        u8: AsPrimitive<C::Word>,
        I: NvicInterruptRegistration<N>,
        N: InterruptNumber,
    {
        let irq_numbers = interrupts.occupy(S::on_interrupt_spi);

        SpiFuture {
            spi: self,
            nop_word: 0x00_u8.as_(),
            _irqs: irq_numbers,
            _rx_channel: NoneT,
            _tx_channel: NoneT,
        }
    }

    /// Turn an [`Spi`] into an [`SpiFuture`]. In cases where the underlying
    /// [`Spi`] is [`Duplex`], reading words need to be accompanied with sending
    /// a no-op word. By default it is set to 0x00, but you can configure it
    /// by using the [`nop_word`](SpiFuture::nop_word) method.
    #[cfg(feature = "thumbv7")]
    #[inline]
    pub fn into_future<N, N0, N1, N2, NOther>(
        self,
        interrupts: Interrupts<N, N0, N1, N2, NOther>,
    ) -> SpiFuture<C, A, N>
    where
        C::Word: Copy,
        u8: AsPrimitive<C::Word>,
        N: InterruptNumber,
        N0: NvicInterruptRegistration<N>,
        N1: NvicInterruptRegistration<N>,
        N2: NvicInterruptRegistration<N>,
        NOther: NvicInterruptRegistration<N>,
    {
        let irq_numbers = interrupts.occupy(S::on_interrupt_spi);

        SpiFuture {
            spi: self,
            nop_word: 0x00_u8.as_(),
            _irqs: irq_numbers,
            _rx_channel: NoneT,
            _tx_channel: NoneT,
        }
    }
}

/// `async` version of [`Spi`].
///
/// Create this struct by calling [`I2c::into_future`](I2c::into_future).
pub struct SpiFuture<C, A, N, R = NoneT, T = NoneT>
where
    C: ValidConfig,
    A: Capability,
    N: InterruptNumber,
{
    spi: Spi<C, A>,
    nop_word: C::Word,
    _irqs: InterruptNumbers<N>,
    _rx_channel: R,
    _tx_channel: T,
}

#[cfg(feature = "defmt")]
impl<C, A, N, R, T> defmt::Format for SpiFuture<C, A, N, R, T>
where
    C: ValidConfig,
    A: Capability,
    N: InterruptNumber,
{
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SpiFuture defmt shim\n");
    }
}

/// Convenience type for a [`SpiFuture`] with RX and TX capabilities
pub type SpiFutureDuplex<C> = SpiFuture<C, Duplex, Interrupt>;

/// Convenience type for a [`SpiFuture`] with RX capabilities
pub type SpiFutureRx<C> = SpiFuture<C, Rx, Interrupt>;

/// Convenience type for a [`SpiFuture`] with TX capabilities
pub type SpiFutureTx<C> = SpiFuture<C, Tx, Interrupt>;

#[cfg(feature = "dma")]
/// Convenience type for a [`SpiFuture`] with RX and TX capabilities in DMA
/// mode. The type parameter `R` represents the RX DMA channel ID (`ChX`), and
/// `T` represents the TX DMA channel ID.
pub type SpiFutureDuplexDma<C, R, T> = SpiFuture<
    C,
    Duplex,
    Interrupt,
    crate::dmac::Channel<R, crate::dmac::ReadyFuture>,
    crate::dmac::Channel<T, crate::dmac::ReadyFuture>,
>;

#[cfg(feature = "dma")]
/// Convenience type for a [`SpiFuture`] with RX capabilities in DMA mode. The
/// type parameter `R` represents the RX DMA channel ID (`ChX`).
pub type SpiFutureRxDma<C, R> =
    SpiFuture<C, Rx, Interrupt, crate::dmac::Channel<R, crate::dmac::ReadyFuture>, NoneT>;

#[cfg(feature = "dma")]
/// Convenience type for a [`SpiFuture`] with TX capabilities in DMA mode. The
/// type parameter `T` represents the TX DMA channel ID (`ChX`).
pub type SpiFutureTxDma<C, T> =
    SpiFuture<C, Tx, Interrupt, NoneT, crate::dmac::Channel<T, crate::dmac::ReadyFuture>>;

impl<C, A, N, S, R, T> SpiFuture<C, A, N, R, T>
where
    C: ValidConfig<Sercom = S>,
    A: Capability,
    N: InterruptNumber,
    S: Sercom,
{
    /// Return the underlying [`Spi`].
    pub fn free(self) -> Spi<C, A> {
        self.spi
    }

    /// Configure the no-op word to send when doing read-only transactions.
    pub fn nop_word(&mut self, word: C::Word) {
        self.nop_word = word;
    }

    #[inline]
    async fn wait_flags(&mut self, flags_to_wait: Flags) {
        core::future::poll_fn(|cx| {
            // Scope maybe_pending so we don't forget to re-poll the register later down.
            {
                let maybe_pending = self.spi.config.as_ref().regs.read_flags();
                if flags_to_wait.intersects(maybe_pending) {
                    return Poll::Ready(());
                }
            }

            self.spi.disable_interrupts(Flags::all());

            if flags_to_wait.intersects(Flags::RX) {
                S::rx_waker().register(cx.waker());
            }
            if flags_to_wait.intersects(Flags::TX) {
                S::tx_waker().register(cx.waker());
            }

            self.spi.enable_interrupts(flags_to_wait);
            let maybe_pending = self.spi.config.as_ref().regs.read_flags();

            if !flags_to_wait.intersects(maybe_pending) {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
    }
}

impl<C, N, S> SpiFuture<C, Duplex, N>
where
    C: ValidConfig<Sercom = S>,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    DataWidth: AsPrimitive<C::Word>,
    N: InterruptNumber,
    S: Sercom,
{
    /// Read words into a buffer asynchronously, word by word. Since we are
    /// using a [`Duplex`] [`SpiFuture`], we need to send a word simultaneously
    /// while receiving one. This `no-op` word is configurable via the
    /// [`nop_word`](Self::nop_word) method.
    #[inline]
    pub async fn read(&mut self, buffer: &mut [C::Word]) -> Result<(), Error> {
        for byte in buffer.iter_mut() {
            *byte = self.simultaneous_word(self.nop_word).await?;
        }

        Ok(())
    }

    /// Write words from a buffer asynchronously, word by word
    #[inline]
    pub async fn write(&mut self, words: &[C::Word]) -> Result<(), Error> {
        // When in Duplex mode, read as many words as we write to avoid buffer overflows
        for word in words {
            let _ = self.simultaneous_word(*word).await?;
        }

        Ok(())
    }
}

impl<C, N, S, R, T> SpiFuture<C, Duplex, N, R, T>
where
    C: ValidConfig<Sercom = S>,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    DataWidth: AsPrimitive<C::Word>,
    N: InterruptNumber,
    S: Sercom,
{
    /// Read and write a single word to the bus simultaneously.
    pub async fn simultaneous_word(&mut self, to_send: C::Word) -> Result<C::Word, Error> {
        self.wait_flags(Flags::DRE).await;
        self.spi.read_flags_errors()?;
        unsafe {
            self.spi.write_data(to_send.as_());
        }

        self.wait_flags(Flags::TXC).await;

        self.wait_flags(Flags::RXC).await;
        let word = unsafe { self.spi.read_data().as_() };

        Ok(word)
    }

    /// Perform a transfer, word by word. No-op words will be written if `read`
    /// is longer than `write`. Extra words are ignored if `write` is longer
    /// than `read`.
    async fn transfer_word_by_word(
        &mut self,
        read: &mut [C::Word],
        write: &[C::Word],
    ) -> Result<(), Error> {
        let nop_word = self.nop_word;
        for (r, w) in read
            .iter_mut()
            .zip(write.iter().chain(core::iter::repeat(&nop_word)))
        {
            *r = self.simultaneous_word(*w).await?;
        }

        Ok(())
    }
}

impl<C, A, N> AsRef<Spi<C, A>> for SpiFuture<C, A, N>
where
    C: ValidConfig,
    A: Capability,
    N: InterruptNumber,
{
    #[inline]
    fn as_ref(&self) -> &Spi<C, A> {
        &self.spi
    }
}

impl<C, A, N> AsMut<Spi<C, A>> for SpiFuture<C, A, N>
where
    C: ValidConfig,
    A: Capability,
    N: InterruptNumber,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Spi<C, A> {
        &mut self.spi
    }
}

#[cfg(feature = "nightly")]
mod impl_ehal {
    use super::*;
    use crate::sercom::spi::{Error, Size};
    use embedded_hal_async::spi::{ErrorType, SpiBus, SpiBusFlush, SpiBusRead, SpiBusWrite};

    impl<C, A, N, S, R, T> ErrorType for SpiFuture<C, A, N, R, T>
    where
        C: ValidConfig<Sercom = S>,
        C::Word: PrimInt + AsPrimitive<DataWidth>,
        DataWidth: AsPrimitive<C::Word>,
        A: Capability,
        N: InterruptNumber,
        S: Sercom,
    {
        type Error = Error;
    }

    impl<C, A, N, S, R, T> SpiBusFlush for SpiFuture<C, A, N, R, T>
    where
        C: ValidConfig<Sercom = S>,
        C::Word: PrimInt + AsPrimitive<DataWidth>,
        DataWidth: AsPrimitive<C::Word>,
        A: Capability,
        N: InterruptNumber,
        S: Sercom,
    {
        async fn flush(&mut self) -> Result<(), Self::Error> {
            // Wait for all transactions to complete, ignoring buffer overflow errors.
            self.wait_flags(Flags::TXC | Flags::RXC).await;
            Ok(())
        }
    }

    impl<C, N, S, W> SpiBusWrite<W> for SpiFuture<C, Duplex, N>
    where
        C: ValidConfig<Sercom = S, Word = W>,
        C::Word: PrimInt + AsPrimitive<DataWidth>,
        DataWidth: AsPrimitive<C::Word>,
        N: InterruptNumber,
        S: Sercom + 'static,
    {
        async fn write(&mut self, words: &[C::Word]) -> Result<(), Self::Error> {
            self.write(words).await?;
            Ok(())
        }
    }

    #[cfg(feature = "dma")]
    impl<C, N, S, W, R, T> SpiBusWrite<W> for SpiFuture<C, Duplex, N, R, T>
    where
        C: ValidConfig<Sercom = S, Word = W>,
        C::Word: PrimInt + AsPrimitive<DataWidth> + crate::dmac::Beat,
        C::Size: crate::sercom::spi::Size<Word = C::Word>,
        DataWidth: AsPrimitive<C::Word>,
        N: InterruptNumber,
        S: Sercom + 'static,
        R: crate::dmac::AnyChannel<Status = crate::dmac::ReadyFuture>,
        T: crate::dmac::AnyChannel<Status = crate::dmac::ReadyFuture>,
    {
        async fn write(&mut self, words: &[C::Word]) -> Result<(), Self::Error> {
            self.write(words).await?;
            Ok(())
        }
    }

    impl<C, N, S, W> SpiBusRead<W> for SpiFuture<C, Duplex, N>
    where
        C: ValidConfig<Sercom = S, Word = W>,
        C::Word: PrimInt + AsPrimitive<DataWidth>,
        DataWidth: AsPrimitive<C::Word>,
        N: InterruptNumber,
        S: Sercom + 'static,
    {
        async fn read(&mut self, words: &mut [C::Word]) -> Result<(), Self::Error> {
            self.read(words).await?;
            Ok(())
        }
    }

    #[cfg(feature = "dma")]
    impl<C, N, S, W, R, T> SpiBusRead<W> for SpiFuture<C, Duplex, N, R, T>
    where
        C: ValidConfig<Sercom = S, Word = W>,
        C::Word: PrimInt + AsPrimitive<DataWidth> + crate::dmac::Beat,
        C::Size: crate::sercom::spi::Size<Word = C::Word>,
        DataWidth: AsPrimitive<C::Word>,
        N: InterruptNumber,
        S: Sercom + 'static,
        R: crate::dmac::AnyChannel<Status = crate::dmac::ReadyFuture>,
        T: crate::dmac::AnyChannel<Status = crate::dmac::ReadyFuture>,
    {
        async fn read(&mut self, words: &mut [C::Word]) -> Result<(), Self::Error> {
            self.read(words).await?;
            Ok(())
        }
    }

    impl<C, N, S, W> SpiBus<W> for SpiFuture<C, Duplex, N>
    where
        C: ValidConfig<Sercom = S, Word = W>,
        C::Word: PrimInt + AsPrimitive<DataWidth>,
        DataWidth: AsPrimitive<C::Word>,
        N: InterruptNumber,
        S: Sercom + 'static,
        Self: SpiBusWrite<W> + SpiBusRead<W> + ErrorType<Error = Error>,
    {
        async fn transfer(&mut self, read: &mut [W], write: &[W]) -> Result<(), Self::Error> {
            self.transfer_word_by_word(read, write).await?;
            Ok(())
        }

        async fn transfer_in_place(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
            // Can only ever do word-by-word to avoid DMA race conditions
            for word in words {
                let read = self.simultaneous_word(*word).await?;
                *word = read;
            }

            Ok(())
        }
    }

    #[cfg(feature = "dma")]
    impl<C, N, W, S, R, T> SpiBus<W> for SpiFuture<C, Duplex, N, R, T>
    where
        C: ValidConfig<Sercom = S, Word = W>,
        C::Word: PrimInt + AsPrimitive<DataWidth> + crate::dmac::Beat + Copy,
        C::Size: Size<Word = C::Word>,
        DataWidth: AsPrimitive<C::Word>,
        N: InterruptNumber,
        R: crate::dmac::AnyChannel<Status = crate::dmac::ReadyFuture>,
        T: crate::dmac::AnyChannel<Status = crate::dmac::ReadyFuture>,
        S: Sercom + 'static,
    {
        async fn transfer(&mut self, read: &mut [W], write: &[W]) -> Result<(), Self::Error> {
            self.transfer_dma(Some(read), Some(write)).await?;
            Ok(())
        }

        async fn transfer_in_place(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
            // Can only ever do word-by-word to avoid DMA race conditions
            for word in words {
                let read = self.simultaneous_word(*word).await?;
                *word = read;
            }

            Ok(())
        }
    }
}

#[cfg(feature = "dma")]
mod dma {
    use super::*;
    use crate::{
        dmac::{AnyChannel, Beat, Buffer, ReadyFuture},
        sercom::{
            async_dma::{read_dma, read_dma_buffer, write_dma, write_dma_buffer, SercomPtr},
            spi::Size,
        },
    };

    struct DummyBuffer<T: Beat> {
        word: T,
        length: usize,
    }

    /// Sink/source buffer to use for unidirectional SPI-DMA transfers.
    ///
    /// When reading/writing from a [`Duplex`] [`SpiFuture`] with DMA enabled,
    /// we must always read and write the same number of words, regardless of
    /// whether we care about the result (ie, for [`write`], we discard the read
    /// words, whereas for [`read`], we must send a no-op word).
    ///
    /// This [`Buffer`] implementation provides a source/sink for a single word,
    /// but with a variable length.
    impl<T: Beat> DummyBuffer<T> {
        fn new(word: T, length: usize) -> Self {
            Self { word, length }
        }
    }

    unsafe impl<T: Beat> Buffer for DummyBuffer<T> {
        type Beat = T;

        fn incrementing(&self) -> bool {
            false
        }

        fn buffer_len(&self) -> usize {
            self.length
        }

        fn dma_ptr(&mut self) -> *mut Self::Beat {
            &mut self.word as *mut _
        }
    }

    impl<C, N, S, R> SpiFuture<C, Rx, N, R, NoneT>
    where
        C: ValidConfig<Sercom = S>,
        C::Word: PrimInt + AsPrimitive<DataWidth>,
        DataWidth: AsPrimitive<C::Word>,
        N: InterruptNumber,
        S: Sercom,
    {
        /// Add a DMA channel for receiving transactions
        #[inline]
        pub fn with_rx_dma_channel<Chan: AnyChannel<Status = ReadyFuture>>(
            self,
            rx_channel: Chan,
        ) -> SpiFuture<C, Rx, N, Chan, NoneT> {
            SpiFuture {
                spi: self.spi,
                nop_word: self.nop_word,
                _irqs: self._irqs,
                _tx_channel: self._tx_channel,
                _rx_channel: rx_channel,
            }
        }
    }

    impl<C, N, S, T> SpiFuture<C, Tx, N, NoneT, T>
    where
        C: ValidConfig<Sercom = S>,
        C::Word: PrimInt + AsPrimitive<DataWidth>,
        DataWidth: AsPrimitive<C::Word>,
        N: InterruptNumber,
        S: Sercom,
    {
        /// Add a DMA channel for receiving transactions
        #[inline]
        pub fn with_tx_dma_channel<Chan: AnyChannel<Status = ReadyFuture>>(
            self,
            tx_channel: Chan,
        ) -> SpiFuture<C, Tx, N, NoneT, Chan> {
            SpiFuture {
                spi: self.spi,
                nop_word: self.nop_word,
                _irqs: self._irqs,
                _rx_channel: self._rx_channel,
                _tx_channel: tx_channel,
            }
        }
    }

    impl<C, N, S, R, T> SpiFuture<C, Duplex, N, R, T>
    where
        C: ValidConfig<Sercom = S>,
        C::Word: PrimInt + AsPrimitive<DataWidth>,
        DataWidth: AsPrimitive<C::Word>,
        N: InterruptNumber,
        S: Sercom,
    {
        /// Add a DMA channel for receiving transactions
        #[inline]
        pub fn with_dma_channels<ChanRx, ChanTx>(
            self,
            rx_channel: ChanRx,
            tx_channel: ChanTx,
        ) -> SpiFuture<C, Duplex, N, ChanRx, ChanTx>
        where
            ChanRx: AnyChannel<Status = ReadyFuture>,
            ChanTx: AnyChannel<Status = ReadyFuture>,
        {
            SpiFuture {
                spi: self.spi,
                nop_word: self.nop_word,
                _irqs: self._irqs,
                _rx_channel: rx_channel,
                _tx_channel: tx_channel,
            }
        }
    }

    impl<C, N, S, R, T> SpiFuture<C, Duplex, N, R, T>
    where
        C: ValidConfig<Sercom = S>,
        C::Word: PrimInt + AsPrimitive<DataWidth> + Beat,
        C::Size: Size<Word = C::Word>,
        DataWidth: AsPrimitive<C::Word>,
        N: InterruptNumber,
        R: AnyChannel<Status = ReadyFuture>,
        T: AnyChannel<Status = ReadyFuture>,
        S: Sercom + 'static,
    {
        fn sercom_ptr(&self) -> SercomPtr<C::Word> {
            SercomPtr(self.spi.data_ptr())
        }

        /// Simultaneously transfer words in and out of the SPI bus.
        ///
        /// If `read` and `write` are the same length, we can send everything at
        /// once, and thus DMA transfers can be utilized. If they are of
        /// different lengths, we need to send word by word, so that we
        /// can pad `write` if it is longer than `read`.
        ///
        /// One or both of `read` and `write` can be specified. In any case,
        /// words will simultaneously be sent and received, to avoid buffer
        /// overflow errors. If `write` is omitted, `self.nop_word` will be
        /// sent. If `read` is omitted, the words sent by the device will still
        /// be read, but immediately discarded.
        #[inline]
        pub(super) async fn transfer_dma(
            &mut self,
            read: Option<&mut [C::Word]>,
            write: Option<&[C::Word]>,
        ) -> Result<(), Error> {
            assert!(read.is_some() || write.is_some());

            let spi_ptr = self.sercom_ptr();

            match (read, write) {
                (Some(r), Some(w)) => {
                    if r.len() == w.len() {
                        let tx_fut = write_dma::<_, S>(&mut self._rx_channel, spi_ptr.clone(), w);
                        let rx_fut = read_dma::<_, S>(&mut self._tx_channel, spi_ptr, r);

                        let (read_res, write_res) = futures::join!(rx_fut, tx_fut);
                        write_res.and(read_res).map_err(Error::Dma)?;
                    } else {
                        // Short circuit if we got a length mismatch, as we have to send word by
                        // word
                        self.transfer_word_by_word(r, w).await?;
                        return Ok(());
                    }
                }

                (Some(r), None) => {
                    let source = DummyBuffer::new(self.nop_word, r.len());
                    let rx_fut = read_dma::<_, S>(&mut self._rx_channel, spi_ptr.clone(), r);
                    let tx_fut =
                        write_dma_buffer::<_, _, S>(&mut self._tx_channel, spi_ptr, source);

                    let (read_res, write_res) = futures::join!(rx_fut, tx_fut);
                    write_res.and(read_res).map_err(Error::Dma)?;
                }

                (None, Some(w)) => {
                    // Use a random value as the sink buffer since we're just going to discard the
                    // read words
                    let sink = DummyBuffer::new(0xFF.as_(), w.len());
                    let rx_fut =
                        read_dma_buffer::<_, _, S>(&mut self._rx_channel, spi_ptr.clone(), sink);
                    let tx_fut = write_dma::<_, S>(&mut self._tx_channel, spi_ptr, w);

                    let (read_res, write_res) = futures::join!(rx_fut, tx_fut);
                    write_res.and(read_res).map_err(Error::Dma)?;
                }

                _ => panic!("Must provide at lease one buffer"),
            }

            self.spi.read_flags_errors()?;

            // Wait for transmission to complete. If we don't do that, we might return too
            // early and disable the CS line, resulting in a corrupted transfer.
            self.wait_flags(Flags::TXC).await;

            Ok(())
        }

        /// Read words into a buffer asynchronously, using DMA.
        #[inline]
        pub async fn read(&mut self, words: &mut [C::Word]) -> Result<(), Error> {
            self.transfer_dma(Some(words), None).await
        }

        /// Write words from a buffer asynchronously, using DMA.
        #[inline]
        pub async fn write(&mut self, words: &[C::Word]) -> Result<(), Error> {
            self.transfer_dma(None, Some(words)).await
        }
    }
}
