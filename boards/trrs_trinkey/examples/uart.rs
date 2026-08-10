//! This example shows how to use the UART to perform transfers using the
//! embedded-hal-nb traits.
//! DISCLAIMER - The UART pinout does not follow any specific standard
//!     Further, the pinout of the MCU on the TRRS jack does not seem to work with any sort of "normal" serial trrs adapters I have seen

#![no_std]
#![no_main]

use atsamd_hal::ehal::digital::OutputPin;
use atsamd_hal::ehal::digital::StatefulOutputPin;
use atsamd_hal::fugit::Rate;
use atsamd_hal::gpio::AlternateD;
use atsamd_hal::gpio::Pin;
use atsamd_hal::gpio::PA04;
use atsamd_hal::gpio::PA06;
use atsamd_hal::sercom::uart;
use atsamd_hal::sercom::uart::BaudMode;
use atsamd_hal::sercom::uart::Oversampling;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use bsp::pac;
use hal::nb;
use trrs_trinkey as bsp;

use bsp::{entry, periph_alias};
use hal::clock::GenericClockController;
use hal::ehal_nb::serial::{Read, Write};

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

    let pm = peripherals.pm;
    let pins = bsp::Pins::new(peripherals.port);

    // Take peripheral and pins
    let uart_sercom = periph_alias!(peripherals.trrs_sercom);
    // Referenced https://raw.githubusercontent.com/adafruit/Adafruit-TRRS-Trinkey-PCB/refs/heads/main/Adafruit%20TRRS%20Trinkey%20PrettyPins.svg
    // and https://onlinedocs.microchip.com/oxy/GUID-22527069-B4D6-49B9-BACC-3AF1C52EB48C-en-US-21/GUID-DEBE7851-16DD-4D3B-8C60-F2627E3B3DEE.html
    // to make an arbitrary call on the TX/RX assignments.
    let uart_tx: Pin<PA06, AlternateD> = pins.ring_1.into_alternate();
    let uart_rx: Pin<PA04, AlternateD> = pins.ring_2.into_alternate();

    // Need to set a pin as ground
    let mut sleeve_gnd = pins.sleeve.into_push_pull_output();
    sleeve_gnd.set_low();

    let uart = {
        let gclk0 = clocks.gclk0();
        let clock = &clocks.sercom0_core(&gclk0).unwrap();
        let baud = Rate::<u32, 1, 1>::Hz(9600);
        let pads = uart::Pads::default().rx(uart_rx).tx(uart_tx);
        uart::Config::new(&pm, uart_sercom, pads, clock.freq())
            .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
            .enable()
    };

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
