#![no_std]
#![no_main]

use atsame54_xpro as bsp;
use bsp::hal;
use bsp::hal::clock::v2 as clock;
use dwt_systick_monotonic::DwtSystick;
use dwt_systick_monotonic::ExtU32 as _;
// TODO: Any reason this cannot be in a HAL's prelude?
use hal::ehal::digital::v2::StatefulOutputPin as _;
use hal::prelude::*;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [FREQM])]
mod app {
    use super::*;

    #[monotonic(binds = SysTick, default = true)]
    type Mono = DwtSystick<12_000_000>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: bsp::Led,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

        let (_buses, clocks, tokens) = clock::clock_system_at_reset(
            ctx.device.OSCCTRL,
            ctx.device.OSC32KCTRL,
            ctx.device.GCLK,
            ctx.device.MCLK,
            &mut ctx.device.NVMCTRL,
        );

        let pins = bsp::Pins::new(ctx.device.PORT);
        let xosc = clock::xosc::Xosc::from_crystal(
            tokens.xosc1,
            bsp::pin_alias!(pins.xosc1_x_in),
            bsp::pin_alias!(pins.xosc1_x_out),
            // Xosc1 on Same54Xpro is 12 MHz
            12.MHz(),
        )
        .enable();

        let (gclk0, _, _) = clocks.gclk0.swap_sources(clocks.dfll, xosc);

        let mono = DwtSystick::new(
            &mut ctx.core.DCB,
            ctx.core.DWT,
            ctx.core.SYST,
            gclk0.freq().to_Hz(),
        );

        let led = bsp::pin_alias!(pins.led).into();

        led::spawn().unwrap();

        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[task(local = [led])]
    fn led(ctx: led::Context) {
        ctx.local.led.toggle().unwrap();
        rprintln!(
            "LED {}!",
            if ctx.local.led.is_set_high().unwrap() {
                "OFF"
            } else {
                "ON"
            }
        );
        led::spawn_at(monotonics::now() + 200.millis()).unwrap();
    }
}
