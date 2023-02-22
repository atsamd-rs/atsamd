use super::{ClientFlags, Status};
use crate::pac;
use crate::sercom::*;

/// Mirror of super::reg::Registers, except with client-specific
/// configuration
pub(super) struct Registers<S: Sercom> {
    pub sercom: S,
}

// SAFETY: It is safe to implement Sync for Registers, because it erases the
// interior mutability of the PAC SERCOM struct.
unsafe impl<S: Sercom> Sync for Registers<S> {}

impl<S: Sercom> Registers<S> {
    /// Create a new `Registers` instance
    #[inline]
    pub(super) fn new(sercom: S) -> Self {
        Self { sercom }
    }

    /// Helper function to access the underlying `I2CS` from the given
    /// `SERCOM`
    #[inline]
    fn i2c_slave(&self) -> &pac::sercom0::I2CS {
        self.sercom.i2cs()
    }

    /// Get a pointer to the `DATA` register
    pub(super) fn data_ptr<T>(&self) -> *mut T {
        self.i2c_slave().data.as_ptr() as *mut _
    }

    /// Free the `Registers` struct and return the underlying `Sercom`
    /// instance
    pub(super) fn free(self) -> S {
        self.sercom
    }

    /// Reset the SERCOM peripheral
    pub(super) fn swrst(&mut self) {
        self.i2c_slave().ctrla.write(|w| w.swrst().set_bit());
        while self.i2c_slave().syncbusy.read().swrst().bit_is_set() {}
    }

    /// Configure the SERCOM to use I2C slave mode
    pub(super) fn set_op_mode(&mut self) {
        let mode = pac::sercom0::i2cm::ctrla::MODE_A::I2C_SLAVE;
        self.i2c_master()
            .ctrla
            .modify(|_, w| w.mode().variant(mode));
    }

    /// Run in standby mode
    ///
    /// When set, the I2C peripheral will run in standby mode. See the
    /// datasheet for more details.
    #[inline]
    pub(super) fn set_run_in_standby(&mut self, set: bool) {
        self.i2c_slave().ctrla.modify(|_, w| w.runstdby().bit(set));
    }

    /// Get the current run in standby mode
    #[inline]
    pub(super) fn get_run_in_standby(&self) -> bool {
        self.i2c_slave().ctrla.read().runstdby().bit()
    }

    /// Set Smart Mode
    #[inline]
    pub(super) fn set_smart_mode(&mut self, set: bool) {
        self.i2c_slave().ctrlb.modify(|_, w| w.smen().bit(set));
    }

    /// Get the current Smart Mode setting
    #[inline]
    pub(super) fn get_smart_mode(&self) -> bool {
        self.i2c_slave().ctrlb.read().smen().bit()
    }

    /// Clear specified interrupt flags
    #[inline]
    pub(super) fn clear_flags(&mut self, flags: ClientFlags) {
        self.i2c_slave()
            .intflag
            .modify(|_, w| unsafe { w.bits(flags.bits()) });
    }

    /// Read interrupt flags
    #[inline]
    pub(super) fn read_flags(&self) -> ClientFlags {
        ClientFlags::from_bits_truncate(self.i2c_slave().intflag.read().bits())
    }

    /// Enable specified interrupts
    #[inline]
    pub(super) fn enable_interrupts(&mut self, flags: ClientFlags) {
        self.i2c_slave()
            .intenset
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Disable specified interrupts
    #[inline]
    pub(super) fn disable_interrupts(&mut self, flags: ClientFlags) {
        self.i2c_slave()
            .intenclr
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Clear specified status flags
    #[inline]
    pub(super) fn clear_status(&mut self, status: Status) {
        self.i2c_slave()
            .status
            .modify(|_, w| unsafe { w.bits(status.into()) });
    }

    /// Read status flags
    #[inline]
    pub(super) fn read_status(&self) -> Status {
        self.i2c_slave().status.read().bits().into()
    }
}
