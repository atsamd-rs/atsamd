#![no_std]
#![no_main]

// True Random Number Generator - trng

use bsp::hal;
use feather_m4 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

use hal::trng::Trng;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    // We will use the red led and a delay in this simplest possible
    // demonstration of the random number generator.
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led = pins.d13.into_push_pull_output();
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);

    // Create a struct as a representation of the random number generator peripheral
    let trng = Trng::new(&mut peripherals.mclk, peripherals.trng);

    // Simple loop that blinks the red led with random on and off times that are
    // sourced from the random number generator.
    loop {
        red_led.set_high().unwrap();
        delay.delay_ms(trng.random_u8());
        red_led.set_low().unwrap();
        delay.delay_ms(trng.random_u8());
    }
}
