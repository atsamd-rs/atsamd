use crate::{
    pac::Interrupt,
    sercom::{
        uart::{
            Capability, DataReg, Duplex, Error, Flags, Receive, Rx, RxDuplex, SingleOwner,
            Transmit, Tx, TxDuplex, Uart, ValidConfig,
        },
        InterruptNumbers, Interrupts, Sercom,
    },
    typelevel::NoneT,
};
use core::{marker::PhantomData, task::Poll};
use cortex_m::interrupt::InterruptNumber;
use cortex_m_interrupt::NvicInterruptRegistration;
use num_traits::AsPrimitive;

impl<C, D, S> Uart<C, D>
where
    C: ValidConfig<Sercom = S>,
    D: SingleOwner,
    S: Sercom,
{
    /// Turn a [`Uart`] into a [`UartFuture`]. This method is only available for
    /// [`Uart`]s which have a [`Tx`](crate::sercom::uart::Tx),
    /// [`Rx`](crate::sercom::uart::Rx) or [`Duplex`] [`Capability`].
    #[cfg(feature = "thumbv6")]
    #[inline]
    pub fn into_future<N, I>(self, interrupts: Interrupts<N, I>) -> UartFuture<C, D, N>
    where
        I: NvicInterruptRegistration<N>,
        N: InterruptNumber,
    {
        let irq_numbers = interrupts.occupy(S::on_interrupt_uart);

        UartFuture {
            uart: self,
            _irqs: irq_numbers,
            rx_channel: NoneT,
            tx_channel: NoneT,
        }
    }

    /// Turn a [`Uart`] into a [`UartFuture`]. This method is only available for
    /// [`Uart`]s which have a [`Tx`](crate::sercom::uart::Tx),
    /// [`Rx`](crate::sercom::uart::Rx) or [`Duplex`] [`Capability`].
    #[cfg(feature = "thumbv7")]
    #[inline]
    pub fn into_future<N, N0, N1, N2, NOther>(
        self,
        interrupts: Interrupts<N, N0, N1, N2, NOther>,
    ) -> UartFuture<C, D, N>
    where
        N: InterruptNumber,
        N0: NvicInterruptRegistration<N>,
        N1: NvicInterruptRegistration<N>,
        N2: NvicInterruptRegistration<N>,
        NOther: NvicInterruptRegistration<N>,
    {
        let irq_numbers = interrupts.occupy(S::on_interrupt_uart);

        UartFuture {
            uart: self,
            _irqs: irq_numbers,
            rx_channel: NoneT,
            tx_channel: NoneT,
        }
    }
}

impl<C, D, N> UartFuture<C, D, N>
where
    C: ValidConfig,
    D: SingleOwner,
    N: InterruptNumber,
{
    /// Return the underlying [`Uart`].
    pub fn free(self) -> Uart<C, D> {
        self.uart
    }
}

/// `async` version of [`Uart`].
///
/// Create this struct by calling [`I2c::into_future`](I2c::into_future).
pub struct UartFuture<C, D, N, R = NoneT, T = NoneT>
where
    C: ValidConfig,
    D: Capability,
    N: InterruptNumber,
{
    uart: Uart<C, D>,
    _irqs: InterruptNumbers<N>,
    rx_channel: R,
    tx_channel: T,
}

/// Convenience type for a [`UartFuture`] with RX and TX capabilities
pub type UartFutureDuplex<C> = UartFuture<C, Duplex, Interrupt>;

/// Convenience type for a RX-only [`UartFuture`].
pub type UartFutureHalfRx<C> = UartFuture<C, Rx, Interrupt>;

/// Convenience type for the RX half of a [`Duplex`] [`UartFuture`].
pub type UartFutureRx<C> = UartFuture<C, RxDuplex, Interrupt>;

/// Convenience type for a TX-only [`UartFuture`].
pub type UartFutureTx<C> = UartFuture<C, Tx, Interrupt>;

/// Convenience type for the TX half of a [`Duplex`] [`UartFuture`].
pub type UartFutureTxDuplex<C> = UartFuture<C, TxDuplex, Interrupt>;

#[cfg(feature = "dma")]
/// Convenience type for a [`UartFuture`] with RX and TX capabilities in DMA
/// mode. The type parameter `R` represents the RX DMA channel ID (`ChX`), and
/// `T` represents the TX DMA channel ID.
pub type UartFutureDuplexDma<C, R, T> = UartFuture<
    C,
    Duplex,
    Interrupt,
    crate::dmac::Channel<R, crate::dmac::ReadyFuture>,
    crate::dmac::Channel<T, crate::dmac::ReadyFuture>,
>;

#[cfg(feature = "dma")]
/// Convenience type for a RX-only [`UartFuture`] in DMA mode.
/// The type parameter `R` represents the RX DMA channel ID (`ChX`).
pub type UartFutureRxDma<C, R> =
    UartFuture<C, Rx, Interrupt, crate::dmac::Channel<R, crate::dmac::ReadyFuture>, NoneT>;

#[cfg(feature = "dma")]
/// Convenience type for the RX half of a [`Duplex`] [`UartFuture`] in DMA mode.
/// The type parameter `R` represents the RX DMA channel ID (`ChX`).
pub type UartFutureRxDuplexDma<C, R> =
    UartFuture<C, RxDuplex, Interrupt, crate::dmac::Channel<R, crate::dmac::ReadyFuture>, NoneT>;

#[cfg(feature = "dma")]
/// Convenience type for a TX-only [`UartFuture`] in DMA mode.
/// The type parameter `T` represents the TX DMA channel ID (`ChX`).
pub type UartFutureTxDma<C, T> =
    UartFuture<C, Tx, Interrupt, NoneT, crate::dmac::Channel<T, crate::dmac::ReadyFuture>>;

#[cfg(feature = "dma")]
/// Convenience type for the TX half of a [`Duplex`] [`UartFuture`] in DMA mode.
/// The type parameter `T` represents the TX DMA channel ID (`ChX`).
pub type UartFutureTxDuplexDma<C, T> =
    UartFuture<C, TxDuplex, Interrupt, NoneT, crate::dmac::Channel<T, crate::dmac::ReadyFuture>>;

impl<C, N, R, T> UartFuture<C, Duplex, N, R, T>
where
    C: ValidConfig,
    N: InterruptNumber,
{
    /// Split the [`UartFuture`] into [`RxDuplex`]and [`TxDuplex`] halves.
    #[inline]
    #[allow(clippy::type_complexity)]
    pub fn split(
        self,
    ) -> (
        UartFuture<C, RxDuplex, N, R, NoneT>,
        UartFuture<C, TxDuplex, N, NoneT, T>,
    ) {
        let config = unsafe { core::ptr::read(&self.uart.config) };
        (
            UartFuture {
                uart: Uart {
                    config: self.uart.config,
                    capability: PhantomData,
                },
                _irqs: self._irqs.clone(),
                rx_channel: self.rx_channel,
                tx_channel: NoneT,
            },
            UartFuture {
                uart: Uart {
                    config,
                    capability: PhantomData,
                },
                _irqs: self._irqs,
                tx_channel: self.tx_channel,
                rx_channel: NoneT,
            },
        )
    }

    /// Join [`RxDuplex`] and [`TxDuplex`] halves back into a full
    /// `UartFuture<C, Duplex>`
    #[inline]
    pub fn join(
        rx: UartFuture<C, RxDuplex, N, R, NoneT>,
        tx: UartFuture<C, TxDuplex, N, NoneT, T>,
    ) -> Self {
        Self {
            uart: Uart {
                config: rx.uart.config,
                capability: PhantomData,
            },
            _irqs: rx._irqs,
            rx_channel: rx.rx_channel,
            tx_channel: tx.tx_channel,
        }
    }
}

impl<C, D, N, S, R, T> UartFuture<C, D, N, R, T>
where
    C: ValidConfig<Sercom = S>,
    D: Capability,
    N: InterruptNumber,
    S: Sercom,
{
    #[inline]
    async fn wait_flags(&mut self, flags_to_wait: Flags) {
        let flags_to_wait = flags_to_wait & unsafe { Flags::from_bits_unchecked(D::FLAG_MASK) };

        core::future::poll_fn(|cx| {
            // Scope maybe_pending so we don't forget to re-poll the register later down.
            {
                let maybe_pending = self.uart.as_ref().registers.read_flags();
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
            let maybe_pending = self.uart.as_ref().registers.read_flags();

            if !flags_to_wait.intersects(maybe_pending) {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
    }
}

impl<C, D, N, S, R, T> UartFuture<C, D, N, R, T>
where
    C: ValidConfig<Sercom = S>,
    D: Receive,
    N: InterruptNumber,
    S: Sercom,
    DataReg: AsPrimitive<C::Word>,
{
    /// Add a DMA channel for receiving words
    #[cfg(feature = "dma")]
    #[inline]
    pub fn with_rx_dma_channel<Chan: crate::dmac::AnyChannel<Status = crate::dmac::ReadyFuture>>(
        self,
        rx_channel: Chan,
    ) -> UartFuture<C, D, N, Chan, T> {
        UartFuture {
            uart: self.uart,
            _irqs: self._irqs,
            tx_channel: self.tx_channel,
            rx_channel,
        }
    }

    /// Read a single [`Word`](crate::sercom::uart::Word) from the UART.
    #[inline]
    pub async fn read_word(&mut self) -> Result<C::Word, Error> {
        self.wait_flags(Flags::RXC).await;
        self.uart.read_status().try_into()?;
        Ok(unsafe { self.uart.read_data().as_() })
    }
}

impl<C, D, N, S, T> UartFuture<C, D, N, NoneT, T>
where
    C: ValidConfig<Sercom = S>,
    D: Receive,
    N: InterruptNumber,
    S: Sercom,
    DataReg: AsPrimitive<C::Word>,
{
    /// Read the specified number of [`Word`](crate::sercom::uart::Word)s into a
    /// buffer, word by word. In case of an error, returns `Err(Error, usize)`
    /// where the `usize` represents the number of valid words read before
    /// the error occured.
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

impl<C, D, N, S, R, T> UartFuture<C, D, N, R, T>
where
    C: ValidConfig<Sercom = S>,
    D: Transmit,
    N: InterruptNumber,
    S: Sercom,
{
    /// Add a DMA channel for sending words
    #[cfg(feature = "dma")]
    #[inline]
    pub fn with_tx_dma_channel<Chan: crate::dmac::AnyChannel<Status = crate::dmac::ReadyFuture>>(
        self,
        tx_channel: Chan,
    ) -> UartFuture<C, D, N, R, Chan> {
        UartFuture {
            uart: self.uart,
            _irqs: self._irqs,
            rx_channel: self.rx_channel,
            tx_channel,
        }
    }

    /// Write a single [`Word`](crate::sercom::uart::Word) to the UART.
    #[inline]
    pub async fn write_word(&mut self, word: C::Word) {
        self.wait_flags(Flags::DRE).await;
        unsafe { self.uart.write_data(word.as_()) };
    }
}

impl<C, D, N, S, R> UartFuture<C, D, N, R, NoneT>
where
    C: ValidConfig<Sercom = S>,
    D: Transmit,
    N: InterruptNumber,
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

impl<C, D, N, R, T> AsRef<Uart<C, D>> for UartFuture<C, D, N, R, T>
where
    C: ValidConfig,
    D: Capability,
    N: InterruptNumber,
{
    fn as_ref(&self) -> &Uart<C, D> {
        &self.uart
    }
}

impl<C, D, N, R, T> AsMut<Uart<C, D>> for UartFuture<C, D, N, R, T>
where
    C: ValidConfig,
    D: Capability,
    N: InterruptNumber,
{
    fn as_mut(&mut self) -> &mut Uart<C, D> {
        &mut self.uart
    }
}

#[cfg(feature = "dma")]
mod dma {
    use super::*;
    use crate::{
        dmac::{AnyChannel, Beat, ReadyFuture},
        sercom::{
            async_dma::{read_dma, write_dma, SercomPtr},
            uart,
        },
    };

    impl<C, A, N, S, R, T> UartFuture<C, A, N, R, T>
    where
        C: ValidConfig<Sercom = S>,
        C::Word: Beat,
        A: Capability,
        N: InterruptNumber,
        S: Sercom + 'static,
    {
        fn sercom_ptr(&self) -> SercomPtr<C::Word> {
            SercomPtr(self.uart.data_ptr())
        }
    }

    impl<C, D, N, S, R, T> UartFuture<C, D, N, R, T>
    where
        C: ValidConfig<Sercom = S>,
        C::Word: Beat,
        D: Receive,
        N: InterruptNumber,
        S: Sercom + 'static,
        DataReg: AsPrimitive<C::Word>,
        R: AnyChannel<Status = ReadyFuture>,
    {
        /// Read the specified number of [`Word`](crate::sercom::uart::Word)s
        /// into a buffer using DMA.
        #[inline]
        pub async fn read(&mut self, words: &mut [C::Word]) -> Result<(), Error> {
            // SAFETY: Using SercomPtr is safe because we hold on
            // to &mut self as long as the transfer hasn't completed.
            let uart_ptr = self.sercom_ptr();

            read_dma::<_, S>(&mut self.rx_channel, uart_ptr, words)
                .await
                .map_err(uart::Error::Dma)?;

            Ok(())
        }
    }

    impl<C, D, N, S, R, T> UartFuture<C, D, N, R, T>
    where
        C: ValidConfig<Sercom = S>,
        C::Word: Beat,
        D: Transmit,
        N: InterruptNumber,
        S: Sercom + 'static,
        T: AnyChannel<Status = ReadyFuture>,
    {
        /// Write words from a buffer asynchronously, using DMA.
        #[inline]
        pub async fn write(&mut self, words: &[C::Word]) {
            // SAFETY: Using SercomPtr is safe because we hold on
            // to &mut self as long as the transfer hasn't completed.
            let uart_ptr = self.sercom_ptr();

            write_dma::<_, S>(&mut self.tx_channel, uart_ptr, words)
                .await
                .expect("DMA error");
            self.wait_flags(Flags::TXC).await;
        }
    }
}
