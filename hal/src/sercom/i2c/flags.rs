//! Flag definitions
#![allow(clippy::identity_op)]
#![allow(unused_braces)]

use bitflags::bitflags;
use modular_bitfield::specifiers::{B1, B5};
use modular_bitfield::*;

bitflags! {
    /// Interrupt bitflags for I2C transactions
    ///
    /// The available interrupt flags are `MB`, `SB`, and `ERROR`. The binary format of the underlying bits exactly matches the
    /// INTFLAG bits.
    pub struct Flags: u8 {
        /// Master on bus interrupt
        const MB = 0x01;
        /// Slave on bus interrupt
        const SB = 0x02;
        /// Error interrupt
        const ERROR = 0x80;
    }
}

/// Type representing the current bus state
#[derive(BitfieldSpecifier, PartialEq)]
pub enum BusState {
    Unknown = 0x00,
    Idle = 0x01,
    Owner = 0x02,
    Busy = 0x03,
}

/// Status flags for I2C transactions
///
/// The available status flags are `BUSERR`, `ARBLOST`, `RXNACK`,
/// `BUSSTATE`, `LOWTOUT` and `CLKHOLD`, `MEXTTOUT`, `SEXTTOUT` and
/// `LENERR`. The binary format of the underlying bits exactly matches
/// the STATUS bits.
#[bitfield]
#[repr(u16)]
pub struct Status {
    pub buserr: bool,
    pub arblost: bool,
    #[skip(setters)]
    pub rxnack: bool,
    #[skip]
    _reserved: B1,
    pub busstate: BusState,
    pub lowtout: bool,
    #[skip(setters)]
    pub clkhold: bool,
    pub mexttout: bool,
    pub sexttout: bool,
    pub lenerr: bool,
    #[skip]
    _reserved: B5,
}

impl Status {
    pub fn check_bus_error(self) -> Result<(), Error> {
        if self.buserr() {
            Err(Error::BusError)
        } else if self.arblost() {
            Err(Error::ArbitrationLost)
        } else if self.lenerr() {
            Err(Error::LengthError)
        } else if self.rxnack() {
            Err(Error::Nack)
        } else {
            Ok(())
        }
    }
}

/// Errors available for I2C transactions
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    BusError,
    ArbitrationLost,
    LengthError,
    Nack,
    Timeout,
}
