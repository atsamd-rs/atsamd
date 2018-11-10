#![no_std]
#![recursion_limit = "1024"]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use hal::atsamd51j19a::*;
use hal::prelude::*;
pub use hal::*;

use gpio::{Floating, Input, IntoFunction, Output, Port, PushPull};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster5, PadPin, SPIMaster0, SPIMaster2};
use hal::time::Hertz;

#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;
#[cfg(feature = "usb")]
use usb_device::bus::UsbBusWrapper;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: atsamd51j19a,

    /// Analog pin 0.  Can act as a true analog output
    /// as it has a DAC (which is not currently supported
    /// by this hal) as well as input.
    pin a0 = a2,

    /// Analog Pin 1
    pin a1 = a5,
    /// Analog Pin 2
    pin a2 = a6,
    /// Analog Pin 3
    pin a3 = a4,
    /// Analog Pin 4
    pin a4 = b8,
    /// Analog Pin 5
    pin a5 = b9,

    /// Pin 0, rx.
    pin d0 = a23,
    /// Pin 1, tx.
    pin d1 = a22,
    /// Pin 2
    pin d2 = b17,
    /// Pin 3 
    pin d3 = b16,
    /// Pin 4
    pin d4 = b13,
    /// Pin 5
    pin d5 = b14,
    /// Pin 6
    pin d6 = b15,
    /// Pin 7
    pin d7 = b12,
    /// Pin 8
    pin d8 = a21,
    /// Pin 9
    pin d9 = a20,
    /// Pin 10
    pin d10 = a18,
    /// Pin 11
    pin d11 = a19,
    /// Pin 12
    pin d12 = a17,
    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = a16,
    pin sda = b2,
    pin scl = b3,

    /// The data line attached to the neopixel.
    /// Is also attached to SWCLK.
    pin neopixel = b22,

    /// The SPI SCK attached the to 2x3 header
    pin sck = a13,
    /// The SPI MOSI attached the to 2x3 header
    pin mosi = a12,
    /// The SPI MISO attached the to 2x3 header
    pin miso = a14,

    /// The SCK pin attached to the on-board SPI flash
    pin flash_sck = b10,
    /// The MOSI pin attached to the on-board SPI flash
    pin flash_mosi = a8,
    /// The MISO pin attached to the on-board SPI flash
    pin flash_miso = a9,
    /// The CS pin attached to the on-board SPI flash
    pin flash_cs = b11,

    /// The USB D- pad
    pin usb_dm = a24,
    /// The USB D+ pad
    pin usb_dp = a25,
);

/// Convenience for setting up the 2x3 header block for SPI.
/// This powers up SERCOM2 and configures it for use as an
/// SPI Master in SPI Mode 0.
/// Unlike the `flash_spi_master` function, this
/// one does not accept a CS pin; configuring a pin for CS
/// is the responsibility of the caller, because it could be
/// any OutputPin, or even a pulled up line on the slave.
pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom2: SERCOM2,
    mclk: &mut MCLK,
    sck: gpio::Pa13<Input<Floating>>,
    mosi: gpio::Pa12<Input<Floating>>,
    miso: gpio::Pa14<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster2 {
    let gclk0 = clocks.gclk0();
    SPIMaster2::new(
        &clocks.sercom2_core(&gclk0).unwrap(),
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom2,
        mclk,
        hal::sercom::SPI2Pinout::Dipo2Dopo0 {
            miso: miso.into_pad(port),
            mosi: mosi.into_pad(port),
            sck: sck.into_pad(port),
        },
    )
}

// Convenience for accessing the on-board SPI Flash device.
// This is commented out because it should use the QSPI peripheral, not a sercom,
// so we will need to add hal around the QSPI peripheral if we want this feature.
//
/*
 *pub fn flash_spi_master(
 *    clocks: &mut GenericClockController,
 *    sercom0: SERCOM0,
 *    mclk: &mut MCLK,
 *    sck: gpio::Pb10<Input<Floating>>,
 *    mosi: gpio::Pa8<Input<Floating>>,
 *    miso: gpio::Pa9<Input<Floating>>,
 *    cs: gpio::Pb11<Input<Floating>>,
 *    port: &mut Port,
 *) -> (SPIMaster0, gpio::Pb11<Output<PushPull>>) {
 *    let gclk0 = clocks.gclk0();
 *    let flash = SPIMaster0::new(
 *        &clocks.sercom0_core(&gclk0).unwrap(),
 *        48.mhz(),
 *        hal::hal::spi::Mode {
 *            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
 *            polarity: hal::hal::spi::Polarity::IdleLow,
 *        },
 *        sercom0,
 *        mclk,
 *        hal::sercom::SPI0Pinout::Dipo1Dopo3 {
 *            miso: miso.into_pad(port),
 *            mosi: mosi.into_pad(port),
 *            sck: sck.into_pad(port),
 *        },
 *    );
 *
 *    let mut cs = cs.into_push_pull_output(port);
 *    cs.set_high();
 *
 *    (flash, cs)
 *}
 */

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom5: SERCOM5,
    mclk: &mut MCLK,
    sda: gpio::Pb2<Input<Floating>>,
    scl: gpio::Pb3<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster5 {
    let gclk0 = clocks.gclk0();
    I2CMaster5::new(
        &clocks.sercom5_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom5,
        mclk,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}
