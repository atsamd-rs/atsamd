//! Uses RTIC with systick to blink a led in an asynchronous fashion.
#![no_std]
#![no_main]

use feather_m0 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{hal, pac, pin_alias};
use hal::clock::{ClockGenId, ClockSource, GenericClockController};
use hal::prelude::*;
use hal::rtc::rtic::rtc_clock;

// We can use the RTIC monotonic with embassy
hal::rtc_monotonic!(Mono, rtc_clock::ClockCustom<8_192>);

// However, to do so, we need to define this, which is normally defined within
// RTIC. This sets the RTC monotonic interrupt priority to be the most
// important.
#[no_mangle]
static RTIC_ASYNC_MAX_LOGICAL_PRIO: u8 = 1;

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

    // Set the RTC clock to use a 8.192 kHz clock derived from the external 32 kHz
    // oscillator.
    let rtc_clock_src = clocks
        .configure_gclk_divider_and_source(ClockGenId::Gclk2, 4, ClockSource::Xosc32k, false)
        .unwrap();
    clocks.configure_standby(ClockGenId::Gclk2, true);
    let _ = clocks.rtc(&rtc_clock_src).unwrap();

    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    // Start the monotonic
    Mono::start(peripherals.rtc);

    loop {
        let _ = red_led.toggle();
        Mono::delay(1u64.secs()).await;
    }
}
