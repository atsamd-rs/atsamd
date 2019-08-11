#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
pub use hal::target_device::*;
pub use hal::*;

use gpio::{Floating, Input, PfD, Port};

use hal::clock::GenericClockController;
use hal::sercom::{PadPin, UART0};
use hal::time::Hertz;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

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
    /// ss
    pin d10 = a23,
    /// mosi
    pin d11 = a20,
    /// miso
    pin d12 = a22,
    /// led_builtin 
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
    sercom0: SERCOM0,
    nvic: &mut NVIC,
    pm: &mut PM,
    sara_rx: gpio::Pa5<Input<Floating>>,
    sara_tx: gpio::Pa6<Input<Floating>>,
    port: &mut Port,
) -> UART0<hal::sercom::Sercom0Pad1<gpio::Pa5<PfD>>, hal::sercom::Sercom0Pad2<gpio::Pa6<PfD>>, (), ()>
{
    let gclk0 = clocks.gclk0();

    UART0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        baud.into(),
        sercom0,
        nvic,
        pm,
        (sara_rx.into_pad(port), sara_tx.into_pad(port)),
    )
}
