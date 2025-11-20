//! Uses RTIC with a software task to blink an LED.
#![no_std]
#![no_main]

use bsp::hal;
use circuit_playground_express as bsp;
use hal::rtc::rtic::rtc_clock;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

hal::rtc_monotonic!(Mono, rtc_clock::ClockCustom<8_192>);

#[rtic::app(device = bsp::pac, dispatchers = [EVSYS])]
mod app {
    use super::*;
    use bsp::{hal, pin_alias};
    use hal::clock::{ClockGenId, ClockSource, GenericClockController};
    use hal::pac::Peripherals;
    use hal::prelude::*;

    #[local]
    struct Local {}

    #[shared]
    struct Shared {
        // The LED could be a local resource, since it is only used in one task
        // But we want to showcase shared resources and locking
        red_led: bsp::RedLed,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut peripherals: Peripherals = cx.device;
        let pins = bsp::Pins::new(peripherals.port);
        let mut core: rtic::export::Peripherals = cx.core;
        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.gclk,
            &mut peripherals.pm,
            &mut peripherals.sysctrl,
            &mut peripherals.nvmctrl,
        );

        // Set the RTC clock to use a 8.192 kHz clock derived from the internal 32 kHz
        // oscillator.
        let rtc_clock_src = clocks
            .configure_gclk_divider_and_source(ClockGenId::Gclk2, 4, ClockSource::Osc32k, true)
            .unwrap();
        clocks.configure_standby(ClockGenId::Gclk2, true);
        let _ = clocks.rtc(&rtc_clock_src).unwrap();

        let red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

        // Start the monotonic
        Mono::start(peripherals.rtc);

        // We can use the RTC in standby for maximum power savings
        core.SCB.set_sleepdeep();

        // Start the blink task
        blink::spawn().unwrap();

        (Shared { red_led }, Local {})
    }

    #[task(shared = [red_led])]
    async fn blink(mut cx: blink::Context) {
        loop {
            // If the LED were a local resource, the lock would not be necessary
            cx.shared.red_led.lock(|led| led.toggle().unwrap());

            Mono::delay(1u64.secs()).await;
        }
    }
}
