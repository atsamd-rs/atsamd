#![no_std]
#![no_main]

extern crate arduino_nano33iot as hal;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

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
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut uart = hal::uart(
        &mut clocks,
        9600u32.Hz(),
        peripherals.SERCOM5,
        &mut peripherals.PM,
        pins.rx,
        pins.tx,
        &mut pins.port,
    );

    loop {
        // print ASCII characters from ! to ~
        for ch in 33..127 {
            uart.write(ch).unwrap();
            delay.delay_ms(500u16);
        }
    }
}
