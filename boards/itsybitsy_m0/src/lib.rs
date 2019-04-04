#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
pub use hal::target_device::*;
pub use hal::*;

use gpio::{Floating, Input, Output, Port, PushPull};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster3, PadPin, SPIMaster4, SPIMaster5};
use hal::time::Hertz;

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
    sercom4: SERCOM4,
    pm: &mut PM,
    sck: gpio::Pb11<Input<Floating>>,
    mosi: gpio::Pb10<Input<Floating>>,
    miso: gpio::Pa12<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster4 {
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
        hal::sercom::SPI4Pinout::Dipo0Dopo1 {
            miso: miso.into_pad(port),
            mosi: mosi.into_pad(port),
            sck: sck.into_pad(port),
        },
    )
}

/// Convenience for accessing the on-board SPI Flash device.
/// This powers up SERCOM5 and configures it for use as an
/// SPI Master.
pub fn flash_spi_master(
    clocks: &mut GenericClockController,
    sercom5: SERCOM5,
    pm: &mut PM,
    sck: gpio::Pb23<Input<Floating>>,
    mosi: gpio::Pb22<Input<Floating>>,
    miso: gpio::Pb3<Input<Floating>>,
    cs: gpio::Pa13<Input<Floating>>,
    port: &mut Port,
) -> (SPIMaster5, gpio::Pa13<Output<PushPull>>) {
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
        hal::sercom::SPI5Pinout::Dipo1Dopo1 {
            miso: miso.into_pad(port),
            mosi: mosi.into_pad(port),
            sck: sck.into_pad(port),
        },
    );

    let mut cs = cs.into_push_pull_output(port);
    cs.set_high();

    (flash, cs)
}

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom3: SERCOM3,
    pm: &mut PM,
    sda: gpio::Pa22<Input<Floating>>,
    scl: gpio::Pa23<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster3 {
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

#[cfg(feature = "usb")]
pub fn usb_bus(
    usb: USB,
    clocks: &mut GenericClockController,
    pm: &mut PM,
    dm: gpio::Pa24<Input<Floating>>,
    dp: gpio::Pa25<Input<Floating>>,
    port: &mut Port,
) -> UsbBusWrapper<UsbBus> {
    let gclk0 = clocks.gclk0();
    dbgprint!("making usb clock");
    let usb_clock = &clocks.usb(&gclk0).unwrap();
    dbgprint!("got clock");
    UsbBusWrapper::new(UsbBus::new(
        usb_clock,
        pm,
        dm.into_function(port),
        dp.into_function(port),
        usb,
    ))
}
