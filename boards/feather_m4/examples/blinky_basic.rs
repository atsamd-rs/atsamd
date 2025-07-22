#![no_std]
#![no_main]

use feather_m4 as bsp;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use bsp::hal;
use hal::clock::v2::clock_system_at_reset;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let (mut _buses, clocks, _tokens) = clock_system_at_reset(
        peripherals.oscctrl,
        peripherals.osc32kctrl,
        peripherals.gclk,
        peripherals.mclk,
        &mut peripherals.nvmctrl,
    );

    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led = pins.d13.into_push_pull_output();
    let (mut delay, _gclk0) = Delay::new(core.SYST, clocks.gclk0);
    loop {
        delay.delay_ms(2000u16);
        red_led.set_high().unwrap();
        delay.delay_ms(2000u16);
        red_led.set_low().unwrap();
    }
}
