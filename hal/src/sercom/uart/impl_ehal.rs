//! `embedded-hal` trait implementations for [`Uart`]s

use super::{
    Capability, Config, DataReg, EightBit, Error, Error as UartError, Flags, Receive, Transmit,
    Uart, ValidConfig, ValidPads,
};
use crate::{
    ehal_02::{
        blocking,
        serial::{Read, Write},
    },
    typelevel::NoneT,
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

impl<C, D, R, T> embedded_io::ErrorType for Uart<C, D, R, T>
where
    C: ValidConfig,
    D: Capability,
{
    type Error = UartError;
}

impl<P, D, R> embedded_io::Write for Uart<Config<P, EightBit>, D, R, NoneT>
where
    P: ValidPads,
    D: Transmit,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        for word in buf {
            nb::block!(<Self as embedded_hal_nb::serial::Write<u8>>::write(
                self, *word
            ))?;
        }

        Ok(buf.len())
    }

    /// Wait for a `TXC` flag
    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        nb::block!(<Self as embedded_hal_nb::serial::Write<u8>>::flush(self))?;
        Ok(())
    }
}

impl<P, D, T> embedded_io::Read for Uart<Config<P, EightBit>, D, NoneT, T>
where
    P: ValidPads,
    D: Receive,
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }

        for byte in buf.iter_mut() {
            let w = nb::block!(<Self as embedded_hal_nb::serial::Read<u8>>::read(self))?;
            *byte = w;
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

impl<C, D, R, T, W> embedded_hal_nb::serial::ErrorType for Uart<C, D, R, T>
where
    C: ValidConfig<Word = W>,
    W: Copy,
    D: Capability,
{
    type Error = UartError;
}

impl<C, D, R, T> embedded_hal_nb::serial::Read<C::Word> for Uart<C, D, R, T>
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

impl<C, D, R, T> embedded_hal_nb::serial::Write<C::Word> for Uart<C, D, R, T>
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

#[cfg(feature = "dma")]
mod dma {
    use super::*;
    use crate::{
        dmac::{AnyChannel, Beat, Ready},
        sercom::{
            dma::{read_dma, write_dma, SercomPtr, SharedSliceBuffer},
            Sercom,
        },
    };

    impl<C, D, R, T, W> Uart<C, D, R, T>
    where
        C: ValidConfig<Word = W>,
        D: Capability,
        W: Beat,
    {
        pub(in super::super) fn sercom_ptr(&self) -> SercomPtr<W> {
            SercomPtr(self.data_ptr())
        }
    }

    impl<P, D, R, T, S> embedded_io::Write for Uart<Config<P, EightBit>, D, R, T>
    where
        P: ValidPads<Sercom = S>,
        D: Transmit,
        T: AnyChannel<Status = Ready>,
        S: Sercom,
    {
        #[inline]
        fn write(&mut self, bytes: &[u8]) -> Result<usize, Self::Error> {
            let sercom_ptr = self.sercom_ptr();
            let channel = self.tx_channel.as_mut();
            let mut buffer = SharedSliceBuffer::from_slice(bytes);

            unsafe {
                write_dma::<_, _, S>(channel, sercom_ptr, &mut buffer);
            }

            while !channel.xfer_complete() {
                core::hint::spin_loop();
            }

            while !self.read_flags().contains(Flags::TXC) {
                core::hint::spin_loop();
            }

            Ok(bytes.len())
        }

        /// Wait for a `TXC` flag
        #[inline]
        fn flush(&mut self) -> Result<(), Self::Error> {
            nb::block!(<Self as embedded_hal_nb::serial::Write<u8>>::flush(self))?;
            Ok(())
        }
    }

    impl<P, D, R, T, S> embedded_io::Read for Uart<Config<P, EightBit>, D, R, T>
    where
        P: ValidPads<Sercom = S>,
        D: Receive,
        R: AnyChannel<Status = Ready>,
        S: Sercom,
    {
        #[inline]
        fn read(&mut self, mut buffer: &mut [u8]) -> Result<usize, Self::Error> {
            if buffer.is_empty() {
                return Ok(0);
            }

            let sercom_ptr = self.sercom_ptr();
            let channel = self.rx_channel.as_mut();

            unsafe {
                read_dma::<_, _, S>(channel, sercom_ptr, &mut buffer);
            }

            while !channel.xfer_complete() {
                core::hint::spin_loop();
            }

            while !self.read_flags().contains(Flags::RXC) {
                core::hint::spin_loop();
            }

            self.read_flags_errors()?;

            Ok(buffer.len())
        }
    }
}
