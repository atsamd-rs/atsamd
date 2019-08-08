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

    /// Arduino header digital pins
    /// RX
    pin d0 = 	b31,

    /// TX
    pin d1 = 	b30,

    /// DAC
    pin d2 = 	a2,

    /// SCOM1PAD3
    pin d3 = 	a19,

    /// AIN12
    pin d4 = 	b4,

    /// AIN14
    pin d5 = 	b6,

    /// AIN15
    pin d6 = 	b7,

    /// AIN2
    pin d7 = 	b8,

    /// SCOM4PAD2
    pin d8 = 	b10,

    /// SCOM4PAD3
    pin d9 = 	b11,

    /// SS
    pin d10 =	a23,

    /// MOSI
    pin d11 =	a20,

    /// MISO
    pin d12 =	a22,

    /// LED_BUILTIN / SCK   
    pin d13 =	a21,

    /// Arduino header analog pins
    /// A0  
    pin a0 =	b0,

    /// A1  
    pin a1 =	b1,

    /// A2  
    pin a2 =	b3,

    /// A3  
    pin a3 =	b4,

    /// A4  / SDA(I2C1)
    pin a4 =	a8,

    /// A5 / SCL(I2C1) 
    pin a5 =	a9,

    /// Other pins
    /// GROVE1
    pin d14 = 	a10,

    /// GROVE2
    pin d15 = 	a11,

    /// LED_RED 
    pin d16 = 	a12,

    /// LED_GREEN
    pin d17 =	b15,

    /// LED_BLUE
    pin d18 =   a13,

    /// ACCEL_INT1
    pin d19 =	a14,

    /// ACCEL_INT2
    pin d20 =	a15,

    /// MAG_INT
    pin d21 =	a18,

    /// USB_DETECT
    pin d22 =	b16,

    /// SARA_STATUS
    pin d23 =	b22,

    /// CHG_STAT
    pin d24 =	a4

    /// GPS_TIMEPULSE
    pin d25 =	a7,

    /// GPS_ENABLE
    pin d26 =	a28,

    /// SARA_ENABLE 
    pin d27 =	a27,

    /// SARA_RESET
    pin d28 =	b14,

    /// SARA_TX_ENABLE
    pin d29 =	b13,
);

/// Convenience for setting up the D2 and D0 pins to
/// operate as UART RX/TX (respectively) running at the specified baud.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom0: SERCOM0,
    nvic: &mut NVIC,
    pm: &mut PM,
    d2: gpio::Pa5<Input<Floating>>,
    d0: gpio::Pa4<Input<Floating>>,
    port: &mut Port,
) -> UART0<hal::sercom::Sercom0Pad1<gpio::Pa5<PfD>>, hal::sercom::Sercom0Pad0<gpio::Pa4<PfD>>, (), ()>
{
    let gclk0 = clocks.gclk0();

    UART0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        baud.into(),
        sercom0,
        nvic,
        pm,
        (d2.into_pad(port), d0.into_pad(port)),
    )
}
