#![no_std]
#![recursion_limit = "1024"]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;

pub mod pins;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;
pub use hal::{*, atsamd51g19a::*};

use gpio::{Floating, Input, Port};

use hal::prelude::*;
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster4, PadPin};
use hal::time::Hertz;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: atsamd51g19a,

    /// Analog pin 0
    pin a0 = a2,
    /// Analog pin 1
    pin a1 = a5,
    /// INT pin
    pin a2 = a4,
    /// Microphone out
    pin micout = a6,
    /// Microphone in
    pin micin = a7,

    /// SDA
    pin sda = b8,
    /// SCL
    pin scl = b9,

    /// Accelerometer data signal (SDA)
    pin accel_sda = a12,
    /// Accelerometer clock signal (SCL)
    pin accel_scl = a13,

    /// Keypad Column 0
    pin col0 = a14,
    /// Keypad Column 1
    pin col1 = a15,
    /// Keypad Column 2
    pin col2 = a16,
    /// Keypad Column 3
    pin col3 = a17,
    /// Keypad Column 4
    pin col4 = a20,
    /// Keypad Column 5
    pin col5 = a21,
    /// Keypad Column 6
    pin col6 = a22,
    /// Keypad Column 7
    pin col7 = a23,

    /// Keypad Row 0
    pin row0 = a18,
    /// Keypad Row 1
    pin row1 = a19,
    /// Keypad Row 2
    pin row2 = b22,
    /// Keypad Row 3
    pin row3 = b23,

    /// NeoPixels
    pin neopixel = a27,

    /// APA102 (RGB LED control) SCK
    pin dotstar_ci = b2,
    /// APA102 (RGB LED control) MOSI
    pin dotstar_di = b3,
);

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> pins::Sets {
        let Self {
            port,
            a0,
            a1,
            a2,
            micout,
            micin,
            sda,
            scl,
            accel_sda,
            accel_scl,
            col0,
            col1,
            col2,
            col3,
            col4,
            col5,
            col6,
            col7,
            row0,
            row1,
            row2,
            row3,
            neopixel,
            dotstar_ci,
            dotstar_di,
        } = self;

        let accel = pins::Accelerometer {
            sda: accel_sda,
            scl: accel_scl,
        };

        let analog = pins::Analog { a0, a1, a2 };

        let audio = pins::Audio {
            input: micin,
            output: micout,
        };

        let dotstar = pins::Dotstar {
            ci: dotstar_ci,
            di: dotstar_di,
        };

        let i2c = pins::I2C { sda, scl };

        let keypad = pins::Keypad {
            col0,
            col1,
            col2,
            col3,
            col4,
            col5,
            col6,
            col7,
            row0,
            row1,
            row2,
            row3,
        };

        pins::Sets {
            accel,
            analog,
            audio,
            dotstar,
            i2c,
            keypad,
            neopixel,
            port,
        }
    }
}

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom4: SERCOM4,
    mclk: &mut MCLK,
    sda: gpio::Pb8<Input<Floating>>,
    scl: gpio::Pb9<Input<Floating>>,
    port: &mut Port,
) -> I2CMaster4 {
    let gclk0 = clocks.gclk0();
    I2CMaster4::new(
        &clocks.sercom4_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom4,
        mclk,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}
