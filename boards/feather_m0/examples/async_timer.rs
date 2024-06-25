#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{hal, pac, pin_alias};
use feather_m0 as bsp;
use fugit::MillisDurationU32;
use hal::{
    clock::{enable_internal_32kosc, ClockGenId, ClockSource, GenericClockController},
    ehal::digital::v2::ToggleableOutputPin,
    pac::TC4,
    thumbv6m::timer::TimerCounter,
};

atsamd_hal::bind_interrupts!(struct Irqs {
    TC4 => atsamd_hal::async_hal::timer::InterruptHandler<TC4>;
});

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    enable_internal_32kosc(&mut peripherals.SYSCTRL);
    let timer_clock = clocks
        .configure_gclk_divider_and_source(ClockGenId::GCLK2, 1, ClockSource::OSC32K, false)
        .unwrap();
    clocks.configure_standby(ClockGenId::GCLK2, true);

    // configure a clock for the TC4 and TC5 peripherals
    let tc45 = &clocks.tc4_tc5(&timer_clock).unwrap();

    // instantiate a timer object for the TC4 peripheral
    let timer = TimerCounter::tc4_(tc45, peripherals.TC4, &mut peripherals.PM);
    let mut timer = timer.into_future(Irqs);

    loop {
        timer
            .delay(MillisDurationU32::from_ticks(500).convert())
            .await;
        red_led.toggle().unwrap();
    }
}
