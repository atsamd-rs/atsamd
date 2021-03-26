#![no_std]
#![no_main]

extern crate wio_lite_w600 as hal;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = hal::Pins::new(peripherals.PORT);
    let mut led = pins.d13.into_push_pull_output();

    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        led.set_high().unwrap();
        delay.delay_ms(100u16);
        led.set_low().unwrap();
        delay.delay_ms(100u16);
    }
}
