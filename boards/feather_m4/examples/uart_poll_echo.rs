#![no_std]
#![no_main]

// Simple example of reading and writing to the UART
//
// Polls the UART in the main loop. Reads any bytes in
// and echoes them back out. You need to connect a
// TTL level serial port to the TX and RX pins in order
// to see the uart working.

extern crate panic_halt;
extern crate cortex_m;
extern crate feather_m4 as hal;

#[macro_use(block)]
extern crate nb;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::sercom::{PadPin, Sercom5Pad0, Sercom5Pad1, UART5};
use hal::pac::gclk::pchctrl::{GEN_A};
use hal::pac::gclk::genctrl::{SRC_A};
use hal::time::{Hertz};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL, false);
    let gclk2 = clocks.get_gclk(GEN_A::GCLK2).expect("Could not get clock 2");
    
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    
    let tx: Sercom5Pad0<_> = pins
        .d1
        .into_pull_down_input(&mut pins.port)
        .into_pad(&mut pins.port);
    let rx: Sercom5Pad1<_> = pins
        .d0
        .into_pull_down_input(&mut pins.port)
        .into_pad(&mut pins.port);
    let uart_clk = clocks
        .sercom5_core(&gclk2)
        .expect("Could not configure sercom5 clock");
    
    let mut uart = UART5::new(
        &uart_clk,
        Hertz(19200),
        peripherals.SERCOM5,
        &mut peripherals.MCLK,
        (rx, tx),
    );

    // Write out a message on start up
    for byte in b"Hello, world!" {
        block!(uart.write(*byte)).unwrap();
    }

    loop {
        match uart.read() {
            Ok(byte) => {
                block!(uart.write(byte)).unwrap();

                // Blink the red led to show that a character has arrived
                red_led.set_high().unwrap();
                delay.delay_ms(2u16);
                red_led.set_low().unwrap();
            },
            Err(_) => delay.delay_ms(5u16),
        };
    }
}
