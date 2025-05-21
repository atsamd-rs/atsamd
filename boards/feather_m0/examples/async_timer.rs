#![no_std]
#![no_main]

use atsamd_hal::time::Milliseconds;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{hal, pac, pin_alias};
use feather_m0 as bsp;
use hal::{
    clock::{enable_internal_32kosc, ClockGenId, ClockSource, GenericClockController},
    ehal::digital::StatefulOutputPin,
    pac::Tc4,
    timer::TimerCounter,
};

atsamd_hal::bind_interrupts!(struct Irqs {
    TC4 => atsamd_hal::timer::InterruptHandler<Tc4>;
});

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    enable_internal_32kosc(&mut peripherals.sysctrl);
    let timer_clock = clocks
        .configure_gclk_divider_and_source(ClockGenId::Gclk2, 1, ClockSource::Osc32k, false)
        .unwrap();
    clocks.configure_standby(ClockGenId::Gclk2, true);

    // configure a clock for the TC4 and TC5 peripherals
    let tc45 = &clocks.tc4_tc5(&timer_clock).unwrap();

    // instantiate a timer object for the TC4 peripheral
    let timer = TimerCounter::tc4_(tc45, peripherals.tc4, &mut peripherals.pm);
    let mut timer = timer.into_future(Irqs);

    loop {
        timer.delay(Milliseconds::from_ticks(500).convert()).await;
        red_led.toggle().unwrap();
    }
}
