#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

use bsp::{hal, pin_alias};
use feather_m0 as bsp;
use hal::{
    clock::{enable_internal_32kosc, ClockGenId, ClockSource, GenericClockController},
    ehal::digital::v2::ToggleableOutputPin,
    eic::{
        pin::{ExtInt2, Sense},
        EIC,
    },
    gpio::{pin::PA18, Pin, PullUpInterrupt},
    rtc::{Count32Mode, Rtc},
};

atsamd_hal::bind_interrupts!(struct Irqs {
    EIC => atsamd_hal::eic::InterruptHandler;
});

#[rtic::app(device = bsp::pac, dispatchers = [I2S])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        extint: ExtInt2<Pin<PA18, PullUpInterrupt>>,
        red_led: bsp::RedLed,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut peripherals = cx.device;
        let _core = cx.core;

        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let pins = bsp::Pins::new(peripherals.PORT);
        let red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

        let internal_clock = clocks
            .configure_gclk_divider_and_source(ClockGenId::GCLK2, 1, ClockSource::OSC32K, false)
            .unwrap();
        clocks.configure_standby(ClockGenId::GCLK2, true);

        enable_internal_32kosc(&mut peripherals.SYSCTRL);

        // Configure a clock for the EIC peripheral
        let gclk2 = clocks.gclk2();
        let eic_clock = clocks.eic(&gclk2).unwrap();

        let mut eic = EIC::init(&mut peripherals.PM, eic_clock, peripherals.EIC).into_future(Irqs);
        let button: Pin<_, PullUpInterrupt> = pins.d10.into();
        let mut extint = ExtInt2::new(button, &mut eic);
        extint.enable_interrupt_wake();

        async_task::spawn().ok();

        (Shared {}, Local { extint, red_led })
    }

    #[task(local = [extint, red_led])]
    async fn async_task(cx: async_task::Context) {
        let extint = cx.local.extint;
        let red_led = cx.local.red_led;

        loop {
            // Here we show straight falling edge detection without
            extint.wait(Sense::FALL).await;
            defmt::info!("Falling edge detected");
            red_led.toggle().unwrap();
        }
    }
}
