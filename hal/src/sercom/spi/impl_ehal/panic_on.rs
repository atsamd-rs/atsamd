//! [`SpiBus`] implementations for [`PanicOnWrite`] and [`PanicOnRead`]

use num_traits::{AsPrimitive, PrimInt};

use crate::ehal::spi::{ErrorType, SpiBus};

use super::{
    Config, DataWidth, MasterMode, PanicOnRead, PanicOnWrite, Rx, Size, Spi, Tx, ValidConfig,
    ValidPads, Word,
};

impl<T: ErrorType> ErrorType for PanicOnRead<T> {
    type Error = <T as ErrorType>::Error;
}

impl<T: ErrorType> ErrorType for PanicOnWrite<T> {
    type Error = <T as ErrorType>::Error;
}

/// [`SpiBus`] implementation for [`PanicOnRead`] using word-by-word transfers
impl<P, M, C> SpiBus<Word<C>> for PanicOnRead<Spi<Config<P, M, C>, Tx>>
where
    Config<P, M, C>: ValidConfig<OpMode = M>,
    P: ValidPads,
    M: MasterMode,
    C: Size + 'static,
    C::Word: PrimInt + AsPrimitive<DataWidth> + Copy,
    DataWidth: AsPrimitive<C::Word>,
{
    #[inline]
    fn read(&mut self, _words: &mut [Word<C>]) -> Result<(), Self::Error> {
        unimplemented!("`PanicOnRead` panics on SPI reads");
    }

    #[inline]
    fn write(&mut self, words: &[Word<C>]) -> Result<(), Self::Error> {
        self.0.write_word_by_word(words)
    }

    #[inline]
    fn transfer(&mut self, _read: &mut [Word<C>], _write: &[Word<C>]) -> Result<(), Self::Error> {
        unimplemented!("`PanicOnRead` panics on SPI reads");
    }

    #[inline]
    fn transfer_in_place(&mut self, _words: &mut [Word<C>]) -> Result<(), Self::Error> {
        unimplemented!("`PanicOnRead` panics on SPI reads");
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        self.0.flush_tx();
        Ok(())
    }
}

/// [`SpiBus`] implementation for [`PanicOnWrite`] using word-by-word transfers
impl<P, M, C> SpiBus<Word<C>> for PanicOnWrite<Spi<Config<P, M, C>, Rx>>
where
    Config<P, M, C>: ValidConfig<OpMode = M>,
    P: ValidPads,
    M: MasterMode,
    C: Size + 'static,
    C::Word: PrimInt + AsPrimitive<DataWidth> + Copy,
    DataWidth: AsPrimitive<C::Word>,
{
    #[inline]
    fn read(&mut self, words: &mut [Word<C>]) -> Result<(), Self::Error> {
        self.0.read_word_by_word(words)
    }

    #[inline]
    fn write(&mut self, _words: &[Word<C>]) -> Result<(), Self::Error> {
        unimplemented!("`PanicOnWrite` panics on SPI writes");
    }

    #[inline]
    fn transfer(&mut self, _read: &mut [Word<C>], _write: &[Word<C>]) -> Result<(), Self::Error> {
        unimplemented!("`PanicOnWrite` panics on SPI writes");
    }

    #[inline]
    fn transfer_in_place(&mut self, _words: &mut [Word<C>]) -> Result<(), Self::Error> {
        unimplemented!("`PanicOnWrite` panics on SPI writes");
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        unimplemented!("`PanicOnWrite` panics on SPI writes");
    }
}

#[cfg(feature = "dma")]
mod dma {
    use super::*;
    use crate::dmac::{AnyChannel, Beat, Ready};
    use crate::sercom::Sercom;
    use crate::typelevel::NoneT;

    /// [`SpiBus`] implementation for [`PanicOnRead`] using DMA transfers
    impl<P, M, C, T, S> SpiBus<Word<C>> for PanicOnRead<Spi<Config<P, M, C>, Tx, NoneT, T>>
    where
        Config<P, M, C>: ValidConfig<Sercom = S, OpMode = M>,
        P: ValidPads,
        M: MasterMode,
        C: Size + 'static,
        C::Word: PrimInt + AsPrimitive<DataWidth> + Beat,
        S: Sercom,
        DataWidth: AsPrimitive<C::Word>,
        T: AnyChannel<Status = Ready>,
    {
        #[inline]
        fn read(&mut self, _words: &mut [Word<C>]) -> Result<(), Self::Error> {
            unimplemented!("`PanicOnRead` panics on SPI reads");
        }

        #[inline]
        fn write(&mut self, words: &[Word<C>]) -> Result<(), Self::Error> {
            self.0.write_dma(words)?;
            Ok(())
        }

        #[inline]
        fn transfer(
            &mut self,
            _read: &mut [Word<C>],
            _write: &[Word<C>],
        ) -> Result<(), Self::Error> {
            unimplemented!("`PanicOnRead` panics on SPI reads");
        }

        #[inline]
        fn transfer_in_place(&mut self, _words: &mut [Word<C>]) -> Result<(), Self::Error> {
            unimplemented!("`PanicOnRead` panics on SPI reads");
        }

        #[inline]
        fn flush(&mut self) -> Result<(), Self::Error> {
            self.0.flush_tx();
            Ok(())
        }
    }

    /// [`SpiBus`] implementation for [`PanicOnWrite`] using DMA transfers
    impl<P, M, C, R, T> SpiBus<Word<C>> for PanicOnWrite<Spi<Config<P, M, C>, Rx, R, T>>
    where
        Config<P, M, C>: ValidConfig<OpMode = M>,
        P: ValidPads,
        M: MasterMode,
        C: Size + 'static,
        C::Word: PrimInt + AsPrimitive<DataWidth> + Beat,
        DataWidth: AsPrimitive<C::Word>,
        R: AnyChannel<Status = Ready>,
        T: AnyChannel<Status = Ready>,
    {
        #[inline]
        fn read(&mut self, words: &mut [Word<C>]) -> Result<(), Self::Error> {
            self.0.read_dma_master(words)
        }

        #[inline]
        fn write(&mut self, _words: &[Word<C>]) -> Result<(), Self::Error> {
            unimplemented!("`PanicOnWrite` panics on SPI writes");
        }

        #[inline]
        fn transfer(
            &mut self,
            _read: &mut [Word<C>],
            _write: &[Word<C>],
        ) -> Result<(), Self::Error> {
            unimplemented!("`PanicOnWrite` panics on SPI writes");
        }

        #[inline]
        fn transfer_in_place(&mut self, _words: &mut [Word<C>]) -> Result<(), Self::Error> {
            unimplemented!("`PanicOnWrite` panics on SPI writes");
        }

        #[inline]
        fn flush(&mut self) -> Result<(), Self::Error> {
            unimplemented!("`PanicOnWrite` panics on SPI writes");
        }
    }
}
