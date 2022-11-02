#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(device = bsp::pac, dispatchers = [I2S])]
mod app {
    use bsp::{hal, pac, pin_alias};
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

    #[monotonic(binds = RTC, default = true)]
    type Monotonic = Rtc<Count32Mode>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        extint: ExtInt2<Pin<PA18, PullUpInterrupt>, hal::pac::Interrupt>,
        red_led: bsp::RedLed,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
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

        let timer_clock = clocks
            .configure_gclk_divider_and_source(ClockGenId::GCLK2, 1, ClockSource::OSC32K, false)
            .unwrap();
        clocks.configure_standby(ClockGenId::GCLK2, true);

        let eic_irq = cortex_m_interrupt::take_nvic_interrupt!(pac::Interrupt::EIC, 2);
        // tc4_irq.set_priority(2);

        enable_internal_32kosc(&mut peripherals.SYSCTRL);

        // Setup RTC monotonic
        let rtc_clock = clocks.rtc(&timer_clock).unwrap();
        let rtc = Rtc::count32_mode(peripherals.RTC, rtc_clock.freq(), &mut peripherals.PM);

        // configure a clock for the EIC peripheral
        let gclk0 = clocks.gclk0();
        let eic_clock = clocks.eic(&gclk0).unwrap();

        let mut eic =
            EIC::init(&mut peripherals.PM, eic_clock, peripherals.EIC).into_future(eic_irq);
        let button: Pin<_, PullUpInterrupt> = pins.d10.into();
        let mut extint = ExtInt2::new(button, &mut eic);
        extint.enable_interrupt_wake();

        async_task::spawn().ok();

        (Shared {}, Local { extint, red_led }, init::Monotonics(rtc))
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
