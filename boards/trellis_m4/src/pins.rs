//! NeoTrellis M4 Express pins

use gpio::{Floating, Input, Port};
use hal::gpio::*;

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
