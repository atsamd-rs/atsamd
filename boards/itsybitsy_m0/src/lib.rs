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

    /// Analog pin 0.  Can act as a true analog output
    /// as it has a DAC (which is not currently supported
    /// by this hal) as well as input.
    pin a0 = a2,
    /// Analog Pin 1
    pin a1 = b8,
    /// Analog Pin 2
    pin a2 = b9,
    /// Analog Pin 3
    pin a3 = a4,
    /// Analog Pin 4
    pin a4 = a5,
    /// Analog Pin 5
    pin a5 = b2,

    /// Pin 0, rx
    pin d0 = a11,
    /// Pin 1, tx
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
    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = a17,

    /// The SPI MOSI attached the to 2x3 header
    pin mosi = b10,
    /// The SPI MISO attached the to 2x3 header
    pin miso = a12,
    /// The SPI SCK attached the to 2x3 header
    pin sck = b11,

    /// The MOSI pin attached to the on-board SPI flash
    pin flash_mosi = b22,
    /// The MISO pin attached to the on-board SPI flash
    pin flash_miso = b3,
    /// The SCK pin attached to the on-board SPI flash
    pin flash_sck = b23,
    /// The CS pin attached to the on-board SPI flash
    pin flash_cs = a27,

    /// The I2C clock
    pin scl = a23,
    /// The I2C data line
    pin sda = a22,

    /// The DotStar clock
    pin dotstar_ci = a0,
    /// The DotStar data line
    pin dotstar_di = a1,

    pin swdio = a31,
    pin swdclk = a30,

    /// The USB D- pad
    pin usb_dm = a24,
    /// The USB D+ pad
    pin usb_dp = a25,
);
