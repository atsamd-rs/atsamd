#![no_std]
#![no_main]

use grand_central_m4 as hal;
use panic_semihosting as _;

use hal::adc::{Adc, InterruptAdc, FreeRunning};
use hal::clock::GenericClockController;
use hal::entry;
use hal::gpio::{Output, Pb1, PushPull};
use hal::pac::gclk::{genctrl::SRC_A::DFLL, pchctrl::GEN_A::GCLK1};
use hal::pac::{interrupt, Peripherals, ADC0};
use hal::prelude::*;

use cortex_m::peripheral::NVIC;

struct Ctx {
    adc: InterruptAdc<ADC0, FreeRunning>,
    red_led: Pb1<Output<PushPull>>,
}

static mut CTX: Option<Ctx> = None;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut a0 = pins.a0.into_function_b(&mut pins.port);
    let red_led = pins.red_led.into_open_drain_output(&mut pins.port);

    let clk = clocks
        .configure_gclk_divider_and_source(GCLK1, 7, DFLL, false)
        .unwrap();
    let adc0 = Adc::adc0(
        peripherals.ADC0,
        &mut peripherals.MCLK,
        &clocks.adc0(&clk).unwrap(),
        40.khz(),
    );

    let mut iradc0: InterruptAdc<ADC0, FreeRunning> = InterruptAdc::from(adc0);
    iradc0.start_conversion(&mut a0);

    unsafe {
        CTX = Some(Ctx {
            adc: iradc0,
            red_led: red_led,
        });

        NVIC::unmask(interrupt::ADC0_RESRDY);
    }

    loop {}
}

#[interrupt]
fn ADC0_RESRDY() {
    unsafe {
        let ctx = CTX.as_mut().unwrap();
        if let Some(_) = ctx.adc.service_interrupt_ready() {
            ctx.red_led.toggle();
            ctx.red_led.toggle();
        }
    }
}
