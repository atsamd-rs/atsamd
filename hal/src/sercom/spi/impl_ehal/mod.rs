use super::*;
use crate::ehal::spi::{self, SpiBus};
#[allow(unused_imports)]
use crate::ehal_02::{blocking, serial};
use crate::ehal_nb;
use num_traits::PrimInt;

#[cfg(feature = "dma")]
mod dma;
mod panic_on;

#[hal_module(
    any("sercom0-d11", "sercom0-d21") => "thumbv6m.rs",
    "sercom0-d5x" => "thumbv7em.rs")]
pub mod impl_ehal_02 {}

impl spi::Error for Error {
    #[allow(unreachable_patterns)]
    #[inline]
    fn kind(&self) -> crate::ehal::spi::ErrorKind {
        match self {
            Error::Overflow => crate::ehal::spi::ErrorKind::Overrun,
            Error::LengthError => crate::ehal::spi::ErrorKind::Other,
            // Pattern reachable when "dma" feature is enabled
            _ => crate::ehal::spi::ErrorKind::Other,
        }
    }
}

impl<C, D, R, T> ehal::spi::ErrorType for Spi<C, D, R, T>
where
    C: ValidConfig,
    D: Capability,
{
    type Error = Error;
}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl<C, D, R, T> embedded_io::ErrorType for Spi<C, D, R, T>
where
    C: ValidConfig,
    D: Capability,
{
    type Error = Error;
}

impl ehal_nb::serial::Error for Error {
    #[allow(unreachable_patterns)]
    fn kind(&self) -> ehal_nb::serial::ErrorKind {
        match self {
            Error::Overflow => ehal_nb::serial::ErrorKind::Overrun,
            Error::LengthError => ehal_nb::serial::ErrorKind::Other,
            // Pattern reachable when "dma" feature is enabled
            _ => ehal_nb::serial::ErrorKind::Other,
        }
    }
}

impl<C, D, R, T> ehal_nb::serial::ErrorType for Spi<C, D, R, T>
where
    C: ValidConfig,
    D: Capability,
{
    type Error = Error;
}

// Implementations for SPIs in either Master or Slave mode.
impl<P, M, C, D, R, T> Spi<Config<P, M, C>, D, R, T>
where
    Config<P, M, C>: ValidConfig,
    P: ValidPads,
    M: OpMode,
    C: Size + 'static,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    DataWidth: AsPrimitive<C::Word>,
    D: Capability,
{
    /// Read and write a single word to the bus simultaneously.
    #[inline]
    fn transfer_word_in_place(&mut self, word: C::Word) -> Result<C::Word, Error> {
        self.block_on_flags(Flags::DRE)?;

        unsafe {
            self.write_data(word.as_());
        }

        self.flush_rx()?;
        let word = unsafe { self.read_data().as_() };
        Ok(word)
    }

    /// Perform a transfer, word by word.
    ///
    /// No-op words will be written if `read` is longer than `write`. Extra
    /// words are ignored if `write` is longer than `read`.
    #[inline]
    fn transfer_word_by_word(
        &mut self,
        read: &mut [C::Word],
        write: &[C::Word],
    ) -> Result<(), Error> {
        let nop_word = self.config.nop_word;

        for (r, w) in read
            .iter_mut()
            .zip(write.iter().chain(core::iter::repeat(&nop_word.as_())))
        {
            *r = self.transfer_word_in_place(*w)?;
        }

        Ok(())
    }

    /// Wait on a TXC while ignoring buffer overflow errors.
    #[inline]
    fn flush_tx(&mut self) {
        // Ignore buffer overflow errors
        let _ = self.block_on_flags(Flags::TXC);
    }

    /// Wait on RXC flag
    #[inline]
    fn flush_rx(&mut self) -> Result<(), Error> {
        self.block_on_flags(Flags::RXC)
    }
}

// Implementations specific to Master mode SPIs.
impl<P, M, C, D> Spi<Config<P, M, C>, D>
where
    Config<P, M, C>: ValidConfig,
    P: ValidPads,
    M: MasterMode,
    C: Size + 'static,
    C::Word: PrimInt + AsPrimitive<DataWidth> + Copy,
    D: Capability,
    DataWidth: AsPrimitive<C::Word>,
{
    #[inline]
    fn read_word_by_word(&mut self, words: &mut [Word<C>]) -> Result<(), Error> {
        // Due to the nature of how SPI works, we must send a word in order to clock a
        // receive
        for word in words.iter_mut() {
            *word = self.transfer_word_in_place(self.config.nop_word.as_())?;
        }
        Ok(())
    }

    #[inline]
    fn write_word_by_word(&mut self, words: &[Word<C>]) -> Result<(), Error> {
        // Ignore RX buffer overflows by disabling the receiver
        self.config.as_mut().regs.rx_disable();

        for word in words {
            self.block_on_flags(Flags::DRE)?;
            unsafe {
                self.write_data(word.as_());
            }
        }

        // Reenable receiver only if necessary
        if D::RX_ENABLE {
            self.config.as_mut().regs.rx_enable();
        }
        Ok(())
    }
}

/// [`SpiBus`] implementation for [`Spi`], using word-by-word transfers.
impl<P, M, C> SpiBus<Word<C>> for Spi<Config<P, M, C>, Duplex>
where
    Config<P, M, C>: ValidConfig,
    P: ValidPads,
    M: MasterMode,
    C: Size + 'static,
    C::Word: PrimInt + AsPrimitive<DataWidth> + Copy,
    DataWidth: AsPrimitive<C::Word>,
{
    #[inline]
    fn read(&mut self, words: &mut [Word<C>]) -> Result<(), Self::Error> {
        self.read_word_by_word(words)
    }

    #[inline]
    fn write(&mut self, words: &[Word<C>]) -> Result<(), Self::Error> {
        self.write_word_by_word(words)
    }

    #[inline]
    fn transfer(&mut self, read: &mut [Word<C>], write: &[Word<C>]) -> Result<(), Self::Error> {
        self.transfer_word_by_word(read, write)
    }

    #[inline]
    fn transfer_in_place(&mut self, words: &mut [Word<C>]) -> Result<(), Self::Error> {
        for word in words {
            let read = self.transfer_word_in_place(*word)?;
            *word = read;
        }

        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Error> {
        self.flush_tx();
        Ok(())
    }
}
