use bitflags::bitflags;
use modular_bitfield::specifiers::{B1, B5};
use modular_bitfield::*;

bitflags! {
    /// Interrupt bitflags for I2C client transactions
    ///
    /// The available interrupt flags are `PREC`, `AMATCH`, `DRDY`, and `ERROR`. The binary format of
    /// the underlying bits exactly matches the INTFLAG bits.
    pub struct ClientFlags: u8 {
        /// Stop received interrupt
        const PREC = 0x01;
        /// Address match interrupt
        const AMATCH = 0x02;
        /// Data ready interrupt
        const DRDY = 0x08;
        /// Error interrupt
        const ERROR = 0x80;
    }
}
