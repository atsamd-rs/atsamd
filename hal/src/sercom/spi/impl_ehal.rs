use super::*;
use crate::ehal::spi::{self, ErrorType, SpiBus};
#[allow(unused_imports)]
use crate::ehal_02::{blocking, serial};
use num_traits::{AsPrimitive, PrimInt};

#[hal_module(
    any("sercom0-d11", "sercom0-d21") => "impl_ehal_thumbv6m.rs",
    "sercom0-d5x" => "impl_ehal_thumbv7em.rs"
)]
pub mod impl_ehal_02 {}

impl spi::Error for Error {
    fn kind(&self) -> crate::ehal::spi::ErrorKind {
        match self {
            Error::Overflow => crate::ehal::spi::ErrorKind::Overrun,
            Error::LengthError => crate::ehal::spi::ErrorKind::Other,
        }
    }
}

impl<P, M, C> ErrorType for Spi<Config<P, M, C>, Duplex>
where
    Config<P, M, C>: ValidConfig,
    P: ValidPads,
    M: MasterMode,
    C: Size,
{
    type Error = Error;
}

impl<P, M, C> Spi<Config<P, M, C>, Duplex>
where
    Config<P, M, C>: ValidConfig,
    P: ValidPads,
    M: MasterMode,
    C: Size + Copy + 'static,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    DataWidth: AsPrimitive<C::Word>,
{
    /// Read and write a single word to the bus simultaneously.
    fn transfer_word_in_place(&mut self, word: C::Word) -> Result<C::Word, Error> {
        self.block_on_flags(Flags::DRE)?;

        unsafe {
            self.write_data(word.as_());
        }

        self.block_on_flags(Flags::TXC | Flags::RXC)?;
        let word = unsafe { self.read_data().as_() };
        Ok(word)
    }

    /// Perform a transfer, word by word.
    ///
    /// No-op words will be written if `read` is longer than `write`. Extra
    /// words are ignored if `write` is longer than `read`.
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
}

impl<P, M, C> SpiBus<Word<C>> for Spi<Config<P, M, C>, Duplex>
where
    Config<P, M, C>: ValidConfig,
    P: ValidPads,
    M: MasterMode,
    C: Size + Copy + 'static,
    C::Word: PrimInt + AsPrimitive<DataWidth>,
    DataWidth: AsPrimitive<C::Word>,
{
    fn read(&mut self, words: &mut [Word<C>]) -> Result<(), Self::Error> {
        for word in words.iter_mut() {
            // Should replace todo with nop_word
            *word = self.transfer_word_in_place(self.config.nop_word.as_())?;
        }

        Ok(())
    }

    #[inline]
    fn write(&mut self, words: &[Word<C>]) -> Result<(), Self::Error> {
        // We are `Duplex`, so we must receive as many words as we send,
        // otherwise we could trigger an overflow
        for word in words {
            let _ = self.transfer_word_in_place(*word)?;
        }
        Ok(())
    }

    #[inline]
    fn transfer(&mut self, read: &mut [Word<C>], write: &[Word<C>]) -> Result<(), Self::Error> {
        self.transfer_word_by_word(read, write)
    }

    #[inline]
    fn transfer_in_place<'w>(&mut self, words: &mut [Word<C>]) -> Result<(), Self::Error> {
        // Can only ever do word-by-word to avoid DMA race conditions
        for word in words {
            let read = self.transfer_word_in_place(*word)?;
            *word = read;
        }

        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Error> {
        // Ignore buffer overflow errors
        let _ = self.block_on_flags(Flags::TXC);
        Ok(())
    }
}
