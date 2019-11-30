#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

pub use hal::common::*;
pub use hal::samd11::*;
pub use hal::target_device as pac;

use gpio::{Floating, Input, PfC, Port};

use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster0, PadPin, UART0};
use hal::time::Hertz;

define_pins!(
    /// Maps the pins to their physical pins.
    struct Pins,
    target_device: target_device,

    pin d1 = a5,
    pin d2 = a8,
    pin d3 = a9,
    pin d4 = a14,
    pin d5 = a15,
    pin d6 = a28, // RST
    pin d7 = a30,

    pin d8 = a31,
    pin d9 = a24,
    pin d10 = a25,

    // 11 & 12 are GND/VCC

    pin d13 = a2,
    pin d14 = a4,
);

/// Convenience for setting up the D1 and D14 pins to
/// operate as UART RX/TX (respectively) running at the specified baud.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    d1: gpio::Pa5<Input<Floating>>,
    d14: gpio::Pa4<Input<Floating>>,
    port: &mut Port,
) -> UART0<hal::sercom::Sercom0Pad3<gpio::Pa5<PfC>>, hal::sercom::Sercom0Pad2<gpio::Pa4<PfC>>, (), ()>
{
    let gclk0 = clocks.gclk0();

    UART0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        baud.into(),
        sercom0,
        pm,
        (d1.into_pad(port), d14.into_pad(port)),
    )
}

/// Convenience for setting up the D4 and D5 pins to operate as I²C
/// SDA/SDL (respectively) running at the specified baud.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    sda: gpio::Pa14<Input<Floating>>,
    scl: gpio::Pa15<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster0<
    hal::sercom::Sercom0Pad0<gpio::Pa14<gpio::PfC>>,
    hal::sercom::Sercom0Pad1<gpio::Pa15<gpio::PfC>>,
> {
    let gclk0 = clocks.gclk0();

    I2CMaster0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom0,
        pm,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}
