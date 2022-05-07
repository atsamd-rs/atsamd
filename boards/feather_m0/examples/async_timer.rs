#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{hal, pac, pin_alias};
use feather_m0 as bsp;
use hal::{
    clock::{enable_internal_32kosc, ClockGenId, ClockSource, GenericClockController},
    ehal::digital::v2::ToggleableOutputPin,
    interrupt,
    thumbv6m::timer::TimerCounter,
    time::Milliseconds,
};
use pac::Peripherals;

#[embassy::main]
async fn main(_spawner: embassy::executor::Spawner) {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    let mut tc4_irq = hal::interrupt::take!(TC4);
    // tc4_irq.set_priority(2);

    enable_internal_32kosc(&mut peripherals.SYSCTRL);
    let timer_clock = clocks
        .configure_gclk_divider_and_source(ClockGenId::GCLK2, 1, ClockSource::OSC32K, false)
        .unwrap();
    clocks.configure_standby(ClockGenId::GCLK2, true);

    // configure a clock for the TC4 and TC5 peripherals
    let tc45 = &clocks.tc4_tc5(&timer_clock).unwrap();

    // instantiate a timer objec for the TC3 peripheral
    let mut timer = TimerCounter::tc4_(tc45, peripherals.TC4, &mut peripherals.PM);
    let mut timer_fut = timer.as_async(&mut tc4_irq);

    loop {
        timer_fut.delay(Milliseconds(500)).await;
        red_led.toggle().unwrap();
        cortex_m::asm::wfi();
    }
}
