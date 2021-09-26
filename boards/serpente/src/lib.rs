#![no_std]

pub use atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

pub use hal::common::*;

pub use hal::pac;

use gpio::{Floating, Input, PfC, Port};

use hal::clock::GenericClockController;
use hal::sercom::{PadPin, UART0};
use hal::time::Hertz;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    pac: pac,

    /// SPI MOSI UART TX2
    pin d0 = a4,
    /// SPI SCK, UART RX2
    pin d1 = a5,
    /// SPI MISO
    pin d2 = a6,

    /// Digital/Analog
    pin d3 = a7,

    /// I2C SDA, UART TX
    pin d4 = a8,

    /// I2C SCL, UART RX
    pin d5 = a9,

    /// RGB LED, PWM capable
    pin led_g = a19,
    pin led_r = a22,
    pin led_b = a23,

    /// SPI Flash
    pin flash_mosi = a16,
    pin flash_miso = a18,
    pin flash_sck = a17,
    pin flash_cs = a15,
);

/// Convenience for setting up the D2 and D0 pins to
/// operate as UART RX/TX (respectively) running at the specified baud.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    d5: gpio::Pa9<Input<Floating>>,
    d4: gpio::Pa8<Input<Floating>>,
    port: &mut Port,
) -> UART0<hal::sercom::Sercom0Pad1<gpio::Pa9<PfC>>, hal::sercom::Sercom0Pad0<gpio::Pa8<PfC>>, (), ()>
{
    let gclk0 = clocks.gclk0();

    UART0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        baud.into(),
        sercom0,
        pm,
        (d5.into_pad(port), d4.into_pad(port)),
    )
}
