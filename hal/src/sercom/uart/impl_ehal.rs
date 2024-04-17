//! `embedded-hal` trait implementations for [`Uart`]s

use super::{
    Capability, Config, DataReg, EightBit, Error, Error as UartError, Flags, Receive, Transmit,
    Uart, ValidConfig, ValidPads,
};
use crate::ehal_02::{
    blocking,
    serial::{Read, Write},
};
use nb::Error::WouldBlock;
use num_traits::AsPrimitive;

impl<C, D> Read<C::Word> for Uart<C, D>
where
    C: ValidConfig,
    D: Receive,
    DataReg: AsPrimitive<C::Word>,
{
    type Error = Error;

    /// Wait for an `RXC` flag, then read the word
    #[inline]
    fn read(&mut self) -> nb::Result<C::Word, Error> {
        <Self as embedded_hal_nb::serial::Read<C::Word>>::read(self)
    }
}

impl<C, D> Write<C::Word> for Uart<C, D>
where
    C: ValidConfig,
    D: Transmit,
{
    type Error = UartError;

    /// Wait for a `DRE` flag, then write a word
    #[inline]
    fn write(&mut self, word: C::Word) -> nb::Result<(), Self::Error> {
        <Self as embedded_hal_nb::serial::Write<C::Word>>::write(self, word)
    }

    /// Wait for a `TXC` flag
    #[inline]
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        <Self as embedded_hal_nb::serial::Write<C::Word>>::flush(self)
    }
}

impl<C, D> blocking::serial::write::Default<C::Word> for Uart<C, D>
where
    C: ValidConfig,
    D: Transmit,
    Uart<C, D>: Write<C::Word>,
{
}

impl embedded_io::Error for UartError {
    #[inline]
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl<C, D> embedded_io::ErrorType for Uart<C, D>
where
    C: ValidConfig,
    D: Capability,
{
    type Error = UartError;
}

impl<P, D> embedded_io::Write for Uart<Config<P, EightBit>, D>
where
    P: ValidPads,
    D: Transmit,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        for byte in buf {
            while !self.read_flags().contains(Flags::DRE) {
                core::hint::spin_loop();
            }

            unsafe { self.write_data(byte.as_()) };
        }

        Ok(buf.len())
    }

    /// Wait for a `TXC` flag
    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        while !self.read_flags().contains(Flags::TXC) {
            core::hint::spin_loop();
        }

        self.clear_flags(Flags::TXC);
        Ok(())
    }
}

impl<P, D> embedded_io::Read for Uart<Config<P, EightBit>, D>
where
    P: ValidPads,
    D: Receive,
{
    /// Wait for an `RXC` flag, then read the word
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }

        for byte in buf.iter_mut() {
            let flags = self.read_flags_errors()?;
            while !flags.contains(Flags::RXC) {
                core::hint::spin_loop();
            }
            *byte = unsafe { self.read_data().as_() };
        }

        Ok(buf.len())
    }
}

impl embedded_hal_nb::serial::Error for UartError {
    #[inline]
    fn kind(&self) -> embedded_hal_nb::serial::ErrorKind {
        use embedded_hal_nb::serial::ErrorKind;

        match self {
            Self::ParityError => ErrorKind::Parity,
            Self::FrameError => ErrorKind::FrameFormat,
            Self::Overflow => ErrorKind::Overrun,
            _ => ErrorKind::Other,
        }
    }
}

impl<C, D, W> embedded_hal_nb::serial::ErrorType for Uart<C, D>
where
    C: ValidConfig<Word = W>,
    W: Copy,
    D: Capability,
{
    type Error = UartError;
}

impl<C, D> embedded_hal_nb::serial::Read<C::Word> for Uart<C, D>
where
    C: ValidConfig,
    D: Receive,
    DataReg: AsPrimitive<C::Word>,
{
    #[inline]
    fn read(&mut self) -> nb::Result<C::Word, Self::Error> {
        // Wait for an `RXC` flag, then read the word
        let flags = self.read_flags_errors()?;
        if flags.contains(Flags::RXC) {
            unsafe { Ok(self.read_data().as_()) }
        } else {
            Err(WouldBlock)
        }
    }
}

impl<C, D> embedded_hal_nb::serial::Write<C::Word> for Uart<C, D>
where
    C: ValidConfig,
    D: Transmit,
{
    /// Wait for a `DRE` flag, then write a word
    #[inline]
    fn write(&mut self, word: C::Word) -> nb::Result<(), Self::Error> {
        if self.read_flags().contains(Flags::DRE) {
            unsafe { self.write_data(word.as_()) };
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }

    /// Wait for a `TXC` flag
    #[inline]
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if self.read_flags().contains(Flags::TXC) {
            self.clear_flags(Flags::TXC);
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}
