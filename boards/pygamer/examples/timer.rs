#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate embedded_hal;
extern crate nb;
extern crate panic_halt;
extern crate pygamer as hal;

use crate::hal::clock::GenericClockController;
use crate::hal::pac::Peripherals;
use crate::hal::timer::TimerCounter;
use cortex_m_rt::entry;
use hal::prelude::*;
use nb::block;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.MCLK);
    timer.start(3_000_000u32.hz());
    let mut d5 = pins.d5.into_push_pull_output(&mut pins.port);
    loop {
        let _ = d5.set_high();
        let _ = block!(timer.wait());
        let _ = d5.set_low();
        let _ = block!(timer.wait());
    }
}
