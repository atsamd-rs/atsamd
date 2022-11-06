use crate::{
    pac::Interrupt,
    sercom::{
        spi::{
            Capability, DataWidth, Duplex, Error, Flags, Receive, Rx, Spi, Transmit, Tx,
            ValidConfig,
        },
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
    /// Turn an [`Spi`] into a [`SpiFuture`].
    #[cfg(any(feature = "samd11", feature = "samd21"))]
    #[inline]
    pub fn into_future<N, I>(self, interrupts: Interrupts<N, I>) -> SpiFuture<C, A, N>
    where
        I: NvicInterruptRegistration<N>,
        N: InterruptNumber,
    {
        let irq_numbers = interrupts.occupy(S::on_interrupt_i2c);

        SpiFuture {
            spi: self,
            _irqs: irq_numbers,
            _rx_channel: NoneT,
            _tx_channel: NoneT,
        }
    }

    /// Turn an [`Spi`] into an [`SpiFuture`]
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn into_future<N, N0, N1, N2, NOther>(
        self,
        interrupts: Interrupts<N, N0, N1, N2, NOther>,
    ) -> SpiFuture<C, A, N>
    where
        N: InterruptNumber,
        N0: NvicInterruptRegistration<N>,
        N1: NvicInterruptRegistration<N>,
        N2: NvicInterruptRegistration<N>,
        NOther: NvicInterruptRegistration<N>,
    {
        let irq_numbers = interrupts.occupy(S::on_interrupt_i2c);

        SpiFuture {
            spi: self,
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
    _irqs: InterruptNumbers<N>,
    _rx_channel: R,
    _tx_channel: T,
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

impl<C, A, N, S, T> SpiFuture<C, A, N, NoneT, T>
where
    C: ValidConfig<Sercom = S>,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    DataWidth: AsPrimitive<C::Word>,
    A: Receive,
    N: InterruptNumber,
    S: Sercom,
{
    /// Read words into a buffer asynchronously, word by word
    #[inline]
    pub async fn read(&mut self, buffer: &mut [C::Word]) -> Result<(), Error> {
        for byte in buffer {
            *byte = self.read_word().await?;
        }

        Ok(())
    }
}

impl<C, A, N, S, R, T> SpiFuture<C, A, N, R, T>
where
    C: ValidConfig<Sercom = S>,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    DataWidth: AsPrimitive<C::Word>,
    A: Receive,
    N: InterruptNumber,
    S: Sercom,
{
    #[cfg(feature = "dma")]
    /// Add a DMA channel for receiving transactions
    #[inline]
    pub fn with_rx_dma_channel<Chan: crate::dmac::AnyChannel<Status = crate::dmac::ReadyFuture>>(
        self,
        rx_channel: Chan,
    ) -> SpiFuture<C, A, N, Chan, T> {
        SpiFuture {
            spi: self.spi,
            _irqs: self._irqs,
            _tx_channel: self._tx_channel,
            _rx_channel: rx_channel,
        }
    }

    /// Read a single word asynchronously.
    #[inline]
    pub async fn read_word(&mut self) -> Result<C::Word, Error> {
        self.wait_flags(Flags::RXC).await;
        self.spi.read_flags_errors()?;
        let word = unsafe { self.spi.read_data().as_() };
        Ok(word)
    }
}

impl<C, A, N, S, R> SpiFuture<C, A, N, R, NoneT>
where
    C: ValidConfig<Sercom = S>,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    DataWidth: AsPrimitive<C::Word>,
    A: Transmit,
    N: InterruptNumber,
    S: Sercom,
{
    /// Write words from a buffer asynchronously, word by word
    #[inline]
    pub async fn write(&mut self, words: &[C::Word]) -> Result<(), Error> {
        for word in words {
            self.write_word(*word).await?;
        }

        Ok(())
    }
}

impl<C, A, N, S, R, T> SpiFuture<C, A, N, R, T>
where
    C: ValidConfig<Sercom = S>,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    DataWidth: AsPrimitive<C::Word>,
    A: Transmit,
    N: InterruptNumber,
    S: Sercom,
{
    #[cfg(feature = "dma")]
    /// Add a DMA channel for sending transactions
    #[inline]
    pub fn with_tx_dma_channel<Chan: crate::dmac::AnyChannel<Status = crate::dmac::ReadyFuture>>(
        self,
        tx_channel: Chan,
    ) -> SpiFuture<C, A, N, R, Chan> {
        SpiFuture {
            spi: self.spi,
            _irqs: self._irqs,
            _rx_channel: self._rx_channel,
            _tx_channel: tx_channel,
        }
    }
    /// Write a single word asynchronously.
    pub async fn write_word(&mut self, word: C::Word) -> Result<(), Error> {
        self.wait_flags(Flags::DRE).await;
        self.spi.read_flags_errors()?;
        unsafe {
            self.spi.write_data(word.as_());
        }
        Ok(())
    }
}

impl<C, A, N, S, R, T> SpiFuture<C, A, N, R, T>
where
    C: ValidConfig<Sercom = S>,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    DataWidth: AsPrimitive<C::Word>,
    A: Receive + Transmit,
    N: InterruptNumber,
    S: Sercom,
{
    /// Read and write a single word to the bus simultaneously.
    pub async fn simultaneous_word(&mut self, to_send: C::Word) -> Result<C::Word, Error> {
        // TODO SAFETY prove that this really safe?
        let mut rx_half = unsafe { core::ptr::read(self) };
        let tx_half = self;

        let (write_res, read_res) =
            futures::join!(tx_half.write_word(to_send), rx_half.read_word());
        core::mem::forget(rx_half);
        write_res.and(read_res)
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
    use crate::sercom::spi::Error;
    use core::future::Future;
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
        type FlushFuture<'a> = impl Future<Output= Result<(), Self::Error>> + 'a where Self: 'a;

        fn flush(&mut self) -> Self::FlushFuture<'_> {
            // Wait for all transactions to complete, ignoring buffer overflow errors.
            async {
                self.wait_flags(Flags::TXC | Flags::RXC).await;
                Ok(())
            }
        }
    }

    impl<C, A, N, S, W> SpiBusWrite<W> for SpiFuture<C, A, N>
    where
        C: ValidConfig<Sercom = S, Word = W>,
        C::Word: PrimInt + AsPrimitive<DataWidth>,
        DataWidth: AsPrimitive<C::Word>,
        A: Transmit,
        N: InterruptNumber,
        S: Sercom + 'static,
    {
        type WriteFuture<'a> = impl Future<Output= Result<(), Self::Error>> + 'a where Self: 'a;

        fn write<'a>(&'a mut self, words: &'a [C::Word]) -> Self::WriteFuture<'a> {
            self.write(words)
        }
    }

    #[cfg(feature = "dma")]
    impl<C, A, N, S, W, R, T> SpiBusWrite<W> for SpiFuture<C, A, N, R, T>
    where
        C: ValidConfig<Sercom = S, Word = W>,
        C::Word: PrimInt + AsPrimitive<DataWidth> + crate::dmac::Beat,
        C::Size: crate::sercom::spi::Size<Word = C::Word>,
        DataWidth: AsPrimitive<C::Word>,
        A: Transmit,
        N: InterruptNumber,
        S: Sercom + 'static,
        T: crate::dmac::AnyChannel<Status = crate::dmac::ReadyFuture>,
    {
        type WriteFuture<'a> = impl Future<Output= Result<(), Self::Error>> + 'a where Self: 'a;

        fn write<'a>(&'a mut self, words: &'a [C::Word]) -> Self::WriteFuture<'a> {
            self.write(words)
        }
    }

    impl<C, A, N, S, W> SpiBusRead<W> for SpiFuture<C, A, N>
    where
        C: ValidConfig<Sercom = S, Word = W>,
        C::Word: PrimInt + AsPrimitive<DataWidth>,
        DataWidth: AsPrimitive<C::Word>,
        A: Receive,
        N: InterruptNumber,
        S: Sercom + 'static,
    {
        type ReadFuture<'a> = impl Future<Output= Result<(), Self::Error>> + 'a where Self: 'a;

        fn read<'a>(&'a mut self, words: &'a mut [C::Word]) -> Self::ReadFuture<'a> {
            self.read(words)
        }
    }

    #[cfg(feature = "dma")]
    impl<C, A, N, S, W, R, T> SpiBusRead<W> for SpiFuture<C, A, N, R, T>
    where
        C: ValidConfig<Sercom = S, Word = W>,
        C::Word: PrimInt + AsPrimitive<DataWidth> + crate::dmac::Beat,
        C::Size: crate::sercom::spi::Size<Word = C::Word>,
        DataWidth: AsPrimitive<C::Word>,
        A: Receive,
        N: InterruptNumber,
        S: Sercom + 'static,
        R: crate::dmac::AnyChannel<Status = crate::dmac::ReadyFuture>,
    {
        type ReadFuture<'a> = impl Future<Output= Result<(), Self::Error>> + 'a where Self: 'a;

        fn read<'a>(&'a mut self, words: &'a mut [C::Word]) -> Self::ReadFuture<'a> {
            self.read(words)
        }
    }

    impl<C, A, N, S, W, R, T> SpiBus<W> for SpiFuture<C, A, N, R, T>
    where
        C: ValidConfig<Sercom = S, Word = W>,
        C::Word: PrimInt + AsPrimitive<DataWidth>,
        DataWidth: AsPrimitive<C::Word>,
        A: Transmit + Receive,
        N: InterruptNumber,
        S: Sercom + 'static,
        SpiFuture<C, A, N, R, T>: SpiBusWrite<W> + SpiBusRead<W> + ErrorType<Error = Error>,
    {
        type TransferFuture<'a> = impl Future<Output= Result<(), Self::Error>> + 'a where Self: 'a;

        fn transfer<'a>(
            &'a mut self,
            read: &'a mut [W],
            write: &'a [W],
        ) -> Self::TransferFuture<'a> {
            async {
                // If `read` and `write` are the same length, we can send everything at once.
                // This way we can use DMA transfers. If they are different
                // lengths, we need to send word by word, so that we can pad
                // `write` if it is longer than `read`.
                if write.len() == read.len() {
                    // TODO SAFETY prove that this really safe?
                    let mut rx_half = unsafe { core::ptr::read(self) };
                    let tx_half = self;

                    let (write_res, read_res) =
                        futures::join!(tx_half.write(write), rx_half.read(read));
                    core::mem::forget(rx_half);
                    write_res.and(read_res)
                }
                // TODO if read is longer than write, we keep sending zeros.
                // Should make this configurable.
                else {
                    // Pad `write` if it is longer than `read`
                    for (r, w) in read
                        .iter_mut()
                        .zip(write.iter().chain(core::iter::repeat(&0.as_())))
                    {
                        *r = self.simultaneous_word(*w).await?;
                    }

                    Ok(())
                }
            }
        }

        type TransferInPlaceFuture<'a> = impl Future<Output= Result<(), Self::Error>> + 'a where Self: 'a;

        fn transfer_in_place<'a>(
            &'a mut self,
            words: &'a mut [W],
        ) -> Self::TransferInPlaceFuture<'a> {
            async {
                for word in words {
                    self.write_word(*word).await?;
                    *word = self.read_word().await?;
                }

                Ok(())
            }
        }
    }
}

#[cfg(feature = "dma")]
mod dma {
    use super::*;
    use crate::{
        dmac::{AnyChannel, Beat, ReadyFuture},
        sercom::{
            async_dma::{read_dma, write_dma, SercomPtr},
            spi::{self, Size},
        },
    };

    impl<C, A, N, S, R, T> SpiFuture<C, A, N, R, T>
    where
        C: ValidConfig<Sercom = S>,
        C::Word: PrimInt + AsPrimitive<DataWidth> + Beat,
        C::Size: Size<Word = C::Word>,
        DataWidth: AsPrimitive<C::Word>,
        A: Capability,
        N: InterruptNumber,
        S: Sercom + 'static,
    {
        fn sercom_ptr(&self) -> SercomPtr<C::Word> {
            SercomPtr(self.spi.data_ptr())
        }
    }

    impl<C, A, N, S, R, T> SpiFuture<C, A, N, R, T>
    where
        C: ValidConfig<Sercom = S>,
        C::Word: PrimInt + AsPrimitive<DataWidth> + Beat,
        C::Size: Size<Word = C::Word>,
        DataWidth: AsPrimitive<C::Word>,
        A: Receive,
        N: InterruptNumber,
        S: Sercom + 'static,
        R: AnyChannel<Status = ReadyFuture>,
    {
        /// Read words into a buffer asynchronously, using DMA.
        #[inline]
        pub async fn read(&mut self, words: &mut [C::Word]) -> Result<(), Error> {
            // SAFETY: Using SercomPtr is safe because we hold on
            // to &mut self as long as the transfer hasn't completed.
            let spi_ptr = self.sercom_ptr();

            read_dma::<_, S>(&mut self._rx_channel, spi_ptr, words)
                .await
                .map_err(spi::Error::Dma)?;

            Ok(())
        }
    }

    impl<C, A, N, S, R, T> SpiFuture<C, A, N, R, T>
    where
        C: ValidConfig<Sercom = S>,
        C::Word: PrimInt + AsPrimitive<DataWidth> + Beat,
        C::Size: Size<Word = C::Word>,
        DataWidth: AsPrimitive<C::Word>,
        A: Transmit,
        N: InterruptNumber,
        S: Sercom + 'static,
        T: AnyChannel<Status = ReadyFuture>,
    {
        /// Write words from a buffer asynchronously, using DMA.
        #[inline]
        pub async fn write(&mut self, words: &[C::Word]) -> Result<(), Error> {
            // SAFETY: Using SercomPtr and ImmutableSlice is safe because we hold on
            // to &mut self and words as long as the transfer hasn't completed.
            let spi_ptr = self.sercom_ptr();

            write_dma::<_, S>(&mut self._tx_channel, spi_ptr, words)
                .await
                .map_err(spi::Error::Dma)?;

            Ok(())
        }
    }
}
