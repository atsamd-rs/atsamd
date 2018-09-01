#![no_std]

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

    pin d0 = pa4: Pa4,
    pin d1 = pa2: Pa2,
    pin d2 = pa5: Pa5,

    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = pa23: Pa23,
    pin mosi = pa0: Pa0,
    pin sck = pa1: Pa1,
);
