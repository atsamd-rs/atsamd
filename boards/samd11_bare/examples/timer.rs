#![no_std]
#![no_main]

use bsp::hal;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;
use samd11_bare as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::pac::Peripherals;
use hal::prelude::*;
use hal::timer::TimerCounter;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc1_tc2(&gclk0).unwrap();
    let mut timer = TimerCounter::tc1_(&timer_clock, peripherals.TC1, &mut peripherals.PM);
    timer.start(1u32.hz());

    let mut pins = bsp::Pins::new(peripherals.PORT);
    let mut d2 = pins.d2.into_open_drain_output(&mut pins.port);

    loop {
        d2.set_high().unwrap();
        nb::block!(timer.wait()).ok();
        d2.set_low().unwrap();
        nb::block!(timer.wait()).ok();
    }
}
