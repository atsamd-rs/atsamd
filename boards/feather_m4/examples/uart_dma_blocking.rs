//! This example shows how to use the UART to perform blocking transfers using
//! DMA and the embedded-io traits.

#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use bsp::pac;
use feather_m4 as bsp;

use bsp::{entry, periph_alias, pin_alias};
use hal::clock::GenericClockController;
use hal::dmac::{DmaController, PriorityLevel};
use hal::embedded_io::{Read, Write};
use hal::fugit::RateExtU32;

use pac::Peripherals;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );

    let mut pm = peripherals.pm;
    let dmac = peripherals.dmac;
    let pins = bsp::Pins::new(peripherals.port);

    // Setup DMA channels for later use
    let mut dmac = DmaController::init(dmac, &mut pm);
    let channels = dmac.split();

    let chan0 = channels.0.init(PriorityLevel::Lvl0);
    let chan1 = channels.1.init(PriorityLevel::Lvl0);

    // Take peripheral and pins
    let uart_sercom = periph_alias!(peripherals.uart_sercom);
    let uart_rx = pin_alias!(pins.uart_rx);
    let uart_tx = pin_alias!(pins.uart_tx);

    // Setup UART peripheral and attach DMA channels
    let uart = bsp::uart(
        &mut clocks,
        9600.Hz(),
        uart_sercom,
        &mut peripherals.mclk,
        uart_rx,
        uart_tx,
    )
    .with_rx_channel(chan0)
    .with_tx_channel(chan1);

    // Split uart in rx + tx halves
    let (mut rx, mut tx) = uart.split();

    loop {
        // Make buffers to store data to send/receive
        let mut rx_buffer = [0x00; 50];
        let mut tx_buffer = [0x00; 50];

        // For fun, store numbers from 0 to 49 in buffer
        for (i, c) in tx_buffer.iter_mut().enumerate() {
            *c = i as u8;
        }

        // Send data using DMA...
        tx.write(&tx_buffer).unwrap();

        //...and receive using DMA
        rx.flush_rx_buffer();
        rx.read(&mut rx_buffer).unwrap();
    }
}
