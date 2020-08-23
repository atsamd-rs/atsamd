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

use gpio::{Floating, Input, PfD, Port};

use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster2, PadPin, UART0};
use hal::time::Hertz;

#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

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

    /// USB host enable pin
    pin usb_host_enable = a28,

    /// The USB SOF 1kHz pad
    pin usb_sof = a23,
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
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
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
        pm,
        (d3.into_pad(port), d4.into_pad(port)),
    )
}

/// Convenience for setting up the D0 and D2 pins to operate as I²C
/// SDA/SDL (respectively) running at the specified baud.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom2: pac::SERCOM2,
    pm: &mut pac::PM,
    sda: gpio::Pa8<Input<Floating>>,
    scl: gpio::Pa9<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster2<
    hal::sercom::Sercom2Pad0<gpio::Pa8<gpio::PfD>>,
    hal::sercom::Sercom2Pad1<gpio::Pa9<gpio::PfD>>,
> {
    let gclk0 = clocks.gclk0();

    I2CMaster2::new(
        &clocks.sercom2_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom2,
        pm,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: gpio::Pa24<Input<Floating>>,
    dp: gpio::Pa25<Input<Floating>>,
    port: &mut Port,
) -> UsbBusAllocator<UsbBus> {
    use gpio::IntoFunction;

    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();

    UsbBusAllocator::new(UsbBus::new(
        usb_clock,
        pm,
        dm.into_function(port),
        dp.into_function(port),
        usb,
    ))
}
