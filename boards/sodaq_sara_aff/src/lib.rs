#![no_std]

pub use atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

pub use hal::common::*;

pub use hal::pac;

use gpio::{Floating, Input, PfD, Port};

use hal::clock::GenericClockController;
use hal::sercom::{PadPin, UART5};
use hal::time::Hertz;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    pac: pac,

    /// arduino header digital pins
    /// rx/ d0
    pin rx = b31,
    /// tx / d1
    pin tx = b30,
    /// dac
    pin d2 = a2,
    /// scom1pad3
    pin d3 = a19,
    /// ain14
    pin d5 = b6,
    /// ain15
    pin d6 = b7,
    /// ain2
    pin d7 = b8,
    /// scom4pad2
    pin d8 = b10,
    /// scom4pad3
    pin d9 = b11,

    /// SPI
    pin ss = a23,
    pin mosi = a20,
    pin miso = a22,

    /// led_builtin + SCK
    pin d13 = a21,

    /// arduino header analog pins
    pin a0 = b0,
    pin a1 = b1,
    pin a2 = b3,
    pin a3 = b4,

    /// grove1
    pin d14 = a10,
    /// grove2
    pin d15 = a11,

    /// leds
    pin led_red  = a12,
    pin led_green = b15,
    pin led_blue = a13,

    /// accelelero & magneto
    pin acc_int1 = a14,
    pin acc_int2 = a15,
    pin mag_int = a18,

    /// usb_detect
    pin usb_det = b16,
    /// chg_stat
    pin chg_stat = a4,

    /// gps
    pin gps_tp = a7,
    pin gps_en = a28,

    // u-blox sara
    pin sara_enable = a27,
    pin sara_reset = b14,
    pin sara_tx_enable = b13,
    pin sara_status = b22,
    pin sara_tx = a6,
    pin sara_rx = a5,

    /// i2c
    pin sda = a16,
    pin scl = a17,
    pin sda1 = a8,
    pin scl1 = a9,
);

/// Convenience for setting up the serial communication pins
/// operate as UART RX/TX (respectively) running at the specified baud.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom5: pac::SERCOM5,
    pm: &mut pac::PM,
    rx: gpio::Pb31<Input<Floating>>,
    tx: gpio::Pb30<Input<Floating>>,
    port: &mut Port,
) -> UART5<
    hal::sercom::Sercom5Pad1<gpio::Pb31<PfD>>,
    hal::sercom::Sercom5Pad0<gpio::Pb30<PfD>>,
    (),
    (),
> {
    let gclk0 = clocks.gclk0();

    UART5::new(
        &clocks.sercom5_core(&gclk0).unwrap(),
        baud.into(),
        sercom5,
        pm,
        (rx.into_pad(port), tx.into_pad(port)),
    )
}
