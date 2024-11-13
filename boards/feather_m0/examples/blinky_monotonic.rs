//! Uses RTIC with systick to blink a led in an asynchronous fashion.
#![no_std]
#![no_main]

use feather_m0 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{hal, pac, pin_alias};
use fugit::MillisDurationU32;
use hal::clock::GenericClockController;
use hal::prelude::*;
use rtic_monotonics::systick::Systick;

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();
    let pins = bsp::Pins::new(peripherals.port);
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let _gclk = clocks.gclk0();

    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    loop {
        let _ = red_led.toggle();
        Systick::delay(MillisDurationU32::from_ticks(1000).convert()).await;
    }
}
