#![no_std]
#![recursion_limit="1024"]

extern crate atsamd21_hal as hal;

pub use hal::atsamd21g18a::*;
use hal::prelude::*;
pub use hal::*;

use gpio::{Floating, Input, Output, Port, PushPull};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster3, PadPin, SPIMaster4, SPIMaster5};
use hal::time::Hertz;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: atsamd21g18a,

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

    /// Pin 0, rx.  Also analog input (A6)
    pin d0 = a11,
    /// Pin 1, tx.  Also analog input (A7)
    pin d1 = a10,
    /// Pin 2
    pin d2 = a14,
    /// Pin 3, PWM capable
    pin d3 = a9,
    /// Pin 4, PWM capable.  Also analog input (A8)
    pin d4 = a8,
    /// Pin 5, PWM capable.  Also analog input (A9)
    pin d5 = a15,
    /// Pin 6, PWM capable
    pin d6 = a20,
    /// Pin 7
    pin d7 = a21,
    /// Pin 8, PWM capable.  Also analog input (A10)
    pin d8 = a6,
    /// Pin 9, PWM capable.  Also analog input (A11)
    pin d9 = a7,
    /// Pin 10, PWM capable
    pin d10 = a18,
    /// Pin 11, PWM capable
    pin d11 = a16,
    /// Pin 12, PWM capable
    pin d12 = a19,
    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = a17,
    pin sda = a22,
    pin scl = a23,

    /// The data line attached to the neopixel.
    /// Is also attached to SWCLK.
    pin neopixel = a30,

    /// The SPI SCK attached the to 2x3 header
    pin sck = b11,
    /// The SPI MOSI attached the to 2x3 header
    pin mosi = b10,
    /// The SPI MISO attached the to 2x3 header
    pin miso = a12,

    /// The SCK pin attached to the on-board SPI flash
    pin flash_sck = b23,
    /// The MOSI pin attached to the on-board SPI flash
    pin flash_mosi = b22,
    /// The MISO pin attached to the on-board SPI flash
    pin flash_miso = b3,
    /// The CS pin attached to the on-board SPI flash
    pin flash_cs = a13,
);

/// Convenience for setting up the 2x3 header block for SPI.
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
