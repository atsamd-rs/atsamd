#![no_std]
#![recursion_limit = "1024"]

extern crate atsamd21_hal as hal;

pub use hal::atsamd21g18a::*;
use hal::prelude::*;
pub use hal::*;

use gpio::{Floating, Input, Output, Port, PushPull};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster5, PadPin, SPIMaster3};
use hal::time::Hertz;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: atsamd21g18a,
    /// Pin 0, rx. Also analog input (A6)
    pin rx = b9,
    /// Pin 1, tx. Also analog input (A7)
    pin tx = b8,
    /// Pin 4, button A.
    pin d4 = a28,
    /// Pin 5, button B.
    pin d5 = a14,
    /// Pin 7, slide switch.
    pin d7 = a15,
    /// Pin 11, speaker enable.
    pin d11 = a30,
    /// Digital pin number 13, which is also attached to the red LED. PWM capable.
    pin d13 = a17,
    /// The I2C SDA. Also D2 and A5.
    pin sda = b2,
    /// The I2C SCL. Also D3 and A4
    pin scl = b3,

    /// The data line attached to the neopixel. Also D8.
    pin neopixel = b23,

    /// The line attached to the speaker. Also D12 and A0.
    pin speaker = a2,

    /// The SPI SCK. Also D6 and A1
    pin sck = a5,
    /// The SPI MOSI. Also D10 and A3
    pin mosi = a7,
    /// The SPI MISO. Also D9 and A2
    pin miso = a6,

    /// The SCK pin attached to the on-board SPI flash
    pin flash_sck = a21,
    /// The MOSI pin attached to the on-board SPI flash
    pin flash_mosi = a20,
    /// The MISO pin attached to the on-board SPI flash
    pin flash_miso = a16,
    /// The CS pin attached to the on-board SPI flash
    pin flash_cs = b22,
);

/// Convenience for accessing the on-board SPI Flash device.
/// This powers up SERCOM5 and configures it for use as an
/// SPI Master.
pub fn flash_spi_master(
    clocks: &mut GenericClockController,
    sercom3: SERCOM3,
    pm: &mut PM,
    sck: gpio::Pa21<Input<Floating>>,
    mosi: gpio::Pa20<Input<Floating>>,
    miso: gpio::Pa16<Input<Floating>>,
    cs: gpio::Pb22<Input<Floating>>,
    port: &mut Port,
) -> (SPIMaster3, gpio::Pb22<Output<PushPull>>) {
    let gclk0 = clocks.gclk0();
    let flash = SPIMaster3::new(
        &clocks.sercom3_core(&gclk0).unwrap(),
        48.mhz(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom3,
        pm,
        hal::sercom::SPI3Pinout::Dipo0Dopo1 {
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
    sercom5: SERCOM5,
    pm: &mut PM,
    sda: gpio::Pb2<Input<Floating>>,
    scl: gpio::Pb3<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster5 {
    let gclk0 = clocks.gclk0();
    I2CMaster5::new(
        &clocks.sercom5_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom5,
        pm,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}
