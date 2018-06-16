#![no_std]

extern crate atsamd21_hal as hal;

pub mod prelude;

pub use hal::atsamd21g18a::*;

use hal::prelude::*;
pub use hal::*;
pub use hal::usb as usb;

use gpio::{Floating, Input, Port};

/// Maps the pins to their arduino names and
/// the numbers printed on the board.
pub struct Pins {
    /// Opaque port reference
    pub port: Port,

    pub tx: gpio::Pa10<Input<Floating>>,
    pub rx: gpio::Pa11<Input<Floating>>,

    pub aref: gpio::Pa3<Input<Floating>>,
    
    pub d2: gpio::Pa14<Input<Floating>>,
    pub d3: gpio::Pa9<Input<Floating>>,
    pub d4: gpio::Pa8<Input<Floating>>,
    pub d5: gpio::Pa15<Input<Floating>>,
    pub d6: gpio::Pa20<Input<Floating>>,
    pub d7: gpio::Pa21<Input<Floating>>,
    pub d9: gpio::Pa7<Input<Floating>>,
    pub d10: gpio::Pa18<Input<Floating>>,
    pub d11: gpio::Pa16<Input<Floating>>,
    pub d12: gpio::Pa19<Input<Floating>>,

    pub a3: gpio::Pa4<Input<Floating>>,
    pub a2: gpio::Pb9<Input<Floating>>,
    pub a1: gpio::Pb8<Input<Floating>>,
    pub a0: gpio::Pa2<Input<Floating>>,

    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pub led: gpio::Pa17<Input<Floating>>,
    pub tx_led: gpio::Pa27<Input<Floating>>,
    pub rx_led: gpio::Pb3<Input<Floating>>,

    pub dm: gpio::Pa24<Input<Floating>>,
    pub dp: gpio::Pa25<Input<Floating>>,
}

pub fn pins(port: atsamd21g18a::PORT) -> Pins {
    let pins = port.split();

    Pins {
        port: pins.port,
        rx: pins.pa11,
        tx: pins.pa10,

        aref: pins.pa3,
        d2: pins.pa14,
        d3: pins.pa9,
        d4: pins.pa8,
        d5: pins.pa15,
        d6: pins.pa20,
        d7: pins.pa21,
        d9: pins.pa7,
        d10: pins.pa18,
        d11: pins.pa16,
        d12: pins.pa19,
        a3: pins.pa4,
        a2: pins.pb9,
        a1: pins.pb8,
        a0: pins.pa2,

        led: pins.pa17,
        tx_led: pins.pa27,
        rx_led: pins.pb3,

        dm: pins.pa24,
        dp: pins.pa25,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
