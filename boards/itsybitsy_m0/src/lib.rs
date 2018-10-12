#![no_std]
#![recursion_limit="1024"]

extern crate atsamd21_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use hal::atsamd21g18a::*;
use hal::prelude::*;
pub use hal::*;

use gpio::{Floating, Input, Port};

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: atsamd21g18a,

    pin d0 = a11,
    pin d1 = a10,
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
    pin d13 = a17,

    pin a0 = a2,
    pin a1 = b8,
    pin a2 = b9,
    pin a3 = a4,
    pin a4 = a5,
    pin a5 = b2,

    pin mosi = b10,
    pin miso = a12,
    pin sck = b11,

    pin flash_mosi = b22,
    pin flash_miso = b3,
    pin flash_sck = b23,
    pin flash_cs = a27,

    pin scl = a23,
    pin sda = a22,

    pin swdio = a31,
    pin swdclk = a30,
);
