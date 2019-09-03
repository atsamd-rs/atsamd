#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

#[cfg(feature = "panic_halt")]
pub extern crate panic_halt;

use hal::prelude::*;
use hal::*;

pub use hal::target_device as pac;
pub use hal::common::*;
pub use hal::samd21::*;

use gpio::{Floating, Input, Port};

// The docs could be further improved with details of the specific channels etc
define_pins!(
    /// Maps the pins to their arduino names and the numbers printed on the board.
    /// Information from: <https://github.com/arduino/ArduinoCore-samd/blob/master/variants/mkrzero/variant.cpp>
    struct Pins,
    target_device: target_device,

    /// Digital 0: PWM, TC
    pin d0 = a22,

    /// Digital 1: PWM, TC
    pin d1 = a23,

    /// Digital 2: PWM, TCC, ADC
    pin d2 = a10,

    /// Digital 3: PWM, TCC, ADC
    pin d3 = a11,

    /// Digital 4: PWM, TC
    pin d4 = b10,

    /// Digital 5: PWM, TC
    pin d5 = b11,

    /// Digital 6: PWM, TCC
    pin d6 = a20,

    /// Digital 7: PWM, TCC
    pin d7 = a21,

    /// SPI MOSI: PWM, TCC
    pin mosi = a16,

    /// SPI SCK
    pin sck = a17,

    /// SPI MISO: PWM, TC
    pin miso = a19,

    /// SDA
    pin sda = a8,

    /// SCL
    pin scl = a9,

    /// RX
    pin rx = b23,

    /// TX
    pin tx = b22,

    /// Analog 0: DAC
    pin a0 = a2,

    /// Analog 1
    pin a1 = b2,

    /// Analog 2
    pin a2 = b3,

    /// Analog 3: PWM, TCC
    pin a3 = a4,

    /// Analog 4: PWM, TCC
    pin a4 = a5,

    /// Analog 5
    pin a5 = a6,

    /// Analog 6
    pin a6 = a7,

    pin usb_n = a24,
    pin usb_p = a25,
    pin usb_id = a18,
    pin aref = a3,
    pin sd_sck = a12,
    pin sd_mosi = a13,
    pin sd_ss = a14,
    pin sd_miso = a15,
    pin sd_cd = a27,

    /// LED built into the board
    pin led_builtin = b8,

    pin bottom_pad = a28,
    pin adc_battery = b9,
    pin xin32 = a0,
    pin xout32 = a1,
);
