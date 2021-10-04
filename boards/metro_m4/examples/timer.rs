#![no_std]
#![no_main]

use bsp::hal;
use metro_m4 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use crate::hal::clock::GenericClockController;
use crate::hal::pac::Peripherals;
use crate::hal::timer::TimerCounter;
use cortex_m_rt::entry;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = bsp::Pins::new(peripherals.PORT);

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.MCLK);
    timer.start(9600u32.hz());
    let mut d0 = pins.d0.into_push_pull_output(&mut pins.port);
    loop {
        d0.set_high().unwrap();
        nb::block!(timer.wait()).ok();
        d0.set_low().unwrap();
        nb::block!(timer.wait()).ok();
    }
}
