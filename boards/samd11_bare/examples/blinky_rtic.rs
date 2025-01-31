//! Uses RTIC with the RTC as time source to blink an LED.
//!
//! The idle task is sleeping the CPU, so in practice this gives similar power
//! figure as the "sleeping_timer_rtc" example.
#![no_std]
#![no_main]

use bsp::hal;
use hal::rtc::rtic::rtc_clock;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;
use rtic::app;
use samd11_bare as bsp;

hal::rtc_monotonic!(Mono, rtc_clock::ClockCustom<2_048>);

#[app(device = bsp::pac, dispatchers = [EVSYS])]
mod app {

    use super::*;
    use hal::clock::{ClockGenId, ClockSource, GenericClockController};
    use hal::pac::Peripherals;
    use hal::prelude::*;

    #[local]
    struct Local {}

    #[shared]
    struct Shared {
        // The LED could be a local resource, since it is only used in one task
        // But we want to showcase shared resources and locking
        red_led: bsp::Led,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut peripherals: Peripherals = cx.device;
        let pins = bsp::Pins::new(peripherals.port);
        let mut core: rtic::export::Peripherals = cx.core;
        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.gclk,
            &mut peripherals.pm,
            &mut peripherals.sysctrl,
            &mut peripherals.nvmctrl,
        );

        // Set the RTC clock to use a 2.048 kHz clock derived from the external 32 kHz
        // oscillator.
        let rtc_clock_src = clocks
            .configure_gclk_divider_and_source(ClockGenId::Gclk2, 16, ClockSource::Xosc32k, false)
            .unwrap();
        clocks.configure_standby(ClockGenId::Gclk2, true);
        let _ = clocks.rtc(&rtc_clock_src).unwrap();

        let red_led: bsp::Led = pins.d2.into();

        // Start the monotonic
        Mono::start(peripherals.rtc);

        // We can use the RTC in standby for maximum power savings
        core.SCB.set_sleepdeep();

        // Start the blink task
        blink_led::spawn().unwrap();

        (Shared { red_led }, Local {})
    }

    /// This function is spawned and never returns.
    #[task(priority = 1, shared=[red_led])]
    async fn blink_led(mut cx: blink_led::Context) {
        loop {
            // If the LED were a local resource, the lock would not be necessary
            cx.shared.red_led.lock(|led| {
                led.toggle().unwrap();
            });
            Mono::delay(1u64.secs()).await;
        }
    }
}
