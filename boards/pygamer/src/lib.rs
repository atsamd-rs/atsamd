#![no_std]
#![recursion_limit = "1024"]

pub mod pins;
use atsamd_hal as hal;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use crate::pins::Pins;
pub use hal::target_device::*;
pub use hal::*;

use hal::prelude::*;
use gpio::{Floating, Input, Port, Output, PushPull};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster2, PadPin, SPIMaster1, SPIMaster4};
use hal::time::Hertz;

use embedded_hal::digital::v1::OutputPin;

#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;
#[cfg(feature = "usb")]
use usb_device::bus::UsbBusWrapper;

/// This powers up SERCOM1 and configures it for use as an
/// SPI Master in SPI Mode 0.
/// This function does not accept a CS pin; configuring a pin for CS
/// is the responsibility of the caller, because it could be
/// any OutputPin, or even a pulled up line on the slave.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom1: SERCOM1,
    mclk: &mut MCLK,
    miso: gpio::Pb22<Input<Floating>>,
    mosi: gpio::Pb23<Input<Floating>>,
    sck: gpio::Pa17<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster1 {
    let gclk0 = clocks.gclk0();
    SPIMaster1::new(
        &clocks.sercom1_core(&gclk0).unwrap(),
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom1,
        mclk,
        hal::sercom::SPI1Pinout::Dipo2Dopo2 {
            miso: miso.into_pad(port),
            mosi: mosi.into_pad(port),
            sck: sck.into_pad(port),
        },
    )
}

/// Convenience for accessing the on-board TFT LCD.
/// This powers up SERCOM4 and configures it for use as an
/// SPI Master.
pub fn tft_spi_master(
    clocks: &mut GenericClockController,
    sercom4: SERCOM4,
    pm: &mut MCLK,
    miso: gpio::Pb14<Input<Floating>>,
    mosi: gpio::Pb15<Input<Floating>>,
    sck: gpio::Pb13<Input<Floating>>,
    cs: gpio::Pb12<Input<Floating>>,
    port: &mut Port,
) -> (SPIMaster4, gpio::Pb12<Output<PushPull>>) {
    let gclk0 = clocks.gclk0();
    let tft_spi = SPIMaster4::new(
        &clocks.sercom4_core(&gclk0).unwrap(),
        48.mhz(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom4,
        pm,
        hal::sercom::SPI4Pinout::Dipo2Dopo2 {
            miso: miso.into_pad(port),
            mosi: mosi.into_pad(port),
            sck: sck.into_pad(port),
        },
    );

    let mut cs = cs.into_push_pull_output(port);
    cs.set_high();

    (tft_spi, cs)
}

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom2: SERCOM2,
    mclk: &mut MCLK,
    sda: gpio::Pa12<Input<Floating>>,
    scl: gpio::Pa13<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster2 {
    let gclk0 = clocks.gclk0();
    I2CMaster2::new(
        &clocks.sercom2_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom2,
        mclk,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}
