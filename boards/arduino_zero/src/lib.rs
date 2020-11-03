#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

#[cfg(feature = "panic_halt")]
pub extern crate panic_halt;

#[cfg(feature = "usb")]
use hal::clock::GenericClockController;
#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

use hal::prelude::*;
use hal::*;

pub use hal::common::*;
pub use hal::samd21::*;
pub use hal::target_device as pac;

use gpio::{Floating, Input, Port};

// The docs could be further improved with details of the specific channels etc
define_pins!(
    /// Maps the pins to their arduino names and the numbers printed on the board.
    /// Information from: <https://github.com/arduino/ArduinoCore-samd/blob/master/variants/mkrzero/variant.cpp>
    struct Pins,
    target_device: target_device,

    /// Digital 2: TC
    pin d2 = a14,

    /// Digital 3: PWM, TCC
    pin d3 = a9,

    /// Digital 4: PWM
    pin d4 = a8,

    /// Digital 5: PWM
    pin d5 = a15,

    /// Digital 6: PWM
    pin d6 = a20,

    /// Digital 7: PWM, TCC
    pin d7 = a21,

    /// Digital 8: PWM
    pin d8 = a6,

    /// Digital 9: PWM
    pin d9 = a7,

    /// Digital 9: PWM
    pin d10 = a18,

    /// SPI MOSI: PWM, TCC
    pin mosi = a16,

    /// SPI SCK
    pin sck = a17,

    /// SPI MISO: PWM, TC
    pin miso = a19,

    /// SDA
    pin sda = a22,

    /// SCL
    pin scl = a23,

    /// RX
    pin rx = a11,

    /// TX
    pin tx = a10,

    /// Analog 0: DAC
    pin a0 = a2,

    /// Analog 1
    pin a1 = b8,

    /// Analog 2
    pin a2 = b9,

    /// Analog 3: PWM, TCC
    pin a3 = a4,

    /// Analog 4: PWM, TCC
    pin a4 = a5,

    /// Analog 5
    pin a5 = b2,

    pin usb_n = a24,
    pin usb_p = a25,
    pin usb_id = a18,
    pin aref = a3,
    pin sd_sck = a12,
    pin sd_mosi = a13,
    pin sd_ss = a14,
    pin sd_miso = a15,
    pin sd_cd = a27,

    /// LED built into the board
    pin led_builtin = a17,
    pin tx_led = a27,
    pin rx_led = b3,

    pin xin32 = a0,
    pin xout32 = a1,
);

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: gpio::Pa24<Input<Floating>>,
    dp: gpio::Pa25<Input<Floating>>,
    port: &mut Port,
) -> UsbBusAllocator<UsbBus> {
    use gpio::IntoFunction;

    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();

    UsbBusAllocator::new(UsbBus::new(
        usb_clock,
        pm,
        dm.into_function(port),
        dp.into_function(port),
        usb,
    ))
}
