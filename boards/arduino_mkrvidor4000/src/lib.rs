#![no_std]

pub use atsamd_hal as hal;
pub use hal::common::*;
pub use hal::pac;

#[cfg(feature = "rt")]
use cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

use gpio::{Floating, Input, Port};

// The docs could be further improved with details of the specific channels etc
define_pins!(
    /// Maps the pins to their arduino names and the numbers printed on the board.
    /// Information from: <https://github.com/arduino/ArduinoCore-samd/blob/master/variants/mkrvidor4000/variant.cpp>
    struct Pins,
    pac: pac,

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
    /// Cannot be used because it is actually adc_battery
    /// pin a6 = a7,

    pin usb_n = a24,
    pin usb_p = a25,
    pin usb_id = a18,

    pin aref = a3,

    /// JTAG pins, also accessible from 10-pin pad on the bottom
    pin fpga_tdi = a12,
    pin fpga_tck = a13,
    pin fpga_tms = a14,
    pin fpga_tdo = a15,

    /// Interrupt pin for SAMD to interrupt FPGA
    pin fpga_mb_int = a28,

    /// Clock generator for FPGA
    pin gclk = a27,

    /// Accessible from 6 pin pad on the bottom
    pin swdio = a31,
    pin swclk = a30,

    /// LED built into the board
    pin led_builtin = b8,

    /// PMIC_BAT
    pin adc_battery = a7,

    pin xin32 = a0,
    pin xout32 = a1,
);
