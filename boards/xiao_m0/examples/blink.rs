#![no_main]
#![no_std]

extern crate panic_halt;

use hal::{clock::GenericClockController, delay::Delay, prelude::*};
use pac::{CorePeripherals, Peripherals};

use bsp::{entry, hal, pac, Led0};
use xiao_m0 as bsp;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port);

    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut led0: Led0 = pins.led0.into_push_pull_output();

    loop {
        delay.delay_ms(200u8);
        led0.toggle().unwrap();
    }
}
