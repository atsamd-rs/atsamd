#![no_std]

extern crate atsamd21_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

#[cfg(feature = "panic_abort")]
pub extern crate panic_abort;

pub use hal::atsamd21g18a::*;
use hal::prelude::*;
pub use hal::*;

use gpio::{Floating, Input, Port};

// This could possibly be simplified with concat_ident p for the name and P for
// the type but I couldn't get it to work. Meta attribues (i.e. doc comments)
// are optional.
macro_rules! input_pins {
    // Insert trailing comma if it's missing
    ($( $(#[$attr:meta])* pin $name:ident = $pin_name:ident: $pin_type:ident),+ ) =>
        (input_pins!($( $(#[$attr])* pin $name = $pin_name: $pin_type),+ , ););

    ($( $(#[$attr:meta])* pin $name:ident = $pin_name:ident: $pin_type:ident),+ , ) => {
        /// Maps the pins to their arduino names and the numbers printed on the board.
        /// Information from: https://github.com/arduino/ArduinoCore-samd/blob/master/variants/mkrzero/variant.cpp
        pub struct Pins {
            /// Opaque port reference
            pub port: Port,
            $($(#[$attr])* pub $name: gpio::$pin_type<Input<Floating>>),+
        }

        pub fn pins(port: atsamd21g18a::PORT) -> Pins {
            let pins = port.split();
            Pins {
                port: pins.port,
                $($name: pins.$pin_name),+
            }
        }
    }
}

// The docs could be further improved with details of the specific channels etc
input_pins!(
    #[doc = "Digital 0: PWM, TC"]
    pin d0 = pa22: Pa22,

    #[doc = "Digital 1: PWM, TC"]
    pin d1 = pa23: Pa23,

    #[doc = "Digital 2: PWM, TCC, ADC"]
    pin d2 = pa10: Pa10,

    #[doc = "Digital 3: PWM, TCC, ADC"]
    pin d3 = pa11: Pa11,

    #[doc = "Digital 4: PWM, TC"]
    pin d4 = pb10: Pb10,

    #[doc = "Digital 5: PWM, TC"]
    pin d5 = pb11: Pb11,

    #[doc = "Digital 6: PWM, TCC"]
    pin d6 = pa20: Pa20,

    #[doc = "Digital 7: PWM, TCC"]
    pin d7 = pa21: Pa21,

    #[doc = "SPI MOSI: PWM, TCC"]
    pin mosi = pa16: Pa16,

    #[doc = "SPI SCK"]
    pin sck = pa17: Pa17,

    #[doc = "SPI MISO: PWM, TC"]
    pin miso = pa19: Pa19,

    #[doc = "SDA"]
    pin sda = pa8: Pa8,

    #[doc = "SCL"]
    pin scl = pa9: Pa9,

    #[doc = "RX"]
    pin rx = pb23: Pb23,

    #[doc = "TX"]
    pin tx = pb22: Pb22,

    #[doc = "Analog 0: DAC"]
    pin a0 = pa2: Pa2,

    #[doc = "Analog 1"]
    pin a1 = pb2: Pb2,

    #[doc = "Analog 2"]
    pin a2 = pb3: Pb3,

    #[doc = "Analog 3: PWM, TCC"]
    pin a3 = pa4: Pa4,

    #[doc = "Analog 4: PWM, TCC"]
    pin a4 = pa5: Pa5,

    #[doc = "Analog 5"]
    pin a5 = pa6: Pa6,

    #[doc = "Analog 6"]
    pin a6 = pa7: Pa7,

    pin usb_n = pa24: Pa24,
    pin usb_p = pa25: Pa25,
    pin usb_id = pa18: Pa18,
    pin aref = pa3: Pa3,
    pin sd_sck = pa12: Pa12,
    pin sd_mosi = pa13: Pa13,
    pin sd_ss = pa14: Pa14,
    pin sd_miso = pa15: Pa15,
    pin sd_cd = pa27: Pa27,

    #[doc = "LED built into the board"]
    pin led_builtin = pb8: Pb8,

    pin bottom_pad = pa28: Pa28,
    pin adc_battery = pb9: Pb9,
    pin xin32 = pa0: Pa0,
    pin xout32 = pa1: Pa1,
);
