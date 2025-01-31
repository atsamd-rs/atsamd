#![no_std]
#![no_main]

use atsame54_xpro as bsp;
use bsp::hal;
use hal::clock::v2::{clock_system_at_reset, osculp32k::OscUlp1k, rtcosc::RtcOsc};
use hal::prelude::*;
use hal::rtc::rtic::rtc_clock;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
// TODO: Any reason this cannot be in a HAL's prelude?
use hal::ehal::digital::StatefulOutputPin;

hal::rtc_monotonic!(Mono, rtc_clock::Clock1k);

#[rtic::app(device = hal::pac, dispatchers = [FREQM])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: bsp::Led,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let mut device = ctx.device;
        let mut core: rtic::export::Peripherals = ctx.core;

        rtt_init_print!();

        let (_buses, clocks, tokens) = clock_system_at_reset(
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

        let pins = bsp::Pins::new(device.port);

        // We can use the RTC in standby for maximum power savings
        core.SCB.set_sleepdeep();

        blink_led::spawn().unwrap();

        (
            Shared {},
            Local {
                led: bsp::pin_alias!(pins.led).into(),
            },
        )
    }

    /// This function is spawned and never returns.
    #[task(priority = 1, local=[led])]
    async fn blink_led(ctx: blink_led::Context) {
        StatefulOutputPin::toggle(ctx.local.led).unwrap();
        rprintln!(
            "LED {}!",
            if ctx.local.led.is_set_high().unwrap() {
                "OFF"
            } else {
                "ON"
            }
        );
        Mono::delay(200u64.millis()).await;
    }
}
