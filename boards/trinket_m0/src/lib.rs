#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
pub use hal::target_device::*;
pub use hal::*;

use gpio::{Floating, Input, PfD, Port};

use hal::clock::GenericClockController;
use hal::sercom::{PadPin, UART0};
use hal::time::Hertz;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

    /// I2C SDA
    pin d0 = a8,
    pin d1 = a2,
    /// I2C SCL
    pin d2 = a9,
    /// UART RX
    pin d3 = a7,
    /// UART TX
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

/// Convenience for setting up the D3 and D4 pins to
/// operate as UART RX/TX (respectively) running at the specified baud.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom0: SERCOM0,
    nvic: &mut NVIC,
    pm: &mut PM,
    d3: gpio::Pa7<Input<Floating>>,
    d4: gpio::Pa6<Input<Floating>>,
    port: &mut Port,
) -> UART0<hal::sercom::Sercom0Pad3<gpio::Pa7<PfD>>, hal::sercom::Sercom0Pad2<gpio::Pa6<PfD>>, (), ()>
{
    let gclk0 = clocks.gclk0();

    UART0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        baud.into(),
        sercom0,
        nvic,
        pm,
        (d3.into_pad(port), d4.into_pad(port)),
    )
}
