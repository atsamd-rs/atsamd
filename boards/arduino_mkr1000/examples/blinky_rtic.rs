//! Uses RTIC with the RTC as time source to blink an LED.
//!
//! The idle task is sleeping the CPU, so in practice this gives similar power
//! figure as the "sleeping_timer_rtc" example.
//!
//! Note on imports: this example requires `cortex-m-rtic`, whih brings the
//! `rtic` package, but also brings/selects the appropriate RTIC backend.
//!
//! If `rtic` package is imported instead of `cortex-m-rtic`, following message
//! is displayed at compilation, containing `No backend feature selected.`:
//!
//! ```sh
//! $ cargo build
//! (...)
//! error: failed to run custom build command for `rtic v2.2.0`
//! Caused by:
//!   process didn't exit successfully: `/home/.../dev/électronique/my-great-prj/target/debug/build/rtic-4840d1be515a89f0/build-script-build` (exit status: 101)
//!   --- stderr
//!   thread 'main' panicked at /home/.../.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rtic-2.2.0/build.rs:18:34:
//!   No backend feature selected.
//!   note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//! ```

#![no_std]
#![no_main]

use arduino_mkr1000 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;
use rtic::app;

#[app(device = bsp::pac, peripherals = true, dispatchers = [EVSYS])]
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
        led: bsp::pins::Led,
    }

    #[monotonic(binds = RTC, default = true)]
    type RtcMonotonic = Rtc<Count32Mode>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut peripherals: Peripherals = cx.device;
        let pins = bsp::pins::Pins::new(peripherals.PORT);
        let mut core: rtic::export::Peripherals = cx.core;
        let mut clocks = GenericClockController::with_external_32kosc(
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
        let led = bsp::pin_alias!(pins.led).into();

        // We can use the RTC in standby for maximum power savings
        core.SCB.set_sleepdeep();

        // Start the blink task
        blink::spawn().unwrap();

        (Shared { led }, Local {}, init::Monotonics(rtc))
    }

    #[task(shared = [led])]
    fn blink(mut cx: blink::Context) {
        // If the LED were a local resource, the lock would not be necessary
        let _ = cx.shared.led.lock(|led| led.toggle());
        blink::spawn_after(Duration::millis(750)).ok();
    }
}
