#![no_std]
#![recursion_limit = "1024"]

pub mod pins;

#[cfg(feature = "adxl343")]
pub use adxl343;

#[cfg(feature = "keypad-unproven")]
pub use keypad;

use atsamd_hal as hal;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;
pub use hal::{target_device::*, *};
pub use pins::Pins;

use gpio::Port;
#[cfg(feature = "keypad-unproven")]
use gpio::Input;
use hal::clock::GenericClockController;
use hal::sercom::{I2CMaster4, UART4};
use hal::time::Hertz;

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
            gpio::Pa18<Input<PullUp>>,
            gpio::Pa19<Input<PullUp>>,
            gpio::Pb22<Input<PullUp>>,
            gpio::Pb23<Input<PullUp>>,
        ),
        columns: (
            gpio::Pa14<Output<OpenDrain>>,
            gpio::Pa15<Output<OpenDrain>>,
            gpio::Pa16<Output<OpenDrain>>,
            gpio::Pa17<Output<OpenDrain>>,
            gpio::Pa20<Output<OpenDrain>>,
            gpio::Pa21<Output<OpenDrain>>,
            gpio::Pa22<Output<OpenDrain>>,
            gpio::Pa23<Output<OpenDrain>>,
        ),
    }
}

#[cfg(feature = "keypad-unproven")]
impl Keypad {
    /// Create a new Keypad from the given pins
    pub fn new(pins: pins::Keypad, port: &mut Port) -> Self {
        keypad_new!(Self {
            rows: (
                pins.row0.into_pull_up_input(port),
                pins.row1.into_pull_up_input(port),
                pins.row2.into_pull_up_input(port),
                pins.row3.into_pull_up_input(port),
            ),
            columns: (
                pins.col0.into_open_drain_output(port),
                pins.col1.into_open_drain_output(port),
                pins.col2.into_open_drain_output(port),
                pins.col3.into_open_drain_output(port),
                pins.col4.into_open_drain_output(port),
                pins.col5.into_open_drain_output(port),
                pins.col6.into_open_drain_output(port),
                pins.col7.into_open_drain_output(port),
            ),
        })
    }
}

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    pins: pins::JST,
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom4: SERCOM4,
    mclk: &mut MCLK,
    port: &mut Port,
) -> I2CMaster4<
    hal::sercom::Sercom4Pad0<gpio::Pb8<gpio::PfD>>,
    hal::sercom::Sercom4Pad1<gpio::Pb9<gpio::PfD>>,
> {
    pins.i2c_master(clocks, bus_speed, sercom4, mclk, port)
}

/// Convenience for setting up the labelled SDA, SCL pins in the
/// JST connector to operate as a UART device at the specified baud rate.
///
/// Here SCL is the RX pin and SDA is the TX pin.
pub fn uart<F: Into<Hertz>>(
    pins: pins::JST,
    clocks: &mut GenericClockController,
    baud: F,
    sercom4: SERCOM4,
    mclk: &mut MCLK,
    nvic: &mut NVIC,
    port: &mut Port,
) -> UART4<
    hal::sercom::Sercom4Pad1<gpio::Pb9<gpio::PfD>>,
    hal::sercom::Sercom4Pad0<gpio::Pb8<gpio::PfD>>,
    (),
    (),
> {
    pins.uart(clocks, baud, sercom4, mclk, nvic, port)
}
