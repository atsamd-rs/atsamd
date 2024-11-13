//! This example shows how to use the UART to perform transfers using the
//! embedded-hal-nb traits.

#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use bsp::pac;
use feather_m0 as bsp;
use hal::nb;

use bsp::{entry, periph_alias, pin_alias};
use hal::clock::GenericClockController;
use hal::ehal_nb::serial::{Read, Write};
use hal::fugit::RateExtU32;

use pac::Peripherals;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    let mut pm = peripherals.pm;
    let pins = bsp::Pins::new(peripherals.port);

    // Take peripheral and pins
    let uart_sercom = periph_alias!(peripherals.uart_sercom);
    let uart_rx = pin_alias!(pins.uart_rx);
    let uart_tx = pin_alias!(pins.uart_tx);

    // Setup UART peripheral
    let uart = bsp::uart(
        &mut clocks,
        9600.Hz(),
        uart_sercom,
        &mut pm,
        uart_rx,
        uart_tx,
    );

    // Split uart in rx + tx halves
    let (mut rx, mut tx) = uart.split();

    // Make buffers to store data to send/receive
    let mut rx_buffer = [0x00; 50];
    let tx_buffer = b"Hello, world!";

    loop {
        // Send data. We block on each byte, but we could also perform some tasks while
        // waiting for the byte to finish sending.
        for c in tx_buffer.iter() {
            nb::block!(tx.write(*c)).unwrap();
        }

        // Receive data. We block on each byte, but we could also perform some tasks
        // while waiting for the byte to finish sending.
        rx.flush_rx_buffer();
        for c in rx_buffer.iter_mut() {
            *c = nb::block!(rx.read()).unwrap();
        }
    }
}
