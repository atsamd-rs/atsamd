#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use bsp::pac;
use metro_m0 as bsp;

use bsp::entry;
use hal::clock::v2 as clock;
use hal::delay::Delay;
use hal::prelude::*;
use pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let (_buses, clocks, _tokens) =
        clock::clock_system_at_reset(peripherals.gclk, peripherals.pm, peripherals.sysctrl);

    let gclk0 = clocks.gclk0;
    let (mut delay, _gclk0) = Delay::new_with_source(core.SYST, gclk0);

    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led: bsp::RedLed = pins.d13.into();
    loop {
        delay.delay_ms(200u8);
        red_led.set_high().unwrap();
        delay.delay_ms(200u8);
        red_led.set_low().unwrap();
    }
}
