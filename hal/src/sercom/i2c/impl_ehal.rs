//! [`embedded-hal`] trait implementations for [`I2c`]s

use super::{config::AnyConfig, flags::Error, I2c};
use crate::ehal::i2c::{self, ErrorKind, ErrorType, NoAcknowledgeSource};

impl i2c::Error for Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Error::BusError => ErrorKind::Bus,
            Error::ArbitrationLost => ErrorKind::ArbitrationLoss,
            Error::LengthError => ErrorKind::Other,
            Error::Nack => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown),
            Error::Timeout => ErrorKind::Other,
        }
    }
}

impl<C: AnyConfig> ErrorType for I2c<C> {
    type Error = Error;
}

impl<C: AnyConfig> i2c::I2c for I2c<C> {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        /// Helper type for keeping track of the type of operation that was
        /// executed last
        #[derive(Clone, Copy)]
        enum Operation {
            Read,
            Write,
        }

        // Keep track of the last executed operation type. The method
        // specification demands, that no repeated start condition is sent
        // between adjacent operations of the same type.
        let mut last_op = None;
        for op in operations {
            match op {
                i2c::Operation::Read(buf) => {
                    if let Some(Operation::Read) = last_op {
                        self.continue_read(buf)?;
                    } else {
                        self.do_read(address, buf)?;
                        last_op = Some(Operation::Read);
                    }
                }
                i2c::Operation::Write(bytes) => {
                    if let Some(Operation::Write) = last_op {
                        self.continue_write(bytes)?;
                    } else {
                        self.do_write(address, bytes)?;
                        last_op = Some(Operation::Write);
                    }
                }
            }
        }

        self.cmd_stop();
        Ok(())
    }

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.do_write(address, bytes)?;
        self.cmd_stop();
        Ok(())
    }

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.do_read(address, buffer)?;
        self.cmd_stop();
        Ok(())
    }

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.do_write_read(address, bytes, buffer)?;
        self.cmd_stop();
        Ok(())
    }
}

impl<C: AnyConfig> crate::ehal_02::blocking::i2c::Write for I2c<C> {
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.do_write(addr, bytes)?;
        self.cmd_stop();
        Ok(())
    }
}

impl<C: AnyConfig> crate::ehal_02::blocking::i2c::Read for I2c<C> {
    type Error = Error;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.do_read(addr, buffer)?;
        self.cmd_stop();
        Ok(())
    }
}

impl<C: AnyConfig> crate::ehal_02::blocking::i2c::WriteRead for I2c<C> {
    type Error = Error;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.do_write_read(addr, bytes, buffer)?;
        self.cmd_stop();
        Ok(())
    }
}
