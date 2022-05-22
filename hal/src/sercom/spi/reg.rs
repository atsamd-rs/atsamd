use core::convert::TryInto;

use bitflags::bitflags;
use embedded_hal::spi::{Phase, Polarity};

#[cfg(any(feature = "samd11", feature = "samd21"))]
use crate::pac::sercom0::SPI;
#[cfg(feature = "min-samd51g")]
use crate::pac::sercom0::SPIM;

#[cfg(any(feature = "samd11", feature = "samd21"))]
use crate::pac::sercom0::spi::ctrla::MODE_A;
#[cfg(feature = "min-samd51g")]
use crate::pac::sercom0::spim::ctrla::MODE_A;

use crate::sercom::Sercom;
use crate::time::Hertz;

//=============================================================================
// BitOrder
//=============================================================================

/// Define the bit order of transactions
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BitOrder {
    MsbFirst,
    LsbFirst,
}

//=============================================================================
// Flags
//=============================================================================

bitflags! {
    /// Interrupt bit flags for SPI transactions
    ///
    /// The available interrupt flags are `DRE`, `RXC`, `TXC`, `SSL` and
    /// `ERROR`. The binary format of the underlying bits exactly matches the
    /// `INTFLAG` register.
    pub struct Flags: u8 {
        const DRE = 0x01;
        const TXC = 0x02;
        const RXC = 0x04;
        const SSL = 0x08;
        const ERROR = 0x80;
    }
}

//=============================================================================
// Status
//=============================================================================

bitflags! {
    /// Status bit flags for SPI transactions
    ///
    /// The available status flags are `BUFOVF` and `LENERR`. The binary format
    /// of the underlying bits exactly matches the `STATUS` register.
    pub struct Status: u16 {
        const BUFOVF = 0x0004;
        const LENERR = 0x0800;
    }
}

impl Status {
    /// Check the [`Status`] flags for [`Error`] conditions
    pub fn check_errors(&self) -> Result<(), Error> {
        // Buffer overflow has priority
        if self.contains(Status::BUFOVF) {
            Err(Error::Overflow)
        } else if self.contains(Status::LENERR) {
            Err(Error::LengthError)
        } else {
            Ok(())
        }
    }
}

//=============================================================================
// Error
//=============================================================================

/// Error `enum` for SPI transactions
///
/// The SPI peripheral only has two error types, buffer overflow and transaction
/// length error.
#[derive(Debug)]
pub enum Error {
    Overflow,
    LengthError,
}

//=============================================================================
// DataWidth
//=============================================================================

/// Type alias for the width of the `DATA` register
#[cfg(any(feature = "samd11", feature = "samd21"))]
pub type DataWidth = u16;

/// Type alias for the width of the `DATA` register
#[cfg(feature = "min-samd51g")]
pub type DataWidth = u32;

//==============================================================================
// CtrlA
//==============================================================================

pub(super) struct CtrlA {
    pub bit_order: BitOrder,
    pub cpol: Polarity,
    pub cpha: Phase,
    pub dipo: u8,
    pub dopo: u8,
    pub ibon: bool,
    pub run_in_standby: bool,
}

impl CtrlA {
    pub fn default(dipo: u8, dopo: u8) -> Self {
        Self {
            bit_order: BitOrder::MsbFirst,
            cpol: Polarity::IdleLow,
            cpha: Phase::CaptureOnFirstTransition,
            dipo,
            dopo,
            ibon: false,
            run_in_standby: false,
        }
    }
}

//==============================================================================
// Registers
//==============================================================================

/// Define a task-focused register interface for SPI peripherals
///
/// This struct acts to define a task-focused, rather than register-focused, API
/// for a SERCOM configured as an SPI peripheral. It also serves to erase the
/// inherent interior mutability of the PAC [`Sercom`] type, which allows it to
/// implement [`Sync`].
pub(super) struct Registers<S: Sercom> {
    pub sercom: S,
}

// Safety: The [`Registers`] struct erases interior mutability, so this is now
// safe.
unsafe impl<S: Sercom> Sync for Registers<S> {}

impl<S: Sercom> Registers<S> {
    #[cfg(any(feature = "samd11", feature = "samd21"))]
    #[inline]
    pub fn spi(&self) -> &SPI {
        self.sercom.spi()
    }

    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn spi(&self) -> &SPIM {
        self.sercom.spim()
    }

    /// Reset the SERCOM peripheral
    #[inline]
    pub fn reset(&mut self) {
        self.spi().ctrla.write(|w| w.swrst().set_bit());
        while self.spi().syncbusy.read().swrst().bit_is_set() {}
    }

    #[cfg(feature = "dma")]
    /// Get a pointer to the `DATA` register
    pub fn data_ptr<Z: super::Size>(&self) -> *mut Z::Word {
        self.spi().data.as_ptr() as *mut _
    }

    /// Configure the SPI operating mode
    ///
    /// For maximum flexibility, this module chooses to always operate in 32-bit
    /// extension mode. The LENGTH counter is used to control the number of byes
    /// in each SPI transaction. Due to a hardware bug, ICSPACE must be at least
    /// one. See the silicon errata for more details.
    #[inline]
    pub fn set_op_mode(&mut self, mode: MODE_A, mssen: bool) {
        self.spi().ctrla.modify(|_, w| w.mode().variant(mode));
        self.spi().ctrlb.modify(|_, w| w.mssen().bit(mssen));
        #[cfg(feature = "min-samd51g")]
        self.spi().ctrlc.write(|w| unsafe {
            w.data32b().data_trans_32bit();
            w.icspace().bits(1)
        });
        while self.spi().syncbusy.read().ctrlb().bit_is_set() {}
    }

    /// Return the current transaction length
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn get_length(&self) -> u8 {
        self.spi().length.read().len().bits()
    }

    /// Set the transaction length
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn set_length(&mut self, length: u8) {
        let length = if length == 0 { 1 } else { length };
        self.spi().length.write(|w| unsafe {
            w.len().bits(length);
            w.lenen().set_bit()
        });
        while self.spi().syncbusy.read().length().bit_is_set() {}
    }

    /// Set the character size
    #[cfg(any(feature = "samd11", feature = "samd21"))]
    #[inline]
    pub fn set_char_size(&mut self, bits: u8) {
        self.spi()
            .ctrlb
            .modify(|_, w| unsafe { w.chsize().bits(bits) });
    }

    pub fn get_ctrla(&self) -> CtrlA {
        let raw = self.spi().ctrla.read();
        let bit_order = if raw.dord().bit_is_clear() {
            BitOrder::MsbFirst
        } else {
            BitOrder::LsbFirst
        };
        let cpol = if raw.cpol().bit_is_clear() {
            Polarity::IdleLow
        } else {
            Polarity::IdleHigh
        };
        let cpha = if raw.cpha().bit_is_clear() {
            Phase::CaptureOnFirstTransition
        } else {
            Phase::CaptureOnSecondTransition
        };
        let dipo = raw.dipo().bits();
        let dopo = raw.dopo().bits();
        let ibon = raw.ibon().bit();
        let run_in_standby = raw.runstdby().bit();
        CtrlA {
            bit_order,
            cpol,
            cpha,
            dipo,
            dopo,
            ibon,
            run_in_standby,
        }
    }

    pub fn set_ctrla(&mut self, ctrla: CtrlA) {
        self.spi().ctrla.modify(|_, w| {
            match ctrla.bit_order {
                BitOrder::LsbFirst => w.dord().clear_bit(),
                BitOrder::MsbFirst => w.dord().set_bit(),
            };
            match ctrla.cpol {
                Polarity::IdleLow => w.cpol().clear_bit(),
                Polarity::IdleHigh => w.cpol().set_bit(),
            };
            match ctrla.cpha {
                Phase::CaptureOnFirstTransition => w.cpha().clear_bit(),
                Phase::CaptureOnSecondTransition => w.cpha().set_bit(),
            };
            #[allow(unused_unsafe)]
            unsafe {
                w.dipo().bits(ctrla.dipo)
            };
            unsafe { w.dopo().bits(ctrla.dopo) };
            w.ibon().bit(ctrla.ibon);
            w.runstdby().bit(ctrla.run_in_standby)
        });
    }

    /// Get the baud rate
    #[inline]
    pub fn get_baud(&mut self, freq: Hertz) -> Hertz {
        let baud = self.spi().baud.read().baud().bits() as u32 + 1;
        Hertz(freq.0 / 2 / baud)
    }

    /// Set the baud rate
    #[inline]
    pub fn set_baud(&mut self, freq: Hertz, baud: impl Into<Hertz>) {
        let baud = baud.into().0;
        let baud = if baud == 0 { 1 } else { baud };
        let bits = (freq.0 / 2 / baud).saturating_sub(1);
        let bits = bits.try_into().unwrap_or(u8::MAX);
        self.spi()
            .baud
            .modify(|_, w| unsafe { w.baud().bits(bits) });
    }

    /// Enable interrupts for the specified flags
    #[inline]
    pub fn enable_interrupts(&mut self, flags: Flags) {
        self.spi()
            .intenset
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Disable interrupts for the specified flags
    #[inline]
    pub fn disable_interrupts(&mut self, flags: Flags) {
        self.spi()
            .intenclr
            .write(|w| unsafe { w.bits(flags.bits()) });
    }

    /// Enable the receiver
    #[inline]
    pub fn rx_enable(&mut self) {
        self.spi().ctrlb.modify(|_, w| w.rxen().set_bit());
        while self.spi().syncbusy.read().ctrlb().bit_is_set() {}
    }

    /// Enable the peripheral
    #[inline]
    pub fn enable(&mut self) {
        self.spi().ctrla.modify(|_, w| w.enable().set_bit());
        while self.spi().syncbusy.read().enable().bit_is_set() {}
    }

    /// Disable the peripheral
    #[inline]
    pub fn disable(&mut self) {
        self.spi().ctrla.modify(|_, w| w.enable().clear_bit());
        while self.spi().syncbusy.read().enable().bit_is_set() {}
    }

    /// Read from the `DATA` register
    #[inline]
    pub fn read_data(&mut self) -> DataWidth {
        self.spi().data.read().data().bits()
    }

    /// Write to the `DATA` register
    #[inline]
    pub fn write_data(&mut self, data: DataWidth) {
        // Safety: All bit patterns are memory safe
        self.spi().data.write(|w| unsafe { w.data().bits(data) })
    }

    /// Read the interrupt flags
    #[inline]
    pub fn read_flags(&self) -> Flags {
        let bits = self.spi().intflag.read().bits();
        Flags::from_bits_truncate(bits)
    }

    /// Clear interrupt flags
    #[inline]
    pub fn clear_flags(&mut self, flags: Flags) {
        unsafe { self.spi().intflag.write(|w| w.bits(flags.bits())) };
    }

    /// Read the error status flags
    #[inline]
    pub fn read_status(&self) -> Status {
        let bits = self.spi().status.read().bits();
        Status::from_bits_truncate(bits)
    }

    /// Clear error status flags
    #[inline]
    pub fn clear_status(&mut self, status: Status) {
        unsafe { self.spi().status.write(|w| w.bits(status.bits())) };
    }

    /// Try to read the interrupt flags, but first check the error status flags.
    #[inline]
    pub fn read_flags_errors(&self) -> Result<Flags, Error> {
        self.read_status().check_errors()?;
        Ok(self.read_flags())
    }
}
