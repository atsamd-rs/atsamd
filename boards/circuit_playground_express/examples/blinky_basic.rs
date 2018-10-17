#![feature(used)]
#![no_std]
#![no_main]

extern crate circuit_playground_express as hal;
extern crate panic_abort;

#[macro_use(entry)]
extern crate cortex_m_rt;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::{CorePeripherals, Peripherals};

entry!(main);

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
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        delay.delay_ms(200u8);
        red_led.set_high();
        delay.delay_ms(200u8);
        red_led.set_low();
    }
}
