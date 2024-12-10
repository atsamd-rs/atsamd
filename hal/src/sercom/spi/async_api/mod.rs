use crate::{
    async_hal::interrupts::{Binding, Handler, InterruptSource},
    sercom::{
        spi::{
            Capability, Config, DataWidth, Duplex, Error, Flags, MasterMode, OpMode, Rx, Size, Spi,
            Tx, ValidConfig, ValidPads,
        },
        Sercom,
    },
    typelevel::NoneT,
};
use atsamd_hal_macros::hal_macro_helper;
use core::{marker::PhantomData, task::Poll};
use embedded_hal_async::spi::{ErrorType, SpiBus};
use num_traits::{AsPrimitive, PrimInt};

#[cfg(feature = "dma")]
mod dma;
#[cfg(feature = "dma")]
pub use dma::*;

use super::{Receive, Slave, Transmit};

/// Interrupt handler for async SPI operarions
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
            let spi = S::reg_block(&mut peripherals).spi();
            #[hal_cfg("sercom0-d5x")]
            let spi = S::reg_block(&mut peripherals).spim();

            let flags_pending = Flags::from_bits_truncate(spi.intflag().read().bits());
            let enabled_flags = Flags::from_bits_truncate(spi.intenset().read().bits());

            // Disable interrupts, but don't clear the flags. The future will take care of
            // clearing flags and re-enabling interrupts when woken.
            if (Flags::RX & enabled_flags).intersects(flags_pending) {
                spi.intenclr().write(|w| w.bits(flags_pending.bits()));
                S::rx_waker().wake();
            }

            if (Flags::TX & enabled_flags).intersects(flags_pending) {
                spi.intenclr().write(|w| w.bits(flags_pending.bits()));
                S::tx_waker().wake();
            }
        }
    }
}

impl<C, A, S> Spi<C, A>
where
    C: ValidConfig<Sercom = S>,
    A: Capability,
    S: Sercom,
{
    /// Turn an [`Spi`] into a [`SpiFuture`].
    ///
    /// In cases where the underlying [`Spi`] is [`Duplex`], reading words need
    /// to be accompanied with sending a no-op word. By default it is set to
    /// 0x00, but you can configure it by using the
    /// [`nop_word`](crate::sercom::spi::Config::nop_word) method.
    #[inline]
    pub fn into_future<I>(self, _interrupts: I) -> SpiFuture<C, A>
    where
        C::Word: Copy,
        u8: AsPrimitive<C::Word>,
        I: Binding<S::Interrupt, InterruptHandler<S>>,
    {
        S::Interrupt::unpend();
        unsafe { S::Interrupt::enable() };

        SpiFuture { spi: self }
    }
}

/// `async` version of [`Spi`].
///
/// Create this struct by calling [`Spi::into_future`](Spi::into_future).
pub struct SpiFuture<C, A, R = NoneT, T = NoneT>
where
    C: ValidConfig,
    A: Capability,
{
    spi: Spi<C, A, R, T>,
}

#[cfg(feature = "defmt")]
impl<C, A, R, T> defmt::Format for SpiFuture<C, A, R, T>
where
    C: ValidConfig,
    A: Capability,
{
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SpiFuture defmt shim\n");
    }
}

/// Convenience type for a [`SpiFuture`] with RX and TX capabilities
pub type SpiFutureDuplex<C> = SpiFuture<C, Duplex>;

/// Convenience type for a [`SpiFuture`] with RX capabilities
pub type SpiFutureRx<C> = SpiFuture<C, Rx>;

/// Convenience type for a [`SpiFuture`] with TX capabilities
pub type SpiFutureTx<C> = SpiFuture<C, Tx>;

impl<C, A, S, R, T> SpiFuture<C, A, R, T>
where
    C: ValidConfig<Sercom = S>,
    A: Capability,
    S: Sercom,
{
    #[inline]
    async fn wait_flags(&mut self, flags_to_wait: Flags) -> Result<(), Error> {
        let flags_to_wait = flags_to_wait | Flags::ERROR;

        core::future::poll_fn(|cx| {
            // Scope maybe_pending so we don't forget to re-poll the register later down.
            {
                let maybe_pending = self.spi.config.as_ref().regs.read_flags();
                if flags_to_wait.intersects(maybe_pending) {
                    return Poll::Ready(self.spi.check_and_clear_error(maybe_pending));
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
                Poll::Ready(self.spi.check_and_clear_error(maybe_pending))
            }
        })
        .await?;

        Ok(())
    }
}

impl<C, D> SpiFuture<C, D, NoneT, NoneT>
where
    C: ValidConfig,
    D: Capability,
{
    /// Return the underlying [`Spi`].
    pub fn free(self) -> Spi<C, D> {
        self.spi
    }
}

impl<P, M, C, D, R, S> SpiFuture<Config<P, M, C>, D, R, NoneT>
where
    Config<P, M, C>: ValidConfig<Sercom = S>,
    P: ValidPads,
    M: OpMode,
    C: Size + 'static,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    D: Transmit,
    DataWidth: AsPrimitive<C::Word>,
    S: Sercom,
{
    /// Write words from a buffer asynchronously, word by word
    #[inline]
    pub async fn write(&mut self, words: &[C::Word]) -> Result<(), Error> {
        if words.is_empty() {
            return Ok(());
        }
        // When in Duplex mode, read as many words as we write to avoid buffer overflows
        for word in words {
            let _ = self.transfer_word_in_place(*word).await?;
        }

        Ok(())
    }
}

impl<P, M, C, D, T, S> SpiFuture<Config<P, M, C>, D, NoneT, T>
where
    Config<P, M, C>: ValidConfig<Sercom = S>,
    P: ValidPads,
    M: MasterMode,
    C: Size + 'static,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    D: Receive,
    DataWidth: AsPrimitive<C::Word>,
    S: Sercom,
{
    /// Read words into a buffer asynchronously, word by word.
    ///
    /// Since we are using a [`Duplex`] [`SpiFuture`], we need to send a word
    /// simultaneously while receiving one. This `no-op` word is
    /// configurable via the [`nop_word`](Self::nop_word) method.
    #[inline]
    pub async fn read(&mut self, buffer: &mut [C::Word]) -> Result<(), Error> {
        if buffer.is_empty() {
            return Ok(());
        }

        let nop_word = self.spi.config.as_ref().nop_word.as_();
        for byte in buffer.iter_mut() {
            *byte = self.transfer_word_in_place(nop_word).await?;
        }

        Ok(())
    }
}

impl<P, M, C, S, D, R, T> SpiFuture<Config<P, M, C>, D, R, T>
where
    Config<P, M, C>: ValidConfig<Sercom = S>,
    P: ValidPads,
    M: OpMode,
    C: Size + 'static,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    DataWidth: AsPrimitive<C::Word>,
    D: Capability,
    S: Sercom,
{
    /// Read and write a single word to the bus simultaneously.
    pub async fn transfer_word_in_place(&mut self, to_send: C::Word) -> Result<C::Word, Error> {
        self.wait_flags(Flags::DRE).await?;
        unsafe {
            self.spi.write_data(to_send.as_());
        }

        self.flush_rx().await?;
        let word = unsafe { self.spi.read_data().as_() };

        Ok(word)
    }

    /// Perform a transfer, word by word.
    ///
    /// No-op words will be written if `read` is longer than `write`. Extra
    /// words are ignored if `write` is longer than `read`.
    async fn transfer_word_by_word(
        &mut self,
        read: &mut [C::Word],
        write: &[C::Word],
    ) -> Result<(), Error> {
        let nop_word = self.spi.config.as_ref().nop_word.as_();
        for (r, w) in read
            .iter_mut()
            .zip(write.iter().chain(core::iter::repeat(&nop_word)))
        {
            *r = self.transfer_word_in_place(*w).await?;
        }

        Ok(())
    }

    /// Wait on a TXC while ignoring buffer overflow errors.
    #[inline]
    async fn flush_tx(&mut self) {
        // Ignore buffer overflow errors
        let _ = self.wait_flags(Flags::TXC).await;
    }

    /// Wait on RXC flag
    #[inline]
    async fn flush_rx(&mut self) -> Result<(), Error> {
        self.wait_flags(Flags::RXC).await
    }
}

impl<C, A, R, T> AsRef<Spi<C, A, R, T>> for SpiFuture<C, A, R, T>
where
    C: ValidConfig,
    A: Capability,
{
    #[inline]
    fn as_ref(&self) -> &Spi<C, A, R, T> {
        &self.spi
    }
}

impl<C, A, R, T> AsMut<Spi<C, A, R, T>> for SpiFuture<C, A, R, T>
where
    C: ValidConfig,
    A: Capability,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Spi<C, A, R, T> {
        &mut self.spi
    }
}

impl<C, A, R, T> ErrorType for SpiFuture<C, A, R, T>
where
    C: ValidConfig,
    A: Capability,
{
    type Error = Error;
}

impl<C, A, R, T> embedded_io::ErrorType for SpiFuture<C, A, R, T>
where
    C: ValidConfig,
    A: Capability,
{
    type Error = Error;
}

impl<P, M, C, S> SpiBus<C::Word> for SpiFuture<Config<P, M, C>, Duplex>
where
    Config<P, M, C>: ValidConfig<Sercom = S>,
    P: ValidPads,
    M: MasterMode,
    C: Size + 'static,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    DataWidth: AsPrimitive<C::Word>,
    S: Sercom,
{
    async fn read(&mut self, words: &mut [C::Word]) -> Result<(), Self::Error> {
        self.read(words).await
    }

    async fn write(&mut self, words: &[C::Word]) -> Result<(), Self::Error> {
        self.write(words).await
    }

    async fn transfer(
        &mut self,
        read: &mut [C::Word],
        write: &[C::Word],
    ) -> Result<(), Self::Error> {
        self.transfer_word_by_word(read, write).await
    }

    async fn transfer_in_place(&mut self, words: &mut [C::Word]) -> Result<(), Self::Error> {
        // Can only ever do word-by-word to avoid DMA race conditions
        for word in words {
            let read = self.transfer_word_in_place(*word).await?;
            *word = read;
        }

        Ok(())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        // Wait for all transactions to complete, ignoring buffer overflow errors.
        self.flush_tx().await;
        Ok(())
    }
}

/// [`embedded_io::Write`] implementation for [`Transmit`] [`SpiFuture`]s in
/// either [`Slave`] or [`MasterMode`].
impl<P, M, Z, D, R, S> embedded_io_async::Write for SpiFuture<Config<P, M, Z>, D, R, NoneT>
where
    Config<P, M, Z>: ValidConfig<Sercom = S>,
    P: ValidPads,
    M: OpMode,
    Z: Size<Word = u8> + 'static,
    D: Transmit,
    S: Sercom,
{
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.write(buf).await?;
        Ok(buf.len())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush_tx().await;
        Ok(())
    }
}

/// [`embedded_io::Read`] implementation for [`Receive`] [`SpiFuture`]s in
/// [`MasterMode`].
impl<P, M, Z, D, T, S> embedded_io_async::Read for SpiFuture<Config<P, M, Z>, D, NoneT, T>
where
    P: ValidPads,
    M: MasterMode,
    Z: Size<Word = u8> + 'static,
    Config<P, M, Z>: ValidConfig<Sercom = S>,
    D: Receive,
    S: Sercom,
{
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.read(buf).await?;
        Ok(buf.len())
    }
}

/// [`embedded_io::Read`] implementation for [`Receive`] [`SpiFuture`]s in
/// [`Slave`] mode.
impl<P, Z, D, T, S> embedded_io_async::Read for SpiFuture<Config<P, Slave, Z>, D, NoneT, T>
where
    P: ValidPads,
    Z: Size<Word = u8> + 'static,
    Config<P, Slave, Z>: ValidConfig<Sercom = S>,
    D: Receive,
    S: Sercom,
{
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }

        for word in buf.iter_mut() {
            self.flush_rx().await?;
            *word = unsafe { self.spi.read_data().as_() };
        }

        Ok(buf.len())
    }
}
