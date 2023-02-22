//! WIP module for I2C client/slave configuration

mod config;
mod flags;
mod reg;

use super::Error;
use super::InactiveTimeout;
use super::PadSet;
use super::Status;
use super::Word;
pub use config::{ClientAnyConfig, ClientConfig, ClientSpecificConfig};
pub use flags::ClientFlags;
use reg::Registers;

/// Abstraction over an I2C peripheral in client mode
pub struct I2cClient<C: ClientAnyConfig> {
    config: C,
}

impl<C: ClientAnyConfig> I2cClient<C> {
    /// Obtain a pointer to the `DATA` register. Necessary for DMA transfers.
    #[inline]
    pub fn data_ptr(&self) -> *mut Word {
        self.config.as_ref().registers.data_ptr()
    }

    /// Read the interrupt flags
    #[inline]
    pub fn read_flags(&self) -> ClientFlags {
        self.config.as_ref().registers.read_flags()
    }

    /// Clear interrupt status flags
    #[inline]
    pub fn clear_flags(&mut self, flags: ClientFlags) {
        self.config.as_mut().registers.clear_flags(flags);
    }

    /// Enable interrupts for the specified flags.
    #[inline]
    pub fn enable_interrupts(&mut self, flags: ClientFlags) {
        self.config.as_mut().registers.enable_interrupts(flags);
    }

    /// Disable interrupts for the specified flags.
    #[inline]
    pub fn disable_interrupts(&mut self, flags: ClientFlags) {
        self.config.as_mut().registers.disable_interrupts(flags);
    }

    /// Read the status flags
    #[inline]
    pub fn read_status(&self) -> Status {
        self.config.as_ref().registers.read_status()
    }

    /// Clear the status flags
    #[inline]
    pub fn clear_status(&mut self, status: Status) {
        self.config.as_mut().registers.clear_status(status);
    }

    #[cfg(feature = "dma")]
    #[inline]
    pub(super) fn start_dma_write(&mut self, address: u8, xfer_len: u8) {
        self.config
            .as_mut()
            .registers
            .start_dma_write(address, xfer_len)
    }

    #[cfg(feature = "dma")]
    #[inline]
    pub(super) fn start_dma_read(&mut self, address: u8, xfer_len: u8) {
        self.config
            .as_mut()
            .registers
            .start_dma_read(address, xfer_len)
    }

    #[cfg(feature = "dma")]
    #[inline]
    pub(super) fn check_bus_status(&self) -> Result<(), Error> {
        self.config.as_ref().registers.check_bus_status()
    }

    #[inline]
    fn do_write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
        self.config.as_mut().registers.do_write(addr, bytes)
    }

    #[inline]
    fn do_read(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Error> {
        self.config.as_mut().registers.do_read(addr, bytes)
    }

    #[inline]
    fn do_write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
        self.config
            .as_mut()
            .registers
            .do_write_read(addr, bytes, buffer)
    }
    #[inline]
    fn cmd_stop(&mut self) {
        self.config.as_mut().registers.cmd_stop()
    }

    /// Reconfigure the I2C peripheral.
    ///
    /// Calling this method will temporarily disable the SERCOM peripheral, as
    /// some registers are enable-protected. This may interrupt any ongoing
    /// transactions.
    ///
    /// ```
    /// use atsamd_hal::sercom::i2c::I2c;
    /// i2c.reconfigure(|c| c.set_run_in_standby(false));
    /// ```
    #[inline]
    pub fn reconfigure<F>(&mut self, update: F)
    where
        F: FnOnce(&mut ClientSpecificConfig<C>),
    {
        self.config.as_mut().registers.enable_peripheral(false);
        update(self.config.as_mut());
        self.config.as_mut().registers.enable_peripheral(true);
    }

    /// Disable the I2C peripheral and return the underlying [`Config`]
    #[inline]
    pub fn disable(self) -> C {
        let mut config = self.config;
        config.as_mut().registers.disable();
        config
    }
}

impl<P: PadSet> AsRef<ClientConfig<P>> for I2cClient<ClientConfig<P>> {
    #[inline]
    fn as_ref(&self) -> &ClientConfig<P> {
        self.config.as_ref()
    }
}
