//! Working with the SERCOM peripherals.
//!
//! The atsamd51 hardware has several SERCOM instances that can
//! be configured to perform a variety of serial communication
//! tasks.  This configuration is expressed through the use of
//! type states to make it difficult to misuse.
//! Each sercom instance is associated with a group of IO pins
//! referred to as a Pad.  When the pins are set to the appropriate
//! peripheral function mode they are routed to the sercom pad.

mod i2c;
mod pads;
mod spi;
mod uart;

pub use self::i2c::*;
pub use self::pads::*;
pub use self::spi::*;
pub use self::uart::*;
