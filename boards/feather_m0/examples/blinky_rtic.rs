//! Uses RTIC with the RTC as time source to blink an LED.
//!
//! The idle task is sleeping the CPU, so in practice this gives similar power
//! figure as the "sleeping_timer_rtc" example.
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate feather_m0 as hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;
extern crate rtic;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [EVSYS])]
mod app {
    use hal::clock::{ClockGenId, ClockSource, GenericClockController};
    use hal::pac::Peripherals;
    use hal::rtc::{Rtc, Count32Mode};
    use rtic_monotonic::Extensions;

    #[resources]
    struct Resources {
        red_led:
            hal::gpio::Pin<hal::gpio::v2::PA17, hal::gpio::v2::Output<hal::gpio::v2::PushPull>>,
    }

    #[monotonic(binds = RTC, default = true)]
    type RtcMonotonic = Rtc<Count32Mode>;

    #[init]
    fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {
        let mut peripherals: Peripherals = cx.device;
        let mut pins = hal::Pins::new(peripherals.PORT);
        let mut core: rtic::export::Peripherals = cx.core;
        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let _gclk = clocks.gclk0();
        let rtc_clock_src = clocks
            .configure_gclk_divider_and_source(ClockGenId::GCLK2, 1, ClockSource::XOSC32K, false)
            .unwrap();
        clocks.configure_standby(ClockGenId::GCLK2, true);
        let rtc_clock = clocks.rtc(&rtc_clock_src).unwrap();
        let rtc = Rtc::count32_mode(peripherals.RTC, rtc_clock.freq(), &mut peripherals.PM);
        let red_led = pins.d13.into_open_drain_output(&mut pins.port);

        // We can use the RTC in standby for maximum power savings
        core.SCB.set_sleepdeep();

        // Start the blink task
        blink::spawn().unwrap();

        (init::LateResources { red_led }, init::Monotonics(rtc))
    }

    #[task(resources = [red_led])]
    fn blink(mut _cx: blink::Context) {
        _cx.resources.red_led.lock(|led| led.toggle());
        blink::spawn_after(1_u32.seconds()).ok();
    }
}
