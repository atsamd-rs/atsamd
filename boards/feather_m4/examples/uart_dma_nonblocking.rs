//! This example shows how to use the UART to perform non-blocking transfers
//! using DMA.

#![no_std]
#![no_main]

use cortex_m::asm;
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

    // Setup UART peripheral. We shouldn't attach DMA channels to the UART; they
    // instead must be passed to `send_with_dma` and `receive_with_dma`.
    let uart = bsp::uart(
        &mut clocks,
        9600.Hz(),
        uart_sercom,
        &mut peripherals.mclk,
        uart_rx,
        uart_tx,
    );

    // Split uart in rx + tx halves
    let (mut rx, tx) = uart.split();

    // Get a 50 byte buffer to store data to send/receive
    const LENGTH: usize = 50;
    let rx_buffer: &'static mut [u8; LENGTH] =
        cortex_m::singleton!(: [u8; LENGTH] = [0x00; LENGTH]).unwrap();
    let tx_buffer: &'static mut [u8; LENGTH] =
        cortex_m::singleton!(: [u8; LENGTH] = [0x00; LENGTH]).unwrap();

    // For fun, store numbers from 0 to 49 in buffer
    for (i, c) in tx_buffer.iter_mut().enumerate() {
        *c = i as u8;
    }

    //...and receive (blocking) using DMA
    rx.flush_rx_buffer();

    // Let's receive AND send data at the same time with DMA. Note that these
    // transfers require static buffers

    // Setup a DMA transfer to send our data asynchronously.
    // We'll set the waker to be a no-op
    let tx_dma = tx.send_with_dma(tx_buffer, chan0);

    // Setup a DMA transfer to receive our data asynchronously.
    // Again, we'll set the waker to be a no-op
    let rx_dma = rx.receive_with_dma(rx_buffer, chan1);

    // Wait for transmit DMA transfer to complete
    let (_chan0, _tx_buffer, _tx) = tx_dma.wait();

    // Wait for receive DMA transfer to complete
    let (_chan1, _rx_buffer, _rx) = rx_dma.wait();

    loop {
        // Go to sleep
        asm::wfi();
    }
}
