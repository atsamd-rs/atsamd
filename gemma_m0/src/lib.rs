#![no_std]
#![recursion_limit="1024"]

extern crate atsamd21_hal as hal;

pub use hal::atsamd21e18a::*;
use hal::prelude::*;
pub use hal::*;

use gpio::{Floating, Input, Port};

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: atsamd21e18a,

    pin d0 = a4,
    pin d1 = a2,
    pin d2 = a5,

    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = a23,
    pin mosi = a0,
    pin sck = a1,
);
