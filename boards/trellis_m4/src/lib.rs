#![no_std]
#![recursion_limit = "1024"]
#![allow(deprecated)]

pub mod pins;

#[cfg(feature = "adxl343")]
pub use adxl343;

#[cfg(feature = "keypad-unproven")]
pub use keypad;

pub use atsamd_hal as hal;

pub use hal::common::*;
pub use hal::pac;
pub use hal::samd51::*;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;
pub use pins::Pins;

#[cfg(feature = "keypad-unproven")]
use gpio::Input;
use gpio::Port;
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster4, UART4};
use hal::time::Hertz;

#[cfg(feature = "keypad-unproven")]
use hal::ehal::digital::v1_compat::{OldInputPin, OldOutputPin};
#[cfg(feature = "keypad-unproven")]
use hal::gpio::{OpenDrain, Output, PullUp};
#[cfg(feature = "keypad-unproven")]
use keypad::{keypad_new, keypad_struct};

/// Number of Neopixels on the device
pub const NEOPIXEL_COUNT: usize = 32;

#[cfg(feature = "keypad-unproven")]
keypad_struct! {
    #[doc="NeoTrellis M4 Express 8x4 keypad"]
    pub struct Keypad {
        rows: (
            OldInputPin<gpio::Pa18<Input<PullUp>>>,
            OldInputPin<gpio::Pa19<Input<PullUp>>>,
            OldInputPin<gpio::Pb22<Input<PullUp>>>,
            OldInputPin<gpio::Pb23<Input<PullUp>>>,
        ),
        columns: (
            OldOutputPin<gpio::Pa14<Output<OpenDrain>>>,
            OldOutputPin<gpio::Pa15<Output<OpenDrain>>>,
            OldOutputPin<gpio::Pa16<Output<OpenDrain>>>,
            OldOutputPin<gpio::Pa17<Output<OpenDrain>>>,
            OldOutputPin<gpio::Pa20<Output<OpenDrain>>>,
            OldOutputPin<gpio::Pa21<Output<OpenDrain>>>,
            OldOutputPin<gpio::Pa22<Output<OpenDrain>>>,
            OldOutputPin<gpio::Pa23<Output<OpenDrain>>>,
        ),
    }
}

#[cfg(feature = "keypad-unproven")]
impl Keypad {
    /// Create a new Keypad from the given pins
    pub fn new(pins: pins::Keypad, port: &mut Port) -> Self {
        keypad_new!(Self {
            rows: (
                pins.row0.into_pull_up_input(port).into(),
                pins.row1.into_pull_up_input(port).into(),
                pins.row2.into_pull_up_input(port).into(),
                pins.row3.into_pull_up_input(port).into(),
            ),
            columns: (
                pins.col0.into_open_drain_output(port).into(),
                pins.col1.into_open_drain_output(port).into(),
                pins.col2.into_open_drain_output(port).into(),
                pins.col3.into_open_drain_output(port).into(),
                pins.col4.into_open_drain_output(port).into(),
                pins.col5.into_open_drain_output(port).into(),
                pins.col6.into_open_drain_output(port).into(),
                pins.col7.into_open_drain_output(port).into(),
            ),
        })
    }
}

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    pins: pins::STEMMA,
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom4: pac::SERCOM4,
    mclk: &mut pac::MCLK,
    port: &mut Port,
) -> I2CMaster4<
    hal::sercom::Sercom4Pad0<gpio::Pb8<gpio::PfD>>,
    hal::sercom::Sercom4Pad1<gpio::Pb9<gpio::PfD>>,
> {
    pins.i2c_master(clocks, bus_speed, sercom4, mclk, port)
}

/// Convenience for setting up the labelled SDA, SCL pins in the
/// STEMMA JST connector to operate as a UART device at the specified
/// baud rate.
///
/// Here SCL is the RX pin and SDA is the TX pin.
pub fn uart<F: Into<Hertz>>(
    pins: pins::STEMMA,
    clocks: &mut GenericClockController,
    baud: F,
    sercom4: pac::SERCOM4,
    mclk: &mut pac::MCLK,
    port: &mut Port,
) -> UART4<
    hal::sercom::Sercom4Pad1<gpio::Pb9<gpio::PfD>>,
    hal::sercom::Sercom4Pad0<gpio::Pb8<gpio::PfD>>,
    (),
    (),
> {
    pins.uart(clocks, baud, sercom4, mclk, port)
}
