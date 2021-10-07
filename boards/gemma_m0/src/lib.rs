#![no_std]

pub use atsamd_hal as hal;
pub use hal::common::*;
pub use hal::pac;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

use gpio::{Floating, Input, PfD, Port};

use hal::clock::GenericClockController;
use hal::sercom::{PadPin, UART0};
use hal::time::Hertz;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    pac: pac,

    /// I2C SDA, UART TX, SPI MOSI
    pin d0 = a4,
    /// ADC, DAC
    pin d1 = a2,
    /// I2C SCL, UART RX, SPI CLK
    pin d2 = a5,

    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = a23,

    // DotStar MOSI
    pin mosi = a0,
    // DotStar SCLK
    pin sck = a1,
);

/// Convenience for setting up the D2 and D0 pins to
/// operate as UART RX/TX (respectively) running at the specified baud.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    d2: gpio::Pa5<Input<Floating>>,
    d0: gpio::Pa4<Input<Floating>>,
    port: &mut Port,
) -> UART0<hal::sercom::Sercom0Pad1<gpio::Pa5<PfD>>, hal::sercom::Sercom0Pad0<gpio::Pa4<PfD>>, (), ()>
{
    let gclk0 = clocks.gclk0();

    UART0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        baud.into(),
        sercom0,
        pm,
        (d2.into_pad(port), d0.into_pad(port)),
    )
}
