//! Blink an led without using the BSP split() method.

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, Pins, RedLed};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use hal::clock::v2::clock_system_at_reset;
use hal::delay::Delay;
use hal::prelude::*;
use hal::watchdog::{Watchdog, WatchdogTimeout};
use pac::{CorePeripherals, Peripherals};

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

    let (mut delay, _gclk0) = Delay::new(core.SYST, clocks.gclk0);
    delay.delay_ms(400u16);

    let pins = Pins::new(peripherals.port);
    let mut red_led: RedLed = pins.d13.into();

    let mut wdt = Watchdog::new(peripherals.wdt);
    wdt.start(WatchdogTimeout::Cycles256 as u8);

    loop {
        delay.delay_ms(200u8);
        wdt.feed();
        red_led.set_high().unwrap();
        delay.delay_ms(200u8);
        wdt.feed();
        red_led.set_low().unwrap();
    }
}
