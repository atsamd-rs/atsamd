#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use samd11_bare as bsp;

use bsp::hal;
use bsp::pac;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::ehal::digital::OutputPin;
use hal::nb;
use hal::time::Hertz;
use hal::timer::TimerCounter;
use hal::timer_traits::InterruptDrivenTimer;
use pac::Peripherals;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc1_tc2(&gclk0).unwrap();
    let mut timer = TimerCounter::tc1_(&timer_clock, peripherals.tc1, &mut peripherals.pm);
    timer.start(Hertz::Hz(1).into_duration());

    let pins = bsp::Pins::new(peripherals.port);
    let mut d2: bsp::Led = pins.d2.into();

    loop {
        d2.set_high().unwrap();
        nb::block!(timer.wait()).ok();
        d2.set_low().unwrap();
        nb::block!(timer.wait()).ok();
    }
}
