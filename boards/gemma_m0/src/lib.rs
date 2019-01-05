#![no_std]

extern crate atsamd21_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
pub use hal::target_device::*;
pub use hal::*;

use gpio::{Floating, Input, Port};

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

    pin d0 = a4,
    pin d1 = a2,
    pin d2 = a5,

    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = a23,
    pin mosi = a0,
    pin sck = a1,
);
