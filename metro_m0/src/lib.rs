#![no_std]

extern crate atsamd21_hal as hal;

pub use hal::atsamd21g18a::*;
use hal::prelude::*;
pub use hal::*;

use gpio::{Floating, Input, Output, Port, PushPull};
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster3, PadPin, SPIMaster4, SPIMaster5};
use hal::time::Hertz;

macro_rules! pins {
    ($( $(#[$attr:meta])* pin $name:ident = $pin_name:ident: $pin_type:ident),+ , ) => {
/// Maps the pins to their arduino names and
/// the numbers printed on the board.
pub struct Pins {
    /// Opaque port reference
    pub port: Port,

    $($(#[$attr])* pub $name: gpio::$pin_type<Input<Floating>>),+
}

impl Pins {
    /// Returns the pins for the device
    pub fn new(port: atsamd21g18a::PORT) -> Self {
        let pins = port.split();
        Pins {
            port: pins.port,
            $($name: pins.$pin_name),+
        }
    }
}

    }
}

pins!(
    /// Analog pin 0.  Can act as a true analog output
    /// as it has a DAC (which is not currently supported
    /// by this hal) as well as input.
    pin a0 = pa2: Pa2,

    /// Analog Pin 1
    pin a1 = pb8: Pb8,
    /// Analog Pin 2
    pin a2 = pb9: Pb9,
    /// Analog Pin 3
    pin a3 = pa4: Pa4,
    /// Analog Pin 4
    pin a4 = pa5: Pa5,
    /// Analog Pin 5
    pin a5 = pb2: Pb2,

    /// Pin 0, rx.  Also analog input (A6)
    pin d0 = pa11: Pa11,
    /// Pin 1, tx.  Also analog input (A7)
    pin d1 = pa10: Pa10,
    /// Pin 2
    pin d2 = pa14: Pa14,
    /// Pin 3, PWM capable
    pin d3 = pa9: Pa9,
    /// Pin 4, PWM capable.  Also analog input (A8)
    pin d4 = pa8: Pa8,
    /// Pin 5, PWM capable.  Also analog input (A9)
    pin d5 = pa15: Pa15,
    /// Pin 6, PWM capable
    pin d6 = pa20: Pa20,
    /// Pin 7
    pin d7 = pa21: Pa21,
    /// Pin 8, PWM capable.  Also analog input (A10)
    pin d8 = pa6: Pa6,
    /// Pin 9, PWM capable.  Also analog input (A11)
    pin d9 = pa7: Pa7,
    /// Pin 10, PWM capable
    pin d10 = pa18: Pa18,
    /// Pin 11, PWM capable
    pin d11 = pa16: Pa16,
    /// Pin 12, PWM capable
    pin d12 = pa19: Pa19,
    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = pa17: Pa17,
    pin sda = pa22: Pa22,
    pin scl = pa23: Pa23,

    /// The data line attached to the neopixel.
    /// Is also attached to SWCLK.
    pin neopixel = pa30: Pa30,

    /// The SPI SCK attached the to 2x3 header
    pin sck = pb11: Pb11,
    /// The SPI MOSI attached the to 2x3 header
    pin mosi = pb10: Pb10,
    /// The SPI MISO attached the to 2x3 header
    pin miso = pa12: Pa12,

    /// The SCK pin attached to the on-board SPI flash
    pin flash_sck = pb23: Pb23,
    /// The MOSI pin attached to the on-board SPI flash
    pin flash_mosi = pb22: Pb22,
    /// The MISO pin attached to the on-board SPI flash
    pin flash_miso = pb3: Pb3,
    /// The CS pin attached to the on-board SPI flash
    pin flash_cs = pa13: Pa13,
);

/// Returns the pins for the device
pub fn pins(port: atsamd21g18a::PORT) -> Pins {
    Pins::new(port)
}

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
