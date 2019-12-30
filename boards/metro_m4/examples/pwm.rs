#![no_std]
#![no_main]

extern crate panic_halt;
extern crate metro_m4 as hal;
extern crate cortex_m_rt;

use hal::clock::GenericClockController;
use hal::pac::{Peripherals, CorePeripherals};
use hal::prelude::*;
use hal::delay::Delay;
use hal::pwm::Pwm2;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut pins = hal::Pins::new(peripherals.PORT);

    let d12 = pins.d12.into_function_e(&mut pins.port);

    let gclk0 = clocks.gclk0();
    let mut pwm2 = Pwm2::new(
        &clocks.tc2_tc3(&gclk0).unwrap(),
        1.khz(),
        peripherals.TC2,
        hal::pwm::TC2Pinout::Pa17(d12),
        &mut peripherals.MCLK,
    );
    let max_duty = pwm2.get_max_duty();

    loop {
        pwm2.set_duty(max_duty / 2);
        delay.delay_ms(1000u16);
        pwm2.set_duty(max_duty / 8);
        delay.delay_ms(1000u16);
    }
}
