//! Capability
//!
//! Type-level constructs for specifiying the capabilities of a UART peripheral

use crate::typelevel::Sealed;
use bitflags::bitflags;
use core::convert::TryFrom;

/// Type-level enum representing the capabilities of a UART peripheral
pub trait Capability: Sealed {
    const FLAG_MASK: u8;
    const STATUS_MASK: u16;
}

/// Type-level enum representing a UART that can transmit
pub trait Transmit: Capability {}

/// Type-level enum representing a UART that can receive
pub trait Receive: Capability {}

/// Type-level enum representing a UART that has transmit or receive
/// capability, but not both
pub trait Simplex: Capability {}

/// Marker type representing a UART that has both transmit and receive
/// capability
pub enum Duplex {}

impl Sealed for Duplex {}

impl Capability for Duplex {
    // All flags are valid for a Duplex UART
    const FLAG_MASK: u8 =
        // DRE
        0x01 |
        // TXC
        0x02 |
        // RXC
        0x04 |
        // RXS
        0x08 |
        // CTSIC
        0x10 |
        // RXBRK
        0x20 |
        // ERROR
        0x80;

    // All status flags are valid for a Duplex UART
    const STATUS_MASK: u16 =
        // PERR
        0x01 |
        // FERR
        0x02 |
        // BUFOVF
        0x04 |
        // CTS
        0x08 |
        // ISF
        0x10 |
        // COLL
        0x20;
}

impl Receive for Duplex {}
impl Transmit for Duplex {}

/// Marker type representing a UART that can only receive
pub enum Rx {}

impl Sealed for Rx {}

impl Capability for Rx {
    // Available interrupt flags for a RX half-UART
    const FLAG_MASK: u8 =
        // RXC
        0x04 |
        // RXS
        0x08 |
        // RXBRK
        0x20 |
        // ERROR
        0x80;

    // Available status flags for a RX half-UART
    const STATUS_MASK: u16 =
        // PERR
        0x01 |
        // FERR
        0x02 |
        // BUFOVF
        0x04 |
        // ISF
        0x10 |
        // COLL
        0x20;
}

impl Receive for Rx {}
impl Simplex for Rx {}

/// Marker type representing a UART that can only transmit
pub enum Tx {}

impl Sealed for Tx {}

impl Capability for Tx {
    // Available interrupt flags for a TX half-UART
    const FLAG_MASK: u8 =
        // DRE
        0x01 |
        // TXC
        0x02 |
        // CTSIC
        0x10;

    // Available status flags for a TX half-UART
    const STATUS_MASK: u16 =
        // CTS
        0x08;
}

impl Transmit for Tx {}
impl Simplex for Tx {}

/// Marker type representing the Rx half of a  [`Duplex`] UART
pub enum RxDuplex {}

impl Sealed for RxDuplex {}

impl Capability for RxDuplex {
    // Available interrupt flags for a RX half-UART
    const FLAG_MASK: u8 =
        // RXC
        0x04 |
        // RXS
        0x08 |
        // RXBRK
        0x20 |
        // ERROR
        0x80;

    // Available status flags for a RX half-UART
    const STATUS_MASK: u16 =
        // PERR
        0x01 |
        // FERR
        0x02 |
        // BUFOVF
        0x04 |
        // ISF
        0x10 |
        // COLL
        0x20;
}

impl Receive for RxDuplex {}

/// Marker type representing a the Tx half of a UART
pub enum TxDuplex {}

impl Sealed for TxDuplex {}

impl Capability for TxDuplex {
    // Available interrupt flags for a TX half-UART
    const FLAG_MASK: u8 =
        // DRE
        0x01 |
        // TXC
        0x02 |
        // CTSIC
        0x10;

    // Available status flags for a TX half-UART
    const STATUS_MASK: u16 =
        // CTS
        0x08;
}

impl Transmit for TxDuplex {}

/// Type-level function mapping [`Capability`] to UART-pad-related types.
///
/// See the documentation on [type-level functions] for more details.
///
/// [type-level functions]: crate::typelevel#type-level-functions
pub trait GetCapability {
    type Capability: Capability;
}

bitflags! {
    /// Interrupt bit flags for UART Rx transactions
    ///
    /// The available interrupt flags are `DRE`, `TXC`, `RXC`, `RXS`, `CTSIC`, `RXBRK` and
    /// `ERROR`. The binary format of the underlying bits exactly matches the
    /// INTFLAG bits.
    pub struct Flags: u8 {
        const DRE = 0x01;
        const TXC = 0x02;
        const RXC = 0x04;
        const RXS = 0x08;
        const CTSIC = 0x10;
        const RXBRK = 0x20;
        const ERROR = 0x80;
    }
}

//=============================================================================
// Status
//=============================================================================

bitflags! {
    /// Status flags for UART Rx transactions
    ///
    /// The available status flags are `PERR`, `FERR`, `BUFOVF`,
    /// `CTS`, `ISF` and `COLL`.
    /// The binary format of the underlying bits exactly matches
    /// the STATUS bits.
    pub struct Status: u16 {
        const PERR = 0x0001;
        const FERR = 0x0002;
        const BUFOVF = 0x0004;
        const CTS = 0x0008;
        const ISF = 0x0010;
        const COLL = 0x0020;
    }
}

/// Error `enum` for UART RX transactions
#[derive(Debug)]
pub enum Error {
    /// Detected a parity error
    ParityError,
    /// Detected a frame error
    FrameError,
    /// Detected a buffer overflow
    Overflow,
    /// Detected an inconsistent sync field
    InconsistentSyncField,
    /// Detected a collision
    CollisionDetected,
}

impl TryFrom<Status> for () {
    type Error = Error;

    #[inline]
    fn try_from(errors: Status) -> Result<(), Error> {
        use Error::*;
        if errors.contains(Status::PERR) {
            Err(ParityError)
        } else if errors.contains(Status::FERR) {
            Err(FrameError)
        } else if errors.contains(Status::BUFOVF) {
            Err(Overflow)
        } else if errors.contains(Status::ISF) {
            Err(InconsistentSyncField)
        } else if errors.contains(Status::COLL) {
            Err(CollisionDetected)
        } else {
            Ok(())
        }
    }
}

impl From<Error> for Status {
    #[inline]
    fn from(err: Error) -> Self {
        use Error::*;
        match err {
            ParityError => Status::PERR,
            FrameError => Status::FERR,
            Overflow => Status::BUFOVF,
            InconsistentSyncField => Status::ISF,
            CollisionDetected => Status::COLL,
        }
    }
}
