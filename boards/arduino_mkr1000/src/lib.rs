#![no_std]

pub use atsamd_hal as hal;
pub use hal::common::*;
pub use hal::pac;

#[cfg(feature = "rt")]
use cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

use hal::clock::GenericClockController;
use hal::time::Hertz;

use hal::sercom::{PadPin, UART5};

use gpio::{Floating, Input, PfD, Port};

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    pac: pac,

    /// RX
    pin rx = b23,

    /// TX
    pin tx = b22,

    /// Digital 0
    pin d0 = a22,

    /// Digital 1
    pin d1 = a23,

    /// Digital 2: ADC
    pin d2 = a10,

    /// Digital 3: ADC
    pin d3 = a11,

    /// Digital 4
    pin d4 = b10,

    /// Digital 5
    pin d5 = b11,

    /// Digital 6: LED_BUILTIN
    pin d6 = a20,

    /// Digital 7
    pin d7 = a21,

    /// Digital 8/SC1 MOSI
    pin mosi = a16,

    /// Digital 9/SC1 SCK
    pin sck = a17,

    /// Digital 10/SC1 MISO
    pin miso = a19,

    /// Digital 11/SC2 SDA
    pin sda = a8,

    /// Digital 12/SC2 SCL
    pin scl = a9,

    /// Analog 0/DAC0
    pin a0 = a2,

    /// Analog 1
    pin a1 = b2,

    /// Analog 2
    pin a2 = b3,

    /// Analog 3
    pin a3 = a4,

    /// Analog 4
    pin a4 = a5,

    /// Analog 5
    pin a5 = a6,

    /// Analog 6
    pin a6 = a7,

    pin usb_dn = a24,
    pin usb_dp = a25,
    pin usb_id = a18,

    pin aref = a3,
);

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom5: pac::SERCOM5,
    pm: &mut pac::PM,
    d13: gpio::Pb23<Input<Floating>>, //rx
    d14: gpio::Pb22<Input<Floating>>, //tx
    port: &mut Port,
) -> UART5<
    hal::sercom::Sercom5Pad3<gpio::Pb23<PfD>>, //rx
    hal::sercom::Sercom5Pad2<gpio::Pb22<PfD>>, //tx
    (),
    (),
> {
    let gclk0 = clocks.gclk0();

    UART5::new(
        &clocks.sercom5_core(&gclk0).unwrap(),
        baud.into(),
        sercom5,
        pm,
        (d13.into_pad(port), d14.into_pad(port)),
    )
}
