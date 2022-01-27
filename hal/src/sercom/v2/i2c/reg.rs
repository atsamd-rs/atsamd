//! Register-level access to I2C configuration

use super::flags::{BusState, Error};
use super::InactiveTimeout;
use super::{Flags, Status};
use crate::pac;
use crate::sercom::v2::*;
use crate::time::Hertz;

const MASTER_ACT_READ: u8 = 2;
const MASTER_ACT_STOP: u8 = 3;

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

    /// Helper function to access the underlying `I2CM` from the given `SERCOM`
    #[inline]
    fn i2c_master(&self) -> &pac::sercom0::I2CM {
        self.sercom.i2cm()
    }

    /// Get a pointer to the `DATA` register
    pub(super) fn data_ptr<T>(&self) -> *mut T {
        self.i2c_master().data.as_ptr() as *mut _
    }

    /// Free the `Registers` struct and return the underlying `Sercom` instance
    #[inline]
    pub(super) fn free(self) -> S {
        self.sercom
    }

    /// Reset the SERCOM peripheral
    #[inline]
    pub(super) fn swrst(&mut self) {
        self.i2c_master().ctrla.write(|w| w.swrst().set_bit());
        while self.i2c_master().syncbusy.read().swrst().bit_is_set() {}
    }

    /// Configure the SERCOM to use I2C master mode
    #[inline]
    pub(super) fn set_op_mode(&mut self, mode: pac::sercom0::i2cm::ctrla::MODE_A) {
        self.i2c_master()
            .ctrla
            .modify(|_, w| w.mode().variant(mode));
    }

    /// Configure the baudrate for I2C master mode
    pub(super) fn set_baud(&mut self, clock_freq: Hertz, baud: impl Into<Hertz>) {
        // Since BAUDLOW is 0, the baud rate is used to generate both SCL high and SCL
        // low periods.
        let baud = (clock_freq.0 / (2 * baud.into().0) - 1) as u8;

        unsafe {
            self.i2c_master().baud.modify(|_, w| w.baud().bits(baud));
        }
    }

    /// Get the contents of the `BAUD` register.
    #[inline]
    pub(super) fn get_baud(&self) -> u32 {
        self.i2c_master().baud.read().bits()
    }

    /// Set SCL Low Time-Out
    ///
    /// If SCL is held low for 25ms-35ms, the master will release its clock
    /// hold, if enabled, and complete the current transaction. A stop condition
    /// will automatically be transmitted. INTFLAG.SB or INTFLAG.MB will be set
    /// as normal, but the clock hold will be released. The STATUS.LOWTOUT and
    /// STATUS.BUSERR status bits will be set.
    #[inline]
    pub(super) fn set_low_timeout(&mut self, set: bool) {
        self.i2c_master()
            .ctrla
            .modify(|_, w| w.lowtouten().bit(set));
    }

    /// Get SCL Low Time-Out
    ///
    /// If SCL is held low for 25ms-35ms, the master will release its clock
    /// hold, if enabled, and complete the current transaction. A stop condition
    /// will automatically be transmitted. INTFLAG.SB or INTFLAG.MB will be set
    /// as normal, but the clock hold will be released. The STATUS.LOWTOUT and
    /// STATUS.BUSERR status bits will be set.
    #[inline]
    pub(super) fn get_low_timeout(&mut self) -> bool {
        self.i2c_master().ctrla.read().lowtouten().bit()
    }

    /// Set the inactive timeout after which the bus state will be set to IDLE.
    /// Necessary for SMBus compatibility.
    #[inline]
    pub(super) fn set_inactive_timeout(&mut self, timeout: super::InactiveTimeout) {
        self.i2c_master()
            .ctrla
            .modify(|_, w| unsafe { w.inactout().bits(timeout as u8) })
    }

    /// Get the inactive timeout setting.
    #[inline]
    pub(super) fn get_inactive_timeout(&mut self) -> InactiveTimeout {
        let timeout = self.i2c_master().ctrla.read().inactout().bits();

        match timeout {
            0 => InactiveTimeout::Disabled,
            1 => InactiveTimeout::Us55,
            2 => InactiveTimeout::Us105,
            3 => InactiveTimeout::Us205,
            _ => unreachable!(),
        }
    }

    /// Run in standby mode
    ///
    /// When set, the I2C peripheral will run in standby mode. See the
    /// datasheet for more details.
    #[inline]
    pub(super) fn set_run_in_standby(&mut self, set: bool) {
        self.i2c_master().ctrla.modify(|_, w| w.runstdby().bit(set));
    }

    /// Get the current run in standby mode
    #[inline]
    pub(super) fn get_run_in_standby(&self) -> bool {
        self.i2c_master().ctrla.read().runstdby().bit()
    }

    /// Set Smart Mode
    #[inline]
    pub(super) fn set_smart_mode(&mut self, set: bool) {
        self.i2c_master().ctrlb.modify(|_, w| w.smen().bit(set));
    }

    /// Get the current Smart Mode setting
    #[inline]
    pub(super) fn get_smart_mode(&self) -> bool {
        self.i2c_master().ctrlb.read().smen().bit()
    }

    /// Clear specified interrupt flags
    #[inline]
    pub(super) fn clear_flags(&mut self, flags: Flags) {
        self.i2c_master()
            .intflag
            .modify(|_, w| unsafe { w.bits(flags.bits()) });
    }

    /// Read interrupt flags
    #[inline]
    pub(super) fn read_flags(&self) -> Flags {
        Flags::from_bits_truncate(self.i2c_master().intflag.read().bits())
    }

    /// Enable specified interrupts
    #[inline]
    pub(super) fn enable_interrupts(&mut self, flags: Flags) {
        self.i2c_master()
            .intenset
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Disable specified interrupts
    #[inline]
    pub(super) fn disable_interrupts(&mut self, flags: Flags) {
        self.i2c_master()
            .intenclr
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Clear specified status flags
    #[inline]
    pub(super) fn clear_status(&mut self, status: Status) {
        self.i2c_master()
            .status
            .modify(|_, w| unsafe { w.bits(status.into()) });
    }

    /// Read status flags
    #[inline]
    pub(super) fn read_status(&self) -> Status {
        self.i2c_master().status.read().bits().into()
    }

    pub(super) fn check_bus_status(&self) -> Result<(), Error> {
        let status = self.read_status();
        if status.busstate() == BusState::Busy
            || (status.arblost() && status.busstate() != BusState::Idle)
            || status.busstate() == BusState::Unknown
        {
            return Err(Error::BusError);
        } else {
            Ok(())
        }
    }

    /// Start a blocking write transaction
    #[inline]
    pub(super) fn start_write_blocking(&mut self, addr: u8) -> Result<(), Error> {
        if self.get_smart_mode() {
            self.disable();
            self.set_smart_mode(false);
            self.enable();
        }

        self.check_bus_status()?;

        // RESET the `ADDR` register, then signal start and transmit encoded
        // address for a write transaction.
        unsafe {
            self.i2c_master()
                .addr
                .write(|w| w.addr().bits(encode_write_address(addr)));
        }

        // wait for transmission to complete
        while !self.i2c_master().intflag.read().mb().bit_is_set() {}
        self.read_status().try_into()
    }

    /// Start a blocking read transaction
    #[inline]
    pub(super) fn start_read_blocking(&mut self, addr: u8) -> Result<(), Error> {
        if self.get_smart_mode() {
            self.disable();
            self.set_smart_mode(false);
            self.enable();
        }

        self.check_bus_status()?;

        self.i2c_master()
            .intflag
            .modify(|_, w| w.error().clear_bit());

        // RESET the `ADDR` register, then signal start (or repeated start if
        // appropriate) and transmit encoded address for a read transaction.
        unsafe {
            self.i2c_master()
                .addr
                .write(|w| w.addr().bits(encode_read_address(addr)));
        }

        // wait for transmission to complete
        loop {
            let intflag = self.i2c_master().intflag.read();
            // If arbitration was lost, it will be signalled via the mb bit
            if intflag.mb().bit_is_set() {
                return Err(Error::ArbitrationLost);
            }
            if intflag.sb().bit_is_set() || intflag.error().bit_is_set() {
                break;
            }
        }

        self.read_status().try_into()
    }

    /// Start DMA write:
    /// * Write `ADDR.LENEN` to 1
    /// * Write the transaction length in `ADDR.LEN`.
    /// * Write `ADDR.ADDR` to the encoded write address
    ///
    /// After ADDR.LEN bytes have been transmitted, a NACK (for master reads)
    /// and STOP are automatically generated.
    #[cfg(feature = "dma")]
    #[inline]
    pub(super) fn start_dma_write(&mut self, address: u8, xfer_len: u8) {
        if !self.get_smart_mode() {
            self.disable();
            self.set_smart_mode(true);
            self.enable();
        }

        self.i2c_master().addr.write(|w| unsafe {
            w.addr().bits(encode_write_address(address));
            w.len().bits(xfer_len);
            w.lenen().set_bit()
        });

        self.sync_sysop();
    }

    /// Start DMA read:
    /// * Write `ADDR.LENEN` to 1
    /// * Write the transaction length in `ADDR.LEN`.
    /// * Write `ADDR.ADDR` to the encoded write address
    ///
    /// After ADDR.LEN bytes have been received, a NACK (for master reads) and
    /// STOP are automatically generated.
    #[cfg(feature = "dma")]
    #[inline]
    pub(super) fn start_dma_read(&mut self, address: u8, xfer_len: u8) {
        if !self.get_smart_mode() {
            self.disable();
            self.set_smart_mode(true);
            self.enable();
        }

        self.i2c_master().addr.write(|w| unsafe {
            w.addr().bits(encode_read_address(address));
            w.len().bits(xfer_len);
            w.lenen().set_bit()
        });

        self.sync_sysop();
    }

    #[inline]
    pub(super) fn issue_command(&mut self, cmd: u8) {
        self.i2c_master()
            .ctrlb
            .modify(|_, w| unsafe { w.cmd().bits(cmd) });

        self.sync_sysop();
    }

    #[inline]
    pub(super) fn cmd_stop(&mut self) {
        self.issue_command(MASTER_ACT_STOP)
    }

    #[inline]
    pub(super) fn cmd_read(&mut self) {
        unsafe {
            self.i2c_master().ctrlb.modify(|_, w| {
                // clear bit means send ack
                w.ackact().clear_bit();
                w.cmd().bits(MASTER_ACT_READ)
            });
        }
        self.sync_sysop();
    }

    #[inline]
    pub(super) fn send_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        for b in bytes {
            unsafe {
                self.i2c_master().data.write(|w| w.bits(*b));
            }

            loop {
                let intflag = self.i2c_master().intflag.read();
                if intflag.mb().bit_is_set() || intflag.error().bit_is_set() {
                    break;
                }
            }
            self.read_status().try_into()?;
        }
        Ok(())
    }

    #[inline]
    pub(super) fn read_one(&mut self) -> u8 {
        while !self.i2c_master().intflag.read().sb().bit_is_set() {}
        self.i2c_master().data.read().bits()
    }

    #[inline]
    pub(super) fn fill_buffer(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        // Some manual iterator gumph because we need to ack bytes after the first.
        let mut iter = buffer.iter_mut();
        *iter.next().expect("buffer len is at least 1") = self.read_one();

        loop {
            match iter.next() {
                None => break,
                Some(dest) => {
                    // Ack the last byte so that we can receive another one
                    self.cmd_read();
                    *dest = self.read_one();
                }
            }
        }

        // arrange to send nack on next command to
        // stop slave from transmitting more data
        self.i2c_master().ctrlb.modify(|_, w| w.ackact().set_bit());

        Ok(())
    }

    #[inline]
    pub(super) fn do_write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
        self.start_write_blocking(addr)?;
        self.send_bytes(bytes)
    }

    #[inline]
    pub(super) fn do_read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
        self.start_read_blocking(addr)?;
        self.fill_buffer(buffer)
    }

    #[inline]
    pub(super) fn do_write_read(
        &mut self,
        addr: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Error> {
        self.start_write_blocking(addr)?;
        self.send_bytes(bytes)?;
        self.start_read_blocking(addr)?;
        self.fill_buffer(buffer)?;
        Ok(())
    }

    /// Set the bus to IDLE
    #[inline]
    pub(super) fn bus_idle(&mut self) {
        // Set the bus idle
        self.i2c_master()
            .status
            .modify(|_, w| unsafe { w.busstate().bits(BusState::Idle as u8) });
        // Wait for it to take effect
        self.sync_sysop();
    }

    #[inline]
    fn sync_sysop(&mut self) {
        while self.i2c_master().syncbusy.read().sysop().bit_is_set() {}
    }

    /// Enable the I2C peripheral
    ///
    /// I2C transactions are not possible until the peripheral is enabled.
    #[inline]
    pub(super) fn enable(&mut self) {
        // Globally enable peripheral
        self.enable_peripheral(true);
        // Set the bus state to IDLE
        self.bus_idle();
    }

    #[inline]
    pub(super) fn disable(&mut self) {
        self.enable_peripheral(false);
    }

    /// Enable or disable the SERCOM peripheral, and wait for the ENABLE bit to
    /// synchronize.
    #[inline]
    pub(super) fn enable_peripheral(&mut self, enable: bool) {
        self.i2c_master()
            .ctrla
            .modify(|_, w| w.enable().bit(enable));
        while self.i2c_master().syncbusy.read().enable().bit_is_set() {}
    }
}

fn encode_write_address(addr_7_bits: u8) -> u16 {
    (addr_7_bits as u16) << 1
}

fn encode_read_address(addr_7_bits: u8) -> u16 {
    (addr_7_bits as u16) << 1 | 1
}
