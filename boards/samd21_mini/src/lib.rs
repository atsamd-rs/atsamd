#![no_std]

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

extern crate atsamd_hal as hal;

pub use hal::common::*;
pub use hal::samd21::*;
pub use hal::target_device as pac;

use hal::prelude::*;
use hal::*;

use gpio::{Floating, Input, Port};

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

    pin tx = a10,
    pin rx = a11,

    pin aref = a3,

    pin d2 = a14,
    pin d3 = a9,
    pin d4 = a8,
    pin d5 = a15,
    pin d6 = a20,
    pin d7 = a21,
    pin d8 = a6,
    pin d9 = a7,
    pin d10 = a18,
    pin d11 = a16,
    pin d12 = a19,

    pin a3 = a4,
    pin a2 = b9,
    pin a1 = b8,
    pin a0 = a2,

    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin led = a17,
    pin tx_led = a27,
    pin rx_led = b3,

    pin dm = a24,
    pin dp = a25,
);

/// Convenience for setting up the labelled SPI peripheral.
/// This powers up SERCOM1 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_sercom1<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom1: pac::SERCOM1,
    pm: &mut pac::PM,
    sck: gpio::Pa19<Input<Floating>>,
    mosi: gpio::Pa18<Input<Floating>>,
    miso: gpio::Pa16<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster1<
    hal::sercom::Sercom1Pad0<gpio::Pa16<gpio::PfC>>,
    hal::sercom::Sercom1Pad2<gpio::Pa18<gpio::PfC>>,
    hal::sercom::Sercom1Pad3<gpio::Pa19<gpio::PfC>>,
> {
    let gclk0 = clocks.gclk0();
    SPIMaster1::new(
        &clocks.sercom1_core(&gclk0).unwrap(),
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom1,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    d0: gpio::Pa11<Input<Floating>>,
    d1: gpio::Pa10<Input<Floating>>,
    port: &mut Port,
) -> UART0<
    hal::sercom::Sercom0Pad3<gpio::Pa11<PfC>>,
    hal::sercom::Sercom0Pad2<gpio::Pa10<PfC>>,
    (),
    (),
> {
    let gclk0 = clocks.gclk0();

    UART0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        baud.into(),
        sercom0,
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
