#![no_std]
#![no_main]

//! This example shows a simple transfer operation with an slave device.
//! The ItsyBitsy will send a simple Hello World message, and the slave
//! is expected to send a response. After the transaction, the response
//! from the slave is echoed in the default UART.

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;

#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use itsybitsy_m4 as bsp;

use bsp::{
    entry,
    hal::{
        clock::GenericClockController,
        delay::Delay,
        ehal::{
            blocking::{delay::DelayMs, spi::Transfer},
            serial::Write,
        },
        pac::{CorePeripherals, Peripherals},
        prelude::*,
        time::{Hertz, MegaHertz},
    },
    spi_master,
};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut serial = bsp::uart(
        &mut clocks,
        Hertz(115200),
        peripherals.SERCOM3,
        &mut peripherals.MCLK,
        pins.d0_rx,
        pins.d1_tx,
    );
    let mut spi1 = spi_master(
        &mut clocks,
        MegaHertz(4),
        peripherals.SERCOM1,
        &mut peripherals.MCLK,
        pins.sck,
        pins.mosi,
        pins.miso,
    );
    let mut red_led = pins.d13.into_push_pull_output();
    let mut cs = pins.a2.into_push_pull_output();
    let message = b"hello world";
    loop {
        cs.set_low().unwrap();
        if let Ok(slave_msg) = spi1.transfer(&mut message.clone()) {
            cs.set_high().unwrap();
            for c in slave_msg {
                let _ = nb::block!(serial.write(*c));
            }
        }
        delay.delay_ms(200u8);
        red_led.set_high().unwrap();
        delay.delay_ms(200u8);
        red_led.set_low().unwrap();
    }
}
