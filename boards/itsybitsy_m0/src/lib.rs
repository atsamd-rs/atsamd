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

use gpio::{Floating, Input, Port, PfC, IntoFunction};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster3, PadPin, SPIMaster4, SPIMaster5, UART0};
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

    /// Analog pin 0.  Can act as a true analog output
    /// as it has a DAC (which is not currently supported
    /// by this hal) as well as input.
    pin a0 = a2,
    /// Analog Pin 1
    pin a1 = b8,
    /// Analog Pin 2
    pin a2 = b9,
    /// Analog Pin 3
    pin a3 = a4,
    /// Analog Pin 4
    pin a4 = a5,
    /// Analog Pin 5
    pin a5 = b2,

    /// Pin 0, rx
    pin d0 = a11,
    /// Pin 1, tx
    pin d1 = a10,
    pin d2 = a14,
    pin d3 = a9,
    pin d4 = a8,
    pin d5 = a15,
    pin d7 = a21,
    pin d9 = a7,
    pin d10 = a18,
    pin d11 = a16,
    pin d12 = a19,
    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = a17,

    /// The SPI MOSI
    pin mosi = b10,
    /// The SPI MISO
    pin miso = a12,
    /// The SPI SCK
    pin sck = b11,

    /// The MOSI pin attached to the on-board SPI flash
    pin flash_mosi = b22,
    /// The MISO pin attached to the on-board SPI flash
    pin flash_miso = b3,
    /// The SCK pin attached to the on-board SPI flash
    pin flash_sck = b23,
    /// The CS pin attached to the on-board SPI flash
    pin flash_cs = a27,

    /// The I2C clock
    pin scl = a23,
    /// The I2C data line
    pin sda = a22,

    /// The DotStar clock
    pin dotstar_ci = a0,
    /// The DotStar data line
    pin dotstar_di = a1,

    pin swdio = a31,
    pin swdclk = a30,

    /// The USB D- pad
    pin usb_dm = a24,
    /// The USB D+ pad
    pin usb_dp = a25,
);

/// Convenience for setting up the externally labelled SPI.
/// This powers up SERCOM4 and configures it for use as an
/// SPI Master in SPI Mode 0.
/// Unlike the `flash_spi_master` function, this
/// one does not accept a CS pin; configuring a pin for CS
/// is the responsibility of the caller, because it could be
/// any OutputPin, or even a pulled up line on the slave.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom4: pac::SERCOM4,
    pm: &mut pac::PM,
    sck: gpio::Pb11<Input<Floating>>,
    mosi: gpio::Pb10<Input<Floating>>,
    miso: gpio::Pa12<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster4<
        hal::sercom::Sercom4Pad0<gpio::Pa12<gpio::PfD>>,
        hal::sercom::Sercom4Pad2<gpio::Pb10<gpio::PfD>>,
        hal::sercom::Sercom4Pad3<gpio::Pb11<gpio::PfD>>
    > {
    let gclk0 = clocks.gclk0();
    SPIMaster4::new(
        &clocks.sercom4_core(&gclk0).unwrap(),
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom4,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port))
    )
}

/// Convenience for accessing the on-board SPI Flash device.
/// This powers up SERCOM5 and configures it for use as an
/// SPI Master.
pub fn flash_spi_master(
    clocks: &mut GenericClockController,
    sercom5: pac::SERCOM5,
    pm: &mut pac::PM,
    sck: gpio::Pb23<Input<Floating>>,
    mosi: gpio::Pb22<Input<Floating>>,
    miso: gpio::Pb3<Input<Floating>>,
    cs: gpio::Pa13<Input<Floating>>,
    port: &mut Port,
) -> (hal::sercom::SPIMaster5<
        hal::sercom::Sercom5Pad1<hal::gpio::Pb3<hal::gpio::PfD>>,
        hal::sercom::Sercom5Pad2<hal::gpio::Pb22<hal::gpio::PfD>>,
        hal::sercom::Sercom5Pad3<hal::gpio::Pb23<hal::gpio::PfD>>
    >,
    hal::gpio::Pa13<hal::gpio::Output<hal::gpio::PushPull>>
) {
    let gclk0 = clocks.gclk0();
    let flash = SPIMaster5::new(
        &clocks.sercom5_core(&gclk0).unwrap(),
        48.mhz(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom5,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    );

    let mut cs = cs.into_push_pull_output(port);

    // We’re confident that set_high won’t error here because on-board
    // GPIO pins don’t error.
    cs.set_high().unwrap();

    (flash, cs)
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
) -> I2CMaster3<
        hal::sercom::Sercom3Pad0<hal::gpio::Pa22<hal::gpio::PfC>>,
        hal::sercom::Sercom3Pad1<hal::gpio::Pa23<hal::gpio::PfC>>
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
    dm: gpio::Pa24<Input<Floating>>,
    dp: gpio::Pa25<Input<Floating>>,
    port: &mut Port,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    dbgprint!("making usb clock");
    let usb_clock = &clocks.usb(&gclk0).unwrap();
    dbgprint!("got clock");

    UsbBusAllocator::new(UsbBus::new(
        usb_clock,
        pm,
        dm.into_function(port),
        dp.into_function(port),
        usb,
    ))
}
