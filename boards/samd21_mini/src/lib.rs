#![no_std]

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

extern crate atsamd_hal as hal;

pub use hal::target_device as pac;
pub use hal::common::*;
pub use hal::samd21::*;

use hal::prelude::*;
use hal::*;

#[cfg(feature = "usb")]
pub use hal::usb;

use gpio::{Floating, Input, Port};

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

    pin tx = a10,
    pin rx = a11,

    pin aref = a3,

    pin d2 = a14,
    pin d3 = a9,
    pin d4 = a8,
    pin d5 = a15,
    pin d6 = a20,
    pin d7 = a21,
    pin d8 = a6,
    pin d9 = a7,
    pin d10 = a18,
    pin d11 = a16,
    pin d12 = a19,

    pin a3 = a4,
    pin a2 = b9,
    pin a1 = b8,
    pin a0 = a2,

    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin led = a17,
    pin tx_led = a27,
    pin rx_led = b3,

    pin dm = a24,
    pin dp = a25,
);
