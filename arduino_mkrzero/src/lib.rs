#![no_std]
#![recursion_limit="1024"]

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

#[macro_use]
extern crate mashup;

use gpio::{Floating, Input, Port};

macro_rules! define_pins2 {
    ($(#[$topattr:meta])* struct $Type:ident,
     target_device: $target_device:ident,
     $( $(#[$attr:meta])* pin $name:ident = $pin_iden:tt),+ , ) => {

mashup!{
    $(m["gen_pin_name" $pin_iden] = p $pin_iden;),+
    $(m["gen_type_name" $pin_iden] = P $pin_iden;),+
}
m!{

$(#[$topattr])*
pub struct $Type {
    /// Opaque port reference
    pub port: Port,

    $($(#[$attr])* pub $name: gpio::"gen_type_name" $pin_iden),+
}

impl Pins {
    /// Returns the pins for the device
    pub fn new(port: $target_device::PORT) -> Self {
        let pins = port.split();
        Pins {
            port: pins.port,
            $($name: port."gen_pin_name" $pin_iden),+
        }
    }
}

}}}




// The docs could be further improved with details of the specific channels etc
define_pins2!(
    /// Maps the pins to their arduino names and the numbers printed on the board.
    /// Information from: <https://github.com/arduino/ArduinoCore-samd/blob/master/variants/mkrzero/variant.cpp>
    struct Pins,
    target_device: atsamd21g18a,

    /// Digital 0: PWM, TC
    pin d0 = a22,
//
//    /// Digital 1: PWM, TC
//    pin d1 = pa23: Pa23,
//
//    /// Digital 2: PWM, TCC, ADC
//    pin d2 = pa10: Pa10,
//
//    /// Digital 3: PWM, TCC, ADC
//    pin d3 = pa11: Pa11,
//
//    /// Digital 4: PWM, TC
//    pin d4 = pb10: Pb10,
//
//    /// Digital 5: PWM, TC
//    pin d5 = pb11: Pb11,
//
//    /// Digital 6: PWM, TCC
//    pin d6 = pa20: Pa20,
//
//    /// Digital 7: PWM, TCC
//    pin d7 = pa21: Pa21,
//
//    /// SPI MOSI: PWM, TCC
//    pin mosi = pa16: Pa16,
//
//    /// SPI SCK
//    pin sck = pa17: Pa17,
//
//    /// SPI MISO: PWM, TC
//    pin miso = pa19: Pa19,
//
//    /// SDA
//    pin sda = pa8: Pa8,
//
//    /// SCL
//    pin scl = pa9: Pa9,
//
//    /// RX
//    pin rx = pb23: Pb23,
//
//    /// TX
//    pin tx = pb22: Pb22,
//
//    /// Analog 0: DAC
//    pin a0 = pa2: Pa2,
//
//    /// Analog 1
//    pin a1 = pb2: Pb2,
//
//    /// Analog 2
//    pin a2 = pb3: Pb3,
//
//    /// Analog 3: PWM, TCC
//    pin a3 = pa4: Pa4,
//
//    /// Analog 4: PWM, TCC
//    pin a4 = pa5: Pa5,
//
//    /// Analog 5
//    pin a5 = pa6: Pa6,
//
//    /// Analog 6
//    pin a6 = pa7: Pa7,
//
//    pin usb_n = pa24: Pa24,
//    pin usb_p = pa25: Pa25,
//    pin usb_id = pa18: Pa18,
//    pin aref = pa3: Pa3,
//    pin sd_sck = pa12: Pa12,
//    pin sd_mosi = pa13: Pa13,
//    pin sd_ss = pa14: Pa14,
//    pin sd_miso = pa15: Pa15,
//    pin sd_cd = pa27: Pa27,
//
//    /// LED built into the board
//    pin led_builtin = pb8: Pb8,
//
//    pin bottom_pad = pa28: Pa28,
//    pin adc_battery = pb9: Pb9,
//    pin xin32 = pa0: Pa0,
//    pin xout32 = pa1: Pa1,
);
