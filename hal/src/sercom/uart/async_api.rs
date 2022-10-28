use crate::{
    sercom::{
        uart::{
            Capability, DataReg, Duplex, Error, Flags, Receive, RxDuplex, SingleOwner, Transmit,
            TxDuplex, Uart, ValidConfig,
        },
        Sercom,
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
    #[inline]
    pub fn into_future<I, N>(self, irq: I) -> UartFuture<C, D, N>
    where
        I: NvicInterruptRegistration<N>,
        N: InterruptNumber,
    {
        let irq_number = irq.number();
        irq.occupy(S::on_interrupt_uart);
        unsafe { cortex_m::peripheral::NVIC::unmask(irq_number) };

        UartFuture {
            uart: self,
            irq_number,
            rx_channel: NoneT,
            tx_channel: NoneT,
        }
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
    irq_number: N,
    rx_channel: R,
    tx_channel: T,
}

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
                irq_number: self.irq_number,
                rx_channel: self.rx_channel,
                tx_channel: NoneT,
            },
            UartFuture {
                uart: Uart {
                    config,
                    capability: PhantomData,
                },
                irq_number: self.irq_number,
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
            irq_number: rx.irq_number,
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
            irq_number: self.irq_number,
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
            irq_number: self.irq_number,
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
        }
    }
}
