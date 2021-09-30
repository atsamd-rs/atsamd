#![no_std]
#![no_main]

use arduino_nano33iot as bsp;
use bsp::hal;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
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
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut uart = bsp::uart(
        &mut clocks,
        9600.hz(),
        peripherals.SERCOM5,
        &mut peripherals.PM,
        pins.rx,
        pins.tx,
    );

    loop {
        // print ASCII characters from ! to ~
        for ch in 33..127 {
            uart.write(ch).unwrap();
            delay.delay_ms(500u16);
        }
    }
}
