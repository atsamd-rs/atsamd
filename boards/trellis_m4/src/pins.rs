//! NeoTrellis M4 Express pins

use gpio::{Floating, Input, Port};
#[cfg(feature = "adxl343")]
use hal::{prelude::*, sercom::I2CError};
use hal::clock::*;
use hal::gpio::*;
use hal::sercom::{I2CMaster2, I2CMaster4, PadPin};
use hal::time::Hertz;
use super::{SERCOM2, SERCOM4, MCLK};

#[cfg(feature = "adxl343")]
use adxl343::Adxl343;

/// Sets of pins split apart by category
pub struct Sets {
    /// Accelerometer pins
    pub accel: Accelerometer,

    /// Analog pins
    pub analog: Analog,

    /// Audio pins
    pub audio: Audio,

    /// Dotstar (RGB LED) pins
    pub dotstar: Dotstar,

    /// I2C pins
    pub i2c: I2C,

    /// Keypad pins
    pub keypad: Keypad,

    /// Neopixel pins
    pub neopixel: NeoPixel,

    /// Port
    pub port: Port,
}

/// Accelerometer pins
pub struct Accelerometer {
    pub sda: Pa12<Input<Floating>>,
    pub scl: Pa13<Input<Floating>>,
}

impl Accelerometer {
    /// Open the Accelerometer I2C device
    #[cfg(feature = "adxl343")]
    pub fn open(
        self,
        clocks: &mut GenericClockController,
        sercom: SERCOM2,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> Result<Adxl343<I2CMaster2>, I2CError> {
        Adxl343::new(self.i2c_master(clocks, 100.khz(), sercom, mclk, port))
    }

    /// Configure accelerometer's SDA and SCL pins as an I2C master"
    pub fn i2c_master<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        bus_speed: F,
        sercom: SERCOM2,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> I2CMaster2 {
        let gclk0 = clocks.gclk0();
        I2CMaster2::new(
            &clocks.sercom2_core(&gclk0).unwrap(),
            bus_speed.into(),
            sercom,
            mclk,
            self.sda.into_pad(port),
            self.scl.into_pad(port),
        )
    }
}

/// Analog pins
pub struct Analog {
    pub a0: Pa2<Input<Floating>>,
    pub a1: Pa5<Input<Floating>>,
    pub a2: Pa4<Input<Floating>>,
}

/// Audio pins
pub struct Audio {
    pub output: Pa6<Input<Floating>>,
    pub input: Pa7<Input<Floating>>,
}

/// Dotstar pins
pub struct Dotstar {
    pub ci: Pb2<Input<Floating>>,
    pub di: Pb3<Input<Floating>>,
}

/// I2C pins
pub struct I2C {
    pub sda: Pb8<Input<Floating>>,
    pub scl: Pb9<Input<Floating>>,
}

impl I2C {
    /// Convenience for setting up the labelled SDA, SCL pins to
    /// operate as an I2C master running at the specified frequency.
    pub fn i2c_master<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        bus_speed: F,
        sercom4: SERCOM4,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> I2CMaster4 {
        let gclk0 = clocks.gclk0();
        I2CMaster4::new(
            &clocks.sercom4_core(&gclk0).unwrap(),
            bus_speed.into(),
            sercom4,
            mclk,
            self.sda.into_pad(port),
            self.scl.into_pad(port),
        )
    }
}

/// Keypad pins
pub struct Keypad {
    pub col0: Pa14<Input<Floating>>,
    pub col1: Pa15<Input<Floating>>,
    pub col2: Pa16<Input<Floating>>,
    pub col3: Pa17<Input<Floating>>,
    pub col4: Pa20<Input<Floating>>,
    pub col5: Pa21<Input<Floating>>,
    pub col6: Pa22<Input<Floating>>,
    pub col7: Pa23<Input<Floating>>,
    pub row0: Pa18<Input<Floating>>,
    pub row1: Pa19<Input<Floating>>,
    pub row2: Pb22<Input<Floating>>,
    pub row3: Pb23<Input<Floating>>,
}

/// NeoPixel pin
pub type NeoPixel = Pa27<Input<Floating>>;
