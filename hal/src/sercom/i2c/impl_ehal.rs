//! `embedded-hal` trait implementations for [`I2c`]s

use super::{config::AnyConfig, flags::Error, Host, I2c};
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

impl<C: AnyConfig<Mode = Host>> Write for I2c<C, Host> {
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.do_write(addr, bytes)?;
        self.cmd_stop();
        Ok(())
    }
}

impl<C: AnyConfig<Mode = Host>> Read for I2c<C, Host> {
    type Error = Error;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.do_read(addr, buffer)?;
        self.cmd_stop();
        Ok(())
    }
}

impl<C: AnyConfig<Mode = Host>> WriteRead for I2c<C, Host> {
    type Error = Error;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.do_write_read(addr, bytes, buffer)?;
        self.cmd_stop();
        Ok(())
    }
}
