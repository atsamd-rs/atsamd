#![no_std]

extern crate atsamd21_hal as hal;

pub mod prelude;

pub use hal::atsamd21g18a::*;

use hal::prelude::*;
pub use hal::usb;
pub use hal::*;

use gpio::{Floating, Input, Port};

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: atsamd21g18a,

    pin tx = pa10: Pa10,
    pin rx = pa11: Pa11,

    pin aref = pa3: Pa3,

    pin d2 = pa14: Pa14,
    pin d3 = pa9: Pa9,
    pin d4 = pa8: Pa8,
    pin d5 = pa15: Pa15,
    pin d6 = pa20: Pa20,
    pin d7 = pa21: Pa21,
    pin d8 = pa6: Pa6,
    pin d9 = pa7: Pa7,
    pin d10 = pa18: Pa18,
    pin d11 = pa16: Pa16,
    pin d12 = pa19: Pa19,

    pin a3 = pa4: Pa4,
    pin a2 = pb9: Pb9,
    pin a1 = pb8: Pb8,
    pin a0 = pa2: Pa2,

    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin led = pa17: Pa17,
    pin tx_led = pa27: Pa27,
    pin rx_led = pb3: Pb3,

    pin dm = pa24: Pa24,
    pin dp = pa25: Pa25,
);
