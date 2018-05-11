#![no_std]

extern crate atsamd21_hal as hal;

pub use hal::atsamd21e18a::*;
use hal::prelude::*;
pub use hal::*;

use gpio::{Floating, Input, Port};

/// Maps the pins to their arduino names and
/// the numbers printed on the board.
pub struct Pins {
    /// Opaque port reference
    pub port: Port,

    pub d0: gpio::Pa4<Input<Floating>>,
    pub d1: gpio::Pa2<Input<Floating>>,
    pub d2: gpio::Pa5<Input<Floating>>,

    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pub d13: gpio::Pa23<Input<Floating>>,
    pub mosi: gpio::Pa0<Input<Floating>>,
    pub sck: gpio::Pa1<Input<Floating>>,
}

/// Returns the pins for the device
pub fn pins(port: atsamd21e18a::PORT) -> Pins {
    let pins = port.split();
    Pins {
        port: pins.port,
        d0: pins.pa4,
        d1: pins.pa2,
        d2: pins.pa5,
        d13: pins.pa23,
        sck: pins.pa1,
        mosi: pins.pa0,
    }
}
