//! Use the SERCOM peripheral for I2C communications.

#[cfg(any(feature = "samd11", feature = "samd21"))]
#[path = "i2c/pads_thumbv6m.rs"]
mod pads;

#[cfg(feature = "min-samd51g")]
#[path = "i2c/pads_thumbv7em.rs"]
mod pads;

pub use pads::*;

mod reg;
use reg::Registers;

mod flags;
pub use flags::*;

mod config;
pub use config::*;

mod impl_ehal;

/// Word size for an I2C message
pub type Word = u8;

/// Inactive timeout configuration
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum InactiveTimeout {
    /// Disabled
    Disabled = 0x0,
    /// 5-6 SCL cycles (50-60 us @ 100 kHz)
    Us55 = 0x1,
    ///10-11 SCL cycles (100-110 us @ 100 kHz)
    Us105 = 0x2,
    /// 20-21 SCL cycles (200-210 us @ 100 kHz)
    Us205 = 0x3,
}

pub struct I2c<C: AnyConfig> {
    config: C,
}

impl<C: AnyConfig> I2c<C> {
    /// Obtain a pointer to the `DATA` register. Necessary for DMA transfers.
    #[inline]
    pub(crate) fn data_ptr(&self) -> *mut Word {
        self.config.as_ref().registers.data_ptr()
    }

    // Read the interrupt flags
    #[inline]
    pub fn read_flags(&self) -> Flags {
        self.config.as_ref().registers.read_flags()
    }

    /// Clear interrupt status flags
    #[inline]
    pub fn clear_flags(&mut self, flags: Flags) {
        self.config.as_mut().registers.clear_flags(flags);
    }

    /// Enable interrupts for the specified flags.
    #[inline]
    pub fn enable_interrupts(&mut self, flags: Flags) {
        self.config.as_mut().registers.enable_interrupts(flags);
    }

    /// Disable interrupts for the specified flags.
    #[inline]
    pub fn disable_interrupts(&mut self, flags: Flags) {
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
    /// use atsamd_hal::sercom::v2::i2c::{BaudMode, Oversampling, Uart};
    /// i2c.reconfigure(|c| c.set_run_in_standby(false));
    /// ```
    #[inline]
    pub fn reconfigure<F>(&mut self, update: F)
    where
        F: FnOnce(&mut SpecificConfig<C>),
    {
        self.config.as_mut().registers.enable_peripheral(false);
        update(&mut self.config.as_mut());
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
