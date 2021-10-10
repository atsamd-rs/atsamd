#![no_std]
#![no_main]

use bsp::hal;
use p1am_100 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::pac::Peripherals;
use hal::prelude::*;
use hal::timer::TimerCounter;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut led: bsp::Led = pins.led.into();

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
        led.set_high().unwrap();
        nb::block!(timer.wait()).unwrap();
        led.set_low().unwrap();
    }
}
