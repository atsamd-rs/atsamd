//! Uses RTIC with the RTC as time source to blink an LED.
//!
//! The idle task is sleeping the CPU, so in practice this gives similar power
//! figure as the "sleeping_timer_rtc" example.
#![no_std]
#![no_main]

use itsybitsy_m0 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;
use rtic;

#[rtic::app(device = bsp::pac, peripherals = true, dispatchers = [EVSYS])]
mod app {
    use super::*;
    use bsp::hal;
    use hal::clock::{ClockGenId, ClockSource, GenericClockController};
    use hal::pac::Peripherals;
    use hal::prelude::*;
    use hal::rtc::{Count32Mode, Duration, Rtc};

    #[local]
    struct Local {}

    #[shared]
    struct Shared {
        // The LED could be a local resource, since it is only used in one task
        // But we want to showcase shared resources and locking
        red_led: bsp::RedLed,
    }

    #[monotonic(binds = RTC, default = true)]
    type RtcMonotonic = Rtc<Count32Mode>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut peripherals: Peripherals = cx.device;
        let pins = bsp::Pins::new(peripherals.PORT);
        let mut core: rtic::export::Peripherals = cx.core;
        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let _gclk = clocks.gclk0();
        let rtc_clock_src = clocks
            .configure_gclk_divider_and_source(ClockGenId::GCLK2, 1, ClockSource::XOSC32K, false)
            .unwrap();
        clocks.configure_standby(ClockGenId::GCLK2, true);
        let rtc_clock = clocks.rtc(&rtc_clock_src).unwrap();
        let rtc = Rtc::count32_mode(peripherals.RTC, rtc_clock.freq(), &mut peripherals.PM);
        let red_led: bsp::RedLed = pins.d13.into();

        // We can use the RTC in standby for maximum power savings
        core.SCB.set_sleepdeep();

        // Start the blink task
        blink::spawn().unwrap();

        (Shared { red_led }, Local {}, init::Monotonics(rtc))
    }

    #[task(shared = [red_led])]
    fn blink(mut cx: blink::Context) {
        // If the LED were a local resource, the lock would not be necessary
        cx.shared.red_led.lock(|led| led.toggle().unwrap());
        blink::spawn_after(Duration::secs(1)).ok();
    }
}
