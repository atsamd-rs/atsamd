//! Uses RTIC with a software task to blink an LED.
#![no_std]
#![no_main]

use feather_m0 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

#[rtic::app(device = bsp::pac, peripherals = true, dispatchers = [EVSYS])]
mod app {
    use super::*;
    use bsp::{hal, pin_alias};
    use fugit::MillisDurationU32;
    use hal::clock::GenericClockController;
    use hal::pac::Peripherals;
    use hal::prelude::*;
    use rtic_monotonics::systick::Systick;

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
        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.gclk,
            &mut peripherals.pm,
            &mut peripherals.sysctrl,
            &mut peripherals.nvmctrl,
        );
        let _gclk = clocks.gclk0();
        let red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

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

            // cx.local.rtc.delay(Duration::secs(1)).await;
            Systick::delay(MillisDurationU32::from_ticks(1000).convert()).await;
        }
    }
}
