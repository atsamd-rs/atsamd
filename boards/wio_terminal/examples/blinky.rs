#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;

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
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let sets = wio::Pins::new(peripherals.PORT).split();
    let mut user_led = sets.user_led.into_push_pull_output();
    user_led.set_low().unwrap();

    loop {
        user_led.toggle().ok();
        delay.delay_ms(200u8);
    }
}
