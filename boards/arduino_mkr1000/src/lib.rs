#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

pub use hal::target_device as pac;
pub use hal::common::*;
pub use hal::samd21::*;

use gpio::{Floating, Input, Port};

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

    /// RX
    pin rx = b23,

    /// TX
    pin tx = b22,

    /// Digital 0
    pin d0 = a22,

    /// Digital 1
    pin d1 = a23,

    /// Digital 2: ADC
    pin d2 = a10,

    /// Digital 3: ADC
    pin d3 = a11,

    /// Digital 4
    pin d4 = b10,

    /// Digital 5
    pin d5 = b11,

    /// Digital 6: LED_BUILTIN
    pin d6 = a20,

    /// Digital 7
    pin d7 = a21,

    /// Digital 8/SC1 MOSI
    pin mosi = a16,

    /// Digital 9/SC1 SCK
    pin sck = a17,

    /// Digital 10/SC1 MISO
    pin miso = a19,

    /// Digital 11/SC2 SDA
    pin sda = a8,

    /// Digital 12/SC2 SCL
    pin scl = a9,

    /// Analog 0/DAC0
    pin a0 = a2,

    /// Analog 1
    pin a1 = b2,

    /// Analog 2
    pin a2 = b3,

    /// Analog 3
    pin a3 = a4,

    /// Analog 4
    pin a4 = a5,

    /// Analog 5
    pin a5 = a6,

    /// Analog 6
    pin a6 = a7,

    pin usb_dn = a24,
    pin usb_dp = a25,
    pin usb_id = a18,

    pin aref = a3,
);
