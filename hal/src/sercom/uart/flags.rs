//! Flag definitions

use bitflags::bitflags;

//=============================================================================
// Interrupt flags
//=============================================================================
const DRE: u8 = 0x01;
const TXC: u8 = 0x02;
const RXC: u8 = 0x04;
const RXS: u8 = 0x08;
pub(super) const CTSIC: u8 = 0x10;
const RXBRK: u8 = 0x20;
const ERROR: u8 = 0x80;

/// Interrupt flags available for RX transactions
pub const RX_FLAG_MASK: u8 = RXC | RXS | RXBRK | ERROR;
/// Interrupt flags available for TX transactions
pub const TX_FLAG_MASK: u8 = DRE | TXC;
/// Interrupt flags available for Duplex transactions
pub const DUPLEX_FLAG_MASK: u8 = RX_FLAG_MASK | TX_FLAG_MASK;

bitflags! {
    /// Interrupt bit flags for UART Rx transactions
    ///
    /// The available interrupt flags are `DRE`, `TXC`, `RXC`, `RXS`, `CTSIC`, `RXBRK` and
    /// `ERROR`. The binary format of the underlying bits exactly matches the
    /// INTFLAG bits.
    #[derive(Clone, Copy)]
    pub struct Flags: u8 {
        const DRE = DRE;
        const TXC = TXC;
        const RXC = RXC;
        const RXS = RXS ;
        const CTSIC = CTSIC;
        const RXBRK = RXBRK;
        const ERROR = ERROR;
    }
}

impl Flags {
    /// [`Flags`] which can be used for receiving
    pub const RX: Self = Self::from_bits_retain(RX_FLAG_MASK);

    /// [`Flags`] which can be used for transmitting
    pub const TX: Self = Self::from_bits_retain(TX_FLAG_MASK);
}

//=============================================================================
// Status flags
//=============================================================================

const PERR: u16 = 0x01;
const FERR: u16 = 0x02;
const BUFOVF: u16 = 0x04;
const CTS: u16 = 0x08;
const ISF: u16 = 0x10;
const COLL: u16 = 0x20;

/// Status flags available for RX transactions
pub const RX_STATUS_MASK: u16 = PERR | FERR | BUFOVF | ISF | COLL;
/// Status flags available for Duplex transactions
pub const DUPLEX_STATUS_MASK: u16 = RX_STATUS_MASK;

bitflags! {
    /// Status flags for UART Rx transactions
    ///
    /// The available status flags are `PERR`, `FERR`, `BUFOVF`,
    /// `CTS`, `ISF` and `COLL`.
    /// The binary format of the underlying bits exactly matches
    /// the STATUS bits.
    pub struct Status: u16 {
        const PERR = PERR;
        const FERR = FERR;
        const BUFOVF = BUFOVF;
        const CTS = CTS;
        const ISF = ISF;
        const COLL = COLL;
    }
}

impl Status {
    /// Check whether [`Self`] originates from an error.
    ///
    /// # Errors
    ///
    /// Returns an error if `STATUS` contains:
    ///
    /// * `PERR` - Parity error
    /// * `FERR` - Frame error
    /// * `BUFOVF` - Buffer overflow
    /// * `ISF` - Inconsistent SYNC field
    /// * `COLL` - Collision
    #[inline]
    pub fn check_bus_error(self) -> Result<(), Error> {
        use Error::*;
        if self.contains(Status::PERR) {
            Err(ParityError)
        } else if self.contains(Status::FERR) {
            Err(FrameError)
        } else if self.contains(Status::BUFOVF) {
            Err(Overflow)
        } else if self.contains(Status::ISF) {
            Err(InconsistentSyncField)
        } else if self.contains(Status::COLL) {
            Err(CollisionDetected)
        } else {
            Ok(())
        }
    }
}

//=============================================================================
// Error
//=============================================================================

/// Errors available for UART transactions
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    /// DMA error
    #[cfg(feature = "dma")]
    Dma(crate::dmac::Error),
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
            #[cfg(feature = "dma")]
            Dma(_) => unimplemented!(),
        }
    }
}
