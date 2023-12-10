#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{hal, pin_alias};
use feather_m0 as bsp;
use fugit::MillisDurationU32;
use hal::{
    async_hal::timer::TimerFuture,
    clock::{enable_internal_32kosc, ClockGenId, ClockSource, GenericClockController},
    ehal::digital::v2::ToggleableOutputPin,
    pac::TC4,
    thumbv6m::timer::TimerCounter,
};

atsamd_hal::bind_interrupts!(struct Irqs {
    TC4 => atsamd_hal::async_hal::timer::InterruptHandler<TC4>;
});

#[rtic::app(device = bsp::pac, dispatchers = [I2S])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        timer: TimerFuture<TC4>,
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

        enable_internal_32kosc(&mut peripherals.SYSCTRL);
        let timer_clock = clocks
            .configure_gclk_divider_and_source(ClockGenId::GCLK2, 1, ClockSource::OSC32K, false)
            .unwrap();
        clocks.configure_standby(ClockGenId::GCLK2, true);

        // configure a clock for the TC4 and TC5 peripherals
        let tc45 = &clocks.tc4_tc5(&timer_clock).unwrap();

        // instantiate a timer object for the TC4 peripheral
        let timer = TimerCounter::tc4_(tc45, peripherals.TC4, &mut peripherals.PM);
        let timer = timer.into_future(Irqs);
        async_task::spawn().ok();

        (Shared {}, Local { timer, red_led })
    }

    #[task(local = [timer, red_led])]
    async fn async_task(cx: async_task::Context) {
        let timer = cx.local.timer;
        let red_led = cx.local.red_led;

        loop {
            timer
                .delay(MillisDurationU32::from_ticks(500).convert())
                .await;
            red_led.toggle().unwrap();
        }
    }
}
