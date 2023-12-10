//! Uses RTIC with systick to blink a led in an asynchronous fashion.
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use feather_m0 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{hal, pin_alias};
use fugit::MillisDurationU32;
use hal::clock::GenericClockController;
use hal::pac::Peripherals;
use hal::prelude::*;
use rtic_monotonics::systick::Systick;

#[rtic::app(device = bsp::pac, peripherals = true, dispatchers = [EVSYS])]
mod app {
    use super::*;

    #[local]
    struct Local {
        red_led: bsp::RedLed,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut peripherals: Peripherals = cx.device;
        let pins = bsp::Pins::new(peripherals.PORT);
        let _core: rtic::export::Peripherals = cx.core;
        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let _gclk = clocks.gclk0();

        let red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

        // Start the blink task
        blink::spawn().unwrap();

        (Shared {}, Local { red_led })
    }

    #[task(local = [red_led])]
    async fn blink(cx: blink::Context) {
        loop {
            let _ = cx.local.red_led.toggle();
            Systick::delay(MillisDurationU32::from_ticks(1000).convert()).await;
        }
    }
}
