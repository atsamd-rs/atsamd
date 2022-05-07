#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{hal, pac, pin_alias};
use feather_m4 as bsp;
use hal::{
    clock::GenericClockController, ehal::digital::v2::ToggleableOutputPin, interrupt,
    thumbv7em::timer::TimerCounter, time::Milliseconds,
};
use pac::Peripherals;

#[embassy::main]
async fn main(_spawner: embassy::executor::Spawner) {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    let mut tc2_irq = hal::interrupt::take!(TC2);
    // tc4_irq.set_priority(2);

    // gclk0 represents a configured clock using the 120MHz oscillator.
    let gclk0 = clocks.gclk0();
    // Configure a clock for TC2 and TC3 peripherals
    let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();
    //Instantiate a timer object for the TC2 peripheral
    let mut timer = TimerCounter::tc2_(&timer_clock, peripherals.TC2, &mut peripherals.MCLK);
    let mut timer_fut = timer.as_async(&mut tc2_irq);

    loop {
        timer_fut.delay(Milliseconds(500)).await;
        red_led.toggle().unwrap();
        cortex_m::asm::wfi();
    }
}
