#![no_std]
#![recursion_limit="1024"]

extern crate atsamd21_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use hal::atsamd21e18a::*;
use hal::prelude::*;
pub use hal::*;

use gpio::{Floating, Input, Port};

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: atsamd21e18a,

    pin d0 = a8,
    pin d1 = a2,
    pin d2 = a9,
    pin d3 = a7,
    pin d4 = a6,

    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = a10,

    /// The DotStar clock
    pin dotstar_ci = a1,
    /// The DotStar data line
    pin dotstar_di = a0,

    pin swdio = a31,
    pin swdclk = a30,

    /// The USB D- pad
    pin usb_dm = a24,
    /// The USB D+ pad
    pin usb_dp = a25,
);
