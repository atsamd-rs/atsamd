use crate::{
    async_hal::interrupts::{Binding, Handler, InterruptSource},
    sercom::{
        uart::{
            Capability, DataReg, Duplex, Error, Flags, Receive, Rx, RxDuplex, SingleOwner,
            Transmit, Tx, TxDuplex, Uart, ValidConfig,
        },
        Sercom,
    },
    typelevel::NoneT,
};
use atsamd_hal_macros::hal_macro_helper;
use core::{marker::PhantomData, task::Poll};
use num_traits::AsPrimitive;

/// Interrupt handler for async UART operarions
pub struct InterruptHandler<S: Sercom> {
    _private: (),
    _sercom: PhantomData<S>,
}

impl<S: Sercom> crate::typelevel::Sealed for InterruptHandler<S> {}

impl<S: Sercom> Handler<S::Interrupt> for InterruptHandler<S> {
    #[inline]
    #[hal_macro_helper]
    unsafe fn on_interrupt() {
        unsafe {
            let mut peripherals = crate::pac::Peripherals::steal();

            #[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
            let uart = S::reg_block(&mut peripherals).usart();
            #[hal_cfg("sercom0-d5x")]
            let uart = S::reg_block(&mut peripherals).usart_int();

            let flags_pending = Flags::from_bits_retain(uart.intflag().read().bits());
            let enabled_flags = Flags::from_bits_retain(uart.intenset().read().bits());
            uart.intenclr().write(|w| w.bits(flags_pending.bits()));

            // Disable interrupts, but don't clear the flags. The future will take care of
            // clearing flags and re-enabling interrupts when woken.
            if (Flags::RX & enabled_flags).intersects(flags_pending) {
                S::rx_waker().wake();
            }

            if (Flags::TX & enabled_flags).intersects(flags_pending) {
                S::tx_waker().wake();
            }
        }
    }
}

impl<C, D, S> Uart<C, D, NoneT, NoneT>
where
    C: ValidConfig<Sercom = S>,
    D: SingleOwner,
    S: Sercom,
{
    /// Turn a [`Uart`] into a [`UartFuture`]. This method is only available for
    /// [`Uart`]s which have a [`Tx`],
    /// [`Rx`] or [`Duplex`] [`Capability`].
    #[inline]
    pub fn into_future<I>(self, _interrupts: I) -> UartFuture<C, D>
    where
        I: Binding<S::Interrupt, InterruptHandler<S>>,
    {
        S::Interrupt::unpend();
        unsafe { S::Interrupt::enable() };

        UartFuture { uart: self }
    }
}

/// `async` version of a [`Uart`].
///
/// Create this struct by calling [`Uart::into_future`](Uart::into_future).
pub struct UartFuture<C, D, R = NoneT, T = NoneT>
where
    C: ValidConfig,
    D: Capability,
{
    uart: Uart<C, D, R, T>,
}

/// Convenience type for a [`UartFuture`] with RX and TX capabilities
pub type UartFutureDuplex<C> = UartFuture<C, Duplex>;

/// Convenience type for a RX-only [`UartFuture`].
pub type UartFutureRx<C> = UartFuture<C, Rx>;

/// Convenience type for the RX half of a [`Duplex`] [`UartFuture`].
pub type UartFutureRxDuplex<C> = UartFuture<C, RxDuplex>;

/// Convenience type for a TX-only [`UartFuture`].
pub type UartFutureTx<C> = UartFuture<C, Tx>;

/// Convenience type for the TX half of a [`Duplex`] [`UartFuture`].
pub type UartFutureTxDuplex<C> = UartFuture<C, TxDuplex>;

impl<C, R, T> UartFuture<C, Duplex, R, T>
where
    C: ValidConfig,
{
    /// Split the [`UartFuture`] into [`RxDuplex`]and [`TxDuplex`] halves.
    #[inline]
    #[allow(clippy::type_complexity)]
    pub fn split(
        self,
    ) -> (
        UartFuture<C, RxDuplex, R, NoneT>,
        UartFuture<C, TxDuplex, NoneT, T>,
    ) {
        let config = unsafe { core::ptr::read(&self.uart.config) };
        (
            UartFuture {
                uart: Uart {
                    config: self.uart.config,
                    capability: PhantomData,
                    rx_channel: self.uart.rx_channel,
                    tx_channel: NoneT,
                },
            },
            UartFuture {
                uart: Uart {
                    config,
                    capability: PhantomData,
                    rx_channel: NoneT,
                    tx_channel: self.uart.tx_channel,
                },
            },
        )
    }

    /// Join [`RxDuplex`] and [`TxDuplex`] halves back into a full
    /// `UartFuture<C, Duplex>`
    #[inline]
    pub fn join(
        rx: UartFuture<C, RxDuplex, R, NoneT>,
        tx: UartFuture<C, TxDuplex, NoneT, T>,
    ) -> Self {
        Self {
            uart: Uart {
                config: rx.uart.config,
                capability: PhantomData,
                rx_channel: rx.uart.rx_channel,
                tx_channel: tx.uart.tx_channel,
            },
        }
    }
}

impl<C, D> UartFuture<C, D, NoneT, NoneT>
where
    C: ValidConfig,
    D: SingleOwner,
{
    /// Return the underlying [`Uart`].
    pub fn free(self) -> Uart<C, D> {
        self.uart
    }
}

impl<C, D, R, T> embedded_io::ErrorType for UartFuture<C, D, R, T>
where
    C: ValidConfig,
    D: Capability,
{
    type Error = Error;
}

impl<C, D, S, R, T> UartFuture<C, D, R, T>
where
    C: ValidConfig<Sercom = S>,
    D: Capability,
    S: Sercom,
{
    #[inline]
    async fn wait_flags(&mut self, flags_to_wait: Flags) {
        let flags_to_wait = flags_to_wait & Flags::from_bits_retain(D::FLAG_MASK);

        core::future::poll_fn(|cx| {
            // Scope maybe_pending so we don't forget to re-poll the register later down.
            {
                let maybe_pending = self.uart.config.as_ref().registers.read_flags();
                if flags_to_wait.intersects(maybe_pending) {
                    return Poll::Ready(());
                }
            }

            if flags_to_wait.intersects(Flags::RX) {
                self.uart.disable_interrupts(Flags::RX);
                S::rx_waker().register(cx.waker());
            }
            if flags_to_wait.intersects(Flags::TX) {
                self.uart.disable_interrupts(Flags::RX);
                S::tx_waker().register(cx.waker());
            }
            self.uart.enable_interrupts(flags_to_wait);
            let maybe_pending = self.uart.config.as_ref().registers.read_flags();

            if !flags_to_wait.intersects(maybe_pending) {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
    }
}

impl<C, D, S, R, T> UartFuture<C, D, R, T>
where
    C: ValidConfig<Sercom = S>,
    D: Receive,
    S: Sercom,
    DataReg: AsPrimitive<C::Word>,
{
    /// Use a DMA channel for receiving words on the RX line
    #[cfg(feature = "dma")]
    #[inline]
    pub fn with_rx_dma_channel<Chan: crate::dmac::AnyChannel<Status = crate::dmac::ReadyFuture>>(
        self,
        rx_channel: Chan,
    ) -> UartFuture<C, D, Chan, T> {
        UartFuture {
            uart: Uart {
                config: self.uart.config,
                capability: PhantomData,
                rx_channel,
                tx_channel: self.uart.tx_channel,
            },
        }
    }

    /// Read a single [`Word`](crate::sercom::uart::Word) from the UART.
    #[inline]
    pub async fn read_word(&mut self) -> Result<C::Word, Error> {
        self.wait_flags(Flags::RXC).await;
        self.uart.read_status().check_bus_error()?;
        Ok(unsafe { self.uart.read_data().as_() })
    }
}

impl<C, D, S, T> UartFuture<C, D, NoneT, T>
where
    C: ValidConfig<Sercom = S>,
    D: Receive,
    S: Sercom,
    DataReg: AsPrimitive<C::Word>,
{
    /// Read the specified number of [`Word`](crate::sercom::uart::Word)s into a
    /// buffer, word by word.
    ///
    /// In case of an error, returns `Err(Error, usize)` where the `usize`
    /// represents the number of valid words read before the error occured.
    #[inline]
    pub async fn read(&mut self, buffer: &mut [C::Word]) -> Result<(), (Error, usize)> {
        for (i, word) in buffer.iter_mut().enumerate() {
            match self.read_word().await {
                Ok(w) => {
                    *word = w;
                }
                Err(e) => {
                    return Err((e, i));
                }
            }
        }
        Ok(())
    }
}

impl<C, D, S, R> embedded_io_async::Read for UartFuture<C, D, NoneT, R>
where
    C: ValidConfig<Sercom = S, Word = u8>,
    D: Receive,
    S: Sercom,
{
    #[inline]
    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        self.read(buffer).await.map_err(|(e, _)| e)?;
        Ok(buffer.len())
    }
}

impl<C, D, S, R, T> UartFuture<C, D, R, T>
where
    C: ValidConfig<Sercom = S>,
    D: Transmit,
    S: Sercom,
{
    /// Use a DMA channel for sending words on the TX line
    #[cfg(feature = "dma")]
    #[inline]
    pub fn with_tx_dma_channel<Chan: crate::dmac::AnyChannel<Status = crate::dmac::ReadyFuture>>(
        self,
        tx_channel: Chan,
    ) -> UartFuture<C, D, R, Chan> {
        UartFuture {
            uart: Uart {
                config: self.uart.config,
                capability: PhantomData,
                rx_channel: self.uart.rx_channel,
                tx_channel,
            },
        }
    }

    /// Write a single [`Word`](crate::sercom::uart::Word) to the UART.
    #[inline]
    pub async fn write_word(&mut self, word: C::Word) {
        self.wait_flags(Flags::DRE).await;
        unsafe { self.uart.write_data(word.as_()) };
    }
}

impl<C, D, S, R> UartFuture<C, D, R, NoneT>
where
    C: ValidConfig<Sercom = S>,
    D: Transmit,
    S: Sercom,
{
    /// Write the specified number of [`Word`](crate::sercom::uart::Word)s from
    /// a buffer to the UART, word by word.
    #[inline]
    pub async fn write(&mut self, buffer: &[C::Word]) {
        for word in buffer {
            self.write_word(*word).await;
        }
    }
}

impl<C, D, S, R> embedded_io_async::Write for UartFuture<C, D, R, NoneT>
where
    C: ValidConfig<Sercom = S, Word = u8>,
    D: Transmit,
    S: Sercom,
{
    #[inline]
    async fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error> {
        self.write(buffer).await;
        Ok(buffer.len())
    }
}

impl<C, D, R, T, S> AsRef<Uart<C, D, R, T>> for UartFuture<C, D, R, T>
where
    C: ValidConfig<Sercom = S>,
    D: Capability,
    S: Sercom,
{
    fn as_ref(&self) -> &Uart<C, D, R, T> {
        &self.uart
    }
}

impl<C, D, R, T, S> AsMut<Uart<C, D, R, T>> for UartFuture<C, D, R, T>
where
    C: ValidConfig<Sercom = S>,
    D: Capability,
    S: Sercom,
{
    fn as_mut(&mut self) -> &mut Uart<C, D, R, T> {
        &mut self.uart
    }
}

#[cfg(feature = "dma")]
mod dma {
    use super::*;
    use crate::{
        dmac::{AnyChannel, Beat, Channel, ReadyFuture},
        sercom::dma::{
            async_dma::{read_dma, write_dma},
            SharedSliceBuffer,
        },
    };

    /// Convenience type for a [`UartFuture`] with RX and TX capabilities in DMA
    /// mode.
    ///
    /// The type parameter `R` represents the RX DMA channel ID (`ChX`), and
    /// `T` represents the TX DMA channel ID.
    pub type UartFutureDuplexDma<C, R, T> =
        UartFuture<C, Duplex, Channel<R, ReadyFuture>, Channel<T, ReadyFuture>>;

    /// Convenience type for a RX-only [`UartFuture`] in DMA mode.
    ///
    /// The type parameter `R` represents the RX DMA channel ID (`ChX`).
    pub type UartFutureRxDma<C, R> = UartFuture<C, Rx, Channel<R, ReadyFuture>, NoneT>;

    /// Convenience type for the RX half of a [`Duplex`] [`UartFuture`] in DMA
    /// mode.
    ///
    /// The type parameter `R` represents the RX DMA channel ID (`ChX`).
    pub type UartFutureRxDuplexDma<C, R> = UartFuture<C, RxDuplex, Channel<R, ReadyFuture>, NoneT>;

    /// Convenience type for a TX-only [`UartFuture`] in DMA mode.
    ///
    /// The type parameter `T` represents the TX DMA channel ID (`ChX`).
    pub type UartFutureTxDma<C, T> = UartFuture<C, Tx, NoneT, Channel<T, ReadyFuture>>;

    /// Convenience type for the TX half of a [`Duplex`] [`UartFuture`] in DMA
    /// mode.
    ///
    /// The type parameter `T` represents the TX DMA channel ID (`ChX`).
    pub type UartFutureTxDuplexDma<C, T> = UartFuture<C, TxDuplex, NoneT, Channel<T, ReadyFuture>>;

    impl<C, D, R, T> UartFuture<C, D, R, T>
    where
        C: ValidConfig,
        D: Capability,
        R: AnyChannel<Status = ReadyFuture>,
    {
        /// Reclaim the RX DMA channel. Subsequent RX operations will no longer
        /// use DMA.
        pub fn take_rx_channel(self) -> (UartFuture<C, D, NoneT, T>, R) {
            let (uart, channel) = self.uart.take_rx_channel();
            (UartFuture { uart }, channel)
        }
    }

    impl<C, D, R, T> UartFuture<C, D, R, T>
    where
        C: ValidConfig,
        D: Capability,
        T: AnyChannel<Status = ReadyFuture>,
    {
        /// Reclaim the TX DMA channel. Subsequent TX operations will no longer
        /// use DMA.
        pub fn take_tx_channel(self) -> (UartFuture<C, D, R, NoneT>, T) {
            let (uart, channel) = self.uart.take_tx_channel();
            (UartFuture { uart }, channel)
        }
    }

    impl<C, D, S, R, T> UartFuture<C, D, R, T>
    where
        C: ValidConfig<Sercom = S>,
        C::Word: Beat,
        D: Receive,
        S: Sercom + 'static,
        DataReg: AsPrimitive<C::Word>,
        R: AnyChannel<Status = ReadyFuture>,
    {
        /// Read the specified number of [`Word`](crate::sercom::uart::Word)s
        /// into a buffer using DMA.
        #[inline]
        pub async fn read(&mut self, mut words: &mut [C::Word]) -> Result<(), Error> {
            // SAFETY: Using SercomPtr is safe because we hold on
            // to &mut self as long as the transfer hasn't completed.
            let uart_ptr = self.uart.sercom_ptr();

            read_dma::<_, _, S>(&mut self.uart.rx_channel, uart_ptr, &mut words).await?;
            Ok(())
        }
    }

    impl<C, D, S, R, T> embedded_io_async::Read for UartFuture<C, D, R, T>
    where
        C: ValidConfig<Sercom = S, Word = u8>,
        D: Receive,
        S: Sercom + 'static,
        DataReg: AsPrimitive<C::Word>,
        R: AnyChannel<Status = ReadyFuture>,
    {
        #[inline]
        async fn read(&mut self, words: &mut [u8]) -> Result<usize, Error> {
            self.read(words).await?;
            Ok(words.len())
        }
    }

    impl<C, D, S, R, T> UartFuture<C, D, R, T>
    where
        C: ValidConfig<Sercom = S>,
        C::Word: Beat,
        D: Transmit,
        S: Sercom + 'static,
        T: AnyChannel<Status = ReadyFuture>,
    {
        /// Write words from a buffer asynchronously, using DMA.
        #[inline]
        pub async fn write(&mut self, words: &[C::Word]) -> Result<(), Error> {
            // SAFETY: Using SercomPtr is safe because we hold on
            // to &mut self as long as the transfer hasn't completed.
            let uart_ptr = self.uart.sercom_ptr();

            let mut words = SharedSliceBuffer::from_slice(words);
            write_dma::<_, _, S>(&mut self.uart.tx_channel, uart_ptr, &mut words).await?;
            self.wait_flags(Flags::TXC).await;
            Ok(())
        }
    }

    impl<C, D, S, R, T> embedded_io_async::Write for UartFuture<C, D, R, T>
    where
        C: ValidConfig<Sercom = S, Word = u8>,
        D: Transmit,
        S: Sercom + 'static,
        T: AnyChannel<Status = ReadyFuture>,
    {
        #[inline]
        async fn write(&mut self, words: &[u8]) -> Result<usize, Error> {
            self.write(words).await?;
            Ok(words.len())
        }
    }
}

#[cfg(feature = "dma")]
pub use dma::*;
