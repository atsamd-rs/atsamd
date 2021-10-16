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

use gpio::{self, *};

use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster3, PadPin, SPIMaster4, UART2};
use hal::time::Hertz;

#[cfg(feature = "usb")]
use gpio::v2::{AnyPin, PA24, PA25};
#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

// The docs could be further improved with details of the specific channels etc
define_pins!(
    /// Maps the pins to their arduino names and the numbers printed on the board.
    /// Information pulled from datasheet and board files for arduino IDE : <https://wiki.seeedstudio.com/Wio-Lite-MG126/#tech-support>
    struct Pins,
    pac: pac,

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
    pin d22 = a12,

    /// Digital 23: MOSI
    pin d23 = b10,

    /// Digital 0: RX
    pin rx = b3,

    /// Digital 1: TX
    pin tx = a27,

    /// USB Data Minus
    pin usb_dm = a24,

    /// USB Data Plus
    pin usb_dp = a25,

    // LED built into the board
    // pin led_builtin = a17,

    //pin bottom_pad = a28,
    //pin adc_battery = b9,
);

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
) -> hal::sercom::I2CMaster3<
    hal::sercom::Sercom3Pad0<gpio::Pa22<gpio::PfC>>,
    hal::sercom::Sercom3Pad1<gpio::Pa23<gpio::PfC>>,
> {
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

/// Convenience function for setting up the D24/SCK, D23/MOSI, and D22/MISO pins
/// as a SPI Master.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    speed: F,
    sercom4: pac::SERCOM4,
    pm: &mut pac::PM,
    sck: gpio::Pb11<Input<Floating>>,
    mosi: gpio::Pb10<Input<Floating>>,
    miso: gpio::Pa12<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster4<
    hal::sercom::Sercom4Pad0<gpio::Pa12<gpio::PfD>>,
    hal::sercom::Sercom4Pad2<gpio::Pb10<gpio::PfD>>,
    hal::sercom::Sercom4Pad3<gpio::Pb11<gpio::PfD>>,
> {
    let gclk0 = clocks.gclk0();

    SPIMaster4::new(
        &clocks.sercom4_core(&gclk0).unwrap(),
        speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom4,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}

/// Convenience for setting up the D0 and D1 pins to
/// operate as UART RX/TX (respectively) running at the specified baud.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom2: pac::SERCOM2,
    pm: &mut pac::PM,
    d0: gpio::Pa11<Input<Floating>>,
    d1: gpio::Pa10<Input<Floating>>,
    port: &mut Port,
) -> UART2<
    hal::sercom::Sercom2Pad3<gpio::Pa11<PfD>>,
    hal::sercom::Sercom2Pad2<gpio::Pa10<PfD>>,
    (),
    (),
> {
    let gclk0 = clocks.gclk0();

    UART2::new(
        &clocks.sercom2_core(&gclk0).unwrap(),
        baud.into(),
        sercom2,
        pm,
        (d0.into_pad(port), d1.into_pad(port)),
    )
}

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: impl AnyPin<Id = PA24>,
    dp: impl AnyPin<Id = PA25>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();

    UsbBusAllocator::new(UsbBus::new(usb_clock, pm, dm, dp, usb))
}
