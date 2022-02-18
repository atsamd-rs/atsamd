#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use bsp::pac;
use itsybitsy_m0 as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::timer::TimerCounter;
use pac::Peripherals;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut red_led: bsp::RedLed = pins.d13.into();

    // gclk0 represents a configured clock using the system 48MHz oscillator
    let gclk0 = clocks.gclk0();
    // configure a clock for the TC4 and TC5 peripherals
    let tc45 = &clocks.tc4_tc5(&gclk0).unwrap();
    // instantiate a timer objec for the TC4 peripheral
    let mut timer = TimerCounter::tc4_(tc45, peripherals.TC4, &mut peripherals.PM);
    // start a 5Hz timer
    timer.start(5.hz());

    // toggle the red LED at the frequency set by the timer
    loop {
        nb::block!(timer.wait()).unwrap();
        red_led.set_high().unwrap();
        nb::block!(timer.wait()).unwrap();
        red_led.set_low().unwrap();
    }
}
