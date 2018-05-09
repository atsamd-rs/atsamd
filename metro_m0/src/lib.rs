#![no_std]

extern crate atsamd21_hal as hal;

pub use hal::atsamd21g18a::*;
use hal::prelude::*;
pub use hal::*;

use gpio::{Floating, Input, Output, Port, PushPull};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster3, PadPin, SPIMaster5};
use hal::time::Hertz;

/// Maps the pins to their arduino names and
/// the numbers printed on the board.
pub struct Pins {
    /// Opaque port reference
    pub port: Port,

    /// Analog pin 0.  Can act as a true analog output
    /// as it has a DAC (which is not currently supported
    /// by this hal) as well as input.
    pub a0: gpio::Pa2<Input<Floating>>,
    /// Analog Pin 1
    pub a1: gpio::Pb8<Input<Floating>>,
    /// Analog Pin 2
    pub a2: gpio::Pb9<Input<Floating>>,
    /// Analog Pin 3
    pub a3: gpio::Pa4<Input<Floating>>,
    /// Analog Pin 4
    pub a4: gpio::Pa5<Input<Floating>>,
    /// Analog Pin 5
    pub a5: gpio::Pb2<Input<Floating>>,

    /// Pin 0, rx.  Also analog input (A6)
    pub d0: gpio::Pa11<Input<Floating>>,
    /// Pin 1, tx.  Also analog input (A7)
    pub d1: gpio::Pa10<Input<Floating>>,
    /// Pin 2
    pub d2: gpio::Pa14<Input<Floating>>,
    /// Pin 3, PWM capable
    pub d3: gpio::Pa9<Input<Floating>>,
    /// Pin 4, PWM capable.  Also analog input (A8)
    pub d4: gpio::Pa8<Input<Floating>>,
    /// Pin 5, PWM capable.  Also analog input (A9)
    pub d5: gpio::Pa15<Input<Floating>>,
    /// Pin 6, PWM capable
    pub d6: gpio::Pa20<Input<Floating>>,
    /// Pin 7
    pub d7: gpio::Pa21<Input<Floating>>,
    /// Pin 8, PWM capable.  Also analog input (A10)
    pub d8: gpio::Pa6<Input<Floating>>,
    /// Pin 9, PWM capable.  Also analog input (A11)
    pub d9: gpio::Pa7<Input<Floating>>,
    /// Pin 10, PWM capable
    pub d10: gpio::Pa18<Input<Floating>>,
    /// Pin 11, PWM capable
    pub d11: gpio::Pa16<Input<Floating>>,
    /// Pin 12, PWM capable
    pub d12: gpio::Pa19<Input<Floating>>,
    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pub d13: gpio::Pa17<Input<Floating>>,
    pub sda: gpio::Pa22<Input<Floating>>,
    pub scl: gpio::Pa23<Input<Floating>>,

    /// The data line attached to the neopixel.
    /// Is also attached to SWCLK.
    pub neopixel: gpio::Pa30<Input<Floating>>,

    /// The SPI SCK attached the to 2x3 header
    pub sck: gpio::Pb11<Input<Floating>>,
    /// The SPI MOSI attached the to 2x3 header
    pub mosi: gpio::Pb10<Input<Floating>>,
    /// The SPI MISO attached the to 2x3 header
    pub miso: gpio::Pa12<Input<Floating>>,

    /// The SCK pin attached to the on-board SPI flash
    pub flash_sck: gpio::Pb23<Input<Floating>>,
    /// The MOSI pin attached to the on-board SPI flash
    pub flash_mosi: gpio::Pb22<Input<Floating>>,
    /// The MISO pin attached to the on-board SPI flash
    pub flash_miso: gpio::Pb3<Input<Floating>>,
    /// The CS pin attached to the on-board SPI flash
    pub flash_cs: gpio::Pa13<Input<Floating>>,
}

/// Returns the pins for the device
pub fn pins(port: atsamd21g18a::PORT) -> Pins {
    let pins = port.split();
    Pins {
        port: pins.port,
        a0: pins.pa2,
        a1: pins.pb8,
        a2: pins.pb9,
        a3: pins.pa4,
        a4: pins.pa5,
        a5: pins.pb2,
        d0: pins.pa11,
        d1: pins.pa10,
        d2: pins.pa14,
        d3: pins.pa9,
        d4: pins.pa8,
        d5: pins.pa15,
        d6: pins.pa20,
        d7: pins.pa21,
        d8: pins.pa6,
        d9: pins.pa7,
        d10: pins.pa18,
        d11: pins.pa16,
        d12: pins.pa19,
        d13: pins.pa17,
        sda: pins.pa22,
        scl: pins.pa23,
        neopixel: pins.pa30,
        sck: pins.pb11,
        mosi: pins.pb10,
        miso: pins.pa12,

        flash_sck: pins.pb23,
        flash_mosi: pins.pb22,
        flash_miso: pins.pb3,
        flash_cs: pins.pa13,
    }
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
