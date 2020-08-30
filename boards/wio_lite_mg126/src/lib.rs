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
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster3, PadPin};
use hal::time::Hertz;

#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

// The docs could be further improved with details of the specific channels etc
define_pins!(
    /// Maps the pins to their arduino names and the numbers printed on the board.
    /// Information pulled from datasheet and board files for arduino IDE : <https://wiki.seeedstudio.com/Wio-Lite-MG126/#tech-support>
    struct Pins,
    target_device: target_device,

    /// Digital 32: SDA
    pin d32 = a22,

    /// Digital 33: SCL
    pin d33 = a23,

    /// Digital 5: PWM, TC
    pin d5 = a15,

    /// Digital 6: PWM, TCC
    pin d6 = a20,

    /// Digital 9
    pin d9 = a7,

    /// Digital 10
    pin d10 = a18,

    /// Digital 11
    pin d11 = a16,

    /// Digital 12
    pin d12 = a19,

    /// Digital 13
    pin d13 = a17,

    /// Analog 0: DAC
    pin a0 = a2,

    /// Analog 1
    pin a1 = b8,

    /// Analog 2
    pin a2 = b9,

    /// Analog 3: PWM, TCC
    pin a3 = a4,

    /// Analog 4: PWM, TCC
    pin a4 = a5,

    /// Analog 5
    pin a5 = b2,

    /// Digital 24: SCK
    pin d24 = b11,

    /// Digital 22: MISO
    pin d22 = b12,

    /// Digital 23: MOSI
    pin d23 = b10,

    /// Digital 0: RX
    pin rx = b3,

    /// Digital 1: TX
    pin tx = a27,

    pin usb_dm = a24,
    pin usb_dp = a25,
    // LED built into the board
    // pin led_builtin = a17,

    //pin bottom_pad = a28,
    //pin adc_battery = b9,
);


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

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom3: pac::SERCOM3,
    pm: &mut pac::PM,
    sda: gpio::Pa22<Input<Floating>>,
    scl: gpio::Pa23<Input<Floating>>,
    port: &mut Port,
) -> hal::sercom::I2CMaster3<hal::sercom::Sercom3Pad0<gpio::Pa22<gpio::PfC>>, hal::sercom::Sercom3Pad1<gpio::Pa23<gpio::PfC>>> {
    let gclk0 = clocks.gclk0();
    I2CMaster3::new(
        &clocks.sercom3_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom3,
        pm,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}
