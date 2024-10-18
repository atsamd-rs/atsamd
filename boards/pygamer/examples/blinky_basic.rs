//! Blink an led without using the BSP split() method.

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, Pins, RedLed};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::watchdog::{Watchdog, WatchdogTimeout};
use pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
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
