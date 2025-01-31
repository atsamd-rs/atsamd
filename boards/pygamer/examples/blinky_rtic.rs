//! Blink an led using an RTIC software task and the RTC-based monotonic.

#![no_std]
#![no_main]

use bsp::{hal, Pins, RedLed};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use hal::clock::v2::{clock_system_at_reset, osculp32k::OscUlp1k, rtcosc::RtcOsc};
use hal::prelude::*;
use hal::rtc::rtic::rtc_clock;
use rtic::app;

hal::rtc_monotonic!(Mono, rtc_clock::Clock1k);

#[app(device = bsp::pac, dispatchers = [EVSYS_0])]
mod app {
    use super::*;

    #[local]
    struct Resources {}

    #[shared]
    struct Shared {
        // The LED could be a local resource, since it is only used in one task
        // But we want to showcase shared resources and locking
        red_led: RedLed,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Resources) {
        let mut device = cx.device;
        let mut core: rtic::export::Peripherals = cx.core;

        // Use v2 of the clocks API so that we can set the RTC clock source
        let (_, clocks, tokens) = clock_system_at_reset(
            device.oscctrl,
            device.osc32kctrl,
            device.gclk,
            device.mclk,
            &mut device.nvmctrl,
        );

        // Enable the 1 kHz clock from the internal 32 kHz source
        let (osculp1k, _) = OscUlp1k::enable(tokens.osculp32k.osculp1k, clocks.osculp32k_base);

        // Enable the RTC clock with the 1 kHz source.
        // Note that currently the proof of this (the `RtcOsc` instance) is not
        // required to start the monotonic.
        let _ = RtcOsc::enable(tokens.rtcosc, osculp1k);

        // Start the monotonic
        Mono::start(device.rtc);

        let pins = Pins::new(device.port).split();

        // We can use the RTC in standby for maximum power savings
        core.SCB.set_sleepdeep();

        blink_led::spawn().ok().unwrap();

        (
            Shared {
                red_led: pins.led_pin.into(),
            },
            Resources {},
        )
    }

    /// This function is spawned and never returns.
    #[task(priority = 1, shared=[red_led])]
    async fn blink_led(mut cx: blink_led::Context) {
        loop {
            // If the LED were a local resource, the lock would not be necessary
            cx.shared.red_led.lock(|led| {
                led.toggle().unwrap();
            });
            Mono::delay(400u64.millis()).await;
        }
    }
}
