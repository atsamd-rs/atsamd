//! Flag definitions

#![allow(unused_braces)]

use bitflags::bitflags;
use core::convert::TryFrom;
use modular_bitfield::specifiers::{B1, B5};
use modular_bitfield::*;

bitflags! {
    /// Interrupt bitflags for I2C transactions
    ///
    /// The available interrupt flags are `MB`, `SB`, and `ERROR`. The binary format of the underlying bits exactly matches the
    /// INTFLAG bits.
    pub struct Flags: u8 {
        const MB = 0x01;
        const SB = 0x02;
        const ERROR = 0x80;
    }
}

#[derive(BitfieldSpecifier, PartialEq)]
pub enum BusState {
    Unknown = 0x00,
    Idle = 0x01,
    Owner = 0x02,
    Busy = 0x03,
}

#[bitfield]
#[repr(u16)]
pub struct Status {
    /// Status flags for I2C transactions
    ///
    /// The available status flags are `BUSERR`, `ARBLOST`, `RXNACK`,
    /// `BUSSTATE`, `LOWTOUT` and `CLKHOLD`, `MEXTTOUT`, `SEXTTOUT` and
    /// `LENERR`. The binary format of the underlying bits exactly matches
    /// the STATUS bits.
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

#[derive(Debug, Clone, Copy)]
pub enum Error {
    BusError,
    ArbitrationLost,
    LengthError,
    Nack,
    Timeout,
}

impl TryFrom<Status> for () {
    type Error = Error;

    #[inline]
    fn try_from(errors: Status) -> Result<(), Error> {
        if errors.buserr() {
            Err(Error::BusError)
        } else if errors.arblost() {
            Err(Error::ArbitrationLost)
        } else if errors.lenerr() {
            Err(Error::LengthError)
        } else if errors.rxnack() {
            Err(Error::Nack)
        } else {
            Ok(())
        }
    }
}

// impl From<Error> for Status {
//     #[inline]
//     fn from(err: Error) -> Self {
//         match err {
//             Error::BusError => Status::new().with_buserr(true),
//             Error::ArbitrationLost => Status::new().with_arblost(true),
//             Error::LengthError => Status::new().with_lenerr(true),
//             Error::Nack => Status::new().with_rxnack(true),
//         }
//     }
// }
