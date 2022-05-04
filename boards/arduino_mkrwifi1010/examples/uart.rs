#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use arduino_mkrwifi1010 as bsp;
use bsp::hal;
use bsp::pac;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::prelude::*;

use pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);

    let mut red_led = pins.led.into_push_pull_output();

    // Setup UART peripheral.
    let mut uart = bsp::uart(
        &mut clocks,
        9600.hz(),
        peripherals.SERCOM5,
        &mut peripherals.PM,
        pins.rx,
        pins.tx,
    );

    // Write out a message on start up.
    for byte in b"Hello world!\r\n" {
        nb::block!(uart.write(*byte)).unwrap();
    }

    loop {
        match uart.read() {
            Ok(byte) => {
                // Echo all received characters.
                nb::block!(uart.write(byte)).unwrap();

                // Blink the red led to show that a character has arrived.
                red_led.set_high().unwrap();
                delay.delay_ms(2u16);
                red_led.set_low().unwrap();
            }
            Err(_) => delay.delay_ms(5u16),
        };
    }
}
