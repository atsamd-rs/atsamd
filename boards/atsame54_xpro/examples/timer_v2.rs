//! Timer V2 example
//! WIP

#![no_std]
#![no_main]

use atsame54_xpro as bsp;
use bsp::hal;
use hal::clock::v2 as clock;
use hal::clock::v2::gclk::Gclk1Id;
use hal::clock::v2::types::*;
// TODO: Prelude enforces import of `fugit::ExtU32` that ruins everything..
// Temporarily remove `crate::prelude::*` import and take what's needed.
use hal::ehal::digital::v2::OutputPin as _;
use hal::eic::pin::*;
use hal::fugit::ExtU64;
use hal::gpio::{Interrupt as GpioInterrupt, *};
use hal::timer::v2::*;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

type Button = ExtInt15<Pin<PB31, GpioInterrupt<PullUp>>>;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [FREQM])]
mod app {
    use super::*;

    #[monotonic(binds = TC0, default = true)]
    type Mono = MonotonicTimer<32, Tc0Tc1, Gclk1Id>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        button: Button,
        led: bsp::Led,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("Application up!");

        let (mut buses, clocks, tokens) = clock::clock_system_at_reset(
            ctx.device.OSCCTRL,
            ctx.device.OSC32KCTRL,
            ctx.device.GCLK,
            ctx.device.MCLK,
            &mut ctx.device.NVMCTRL,
        );

        let (_, _, _, mut mclk) = unsafe { clocks.pac.steal() };

        let pins = bsp::Pins::new(ctx.device.PORT);

        let (pclk_eic, _gclk0) = clock::pclk::Pclk::enable(tokens.pclks.eic, clocks.gclk0);

        let mut eic = hal::eic::init_with_ulp32k(&mut mclk, pclk_eic.into(), ctx.device.EIC);
        let mut button = bsp::pin_alias!(pins.button).into_pull_up_ei();
        eic.button_debounce_pins(&[button.id()]);
        button.sense(&mut eic, Sense::FALL);
        button.enable_interrupt(&mut eic);
        eic.finalize();

        let (osculp32k, _) =
            clock::osculp32k::OscUlp32k::enable(tokens.osculp32k.osculp32k, clocks.osculp32k_base);

        let (gclk1, _osculp32k) = clock::gclk::Gclk::from_source(tokens.gclks.gclk1, osculp32k);
        let gclk1 = gclk1.enable();

        let tc0_clk = buses.apb.enable(tokens.apbs.tc0);
        let tc1_clk = buses.apb.enable(tokens.apbs.tc1);

        let (tc0_tc1_pclk, _gclk1) = clock::pclk::Pclk::enable(tokens.pclks.tc0_tc1, gclk1);

        let mono = Timer::paired(
            ctx.device.TC0,
            tc0_clk,
            ctx.device.TC1,
            tc1_clk,
            tc0_tc1_pclk,
        )
        .into_32_bit()
        .into_monotonic()
        .unwrap();

        let led: bsp::Led = bsp::pin_alias!(pins.led).into();
        bump_activity_led();

        (Shared {}, Local { button, led }, init::Monotonics(mono))
    }

    #[task(binds = EIC_EXTINT_15, local = [button])]
    fn button(ctx: button::Context) {
        ctx.local.button.clear_interrupt();

        rprintln!("Button pressed!");
        hello::spawn().unwrap();
        hello::spawn_after(50.millis()).unwrap();
        hello::spawn_after(100.millis()).unwrap();
        hello::spawn_after(500.millis()).unwrap();
        hello::spawn_after(1.secs()).unwrap();
        hello::spawn_after(5.secs()).unwrap();
    }

    #[task(capacity = 30)]
    fn hello(_: hello::Context) {
        bump_activity_led();
        rprintln!(
            "Hello at {} ms since epoch!",
            monotonics::now().duration_since_epoch().to_millis()
        );
    }

    #[task(local = [led])]
    fn activity_led(ctx: activity_led::Context, led_on: bool) {
        let _ = ctx.local.led.set_state((!led_on).into());
        if led_on {
            let _ = activity_led::spawn_after(100.millis(), false);
        }
    }

    fn bump_activity_led() {
        let _ = activity_led::spawn(true);
    }
}
