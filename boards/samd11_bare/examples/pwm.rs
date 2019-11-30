#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate panic_halt;
extern crate samd11_bare as hal;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::pwm::Pwm1;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut pins = hal::Pins::new(peripherals.PORT);

    let gclk0 = clocks.gclk0();
    let d1 = pins.d1.into_function_e(&mut pins.port);
    let mut pwm1 = Pwm1::new(
        &clocks.tc1_tc2(&gclk0).unwrap(),
        1.khz(),
        peripherals.TC1,
        hal::pwm::TC1Pinout::Pa5(d1),
        &mut peripherals.PM,
    );
    let max_duty = pwm1.get_max_duty();

    loop {
        pwm1.set_duty(max_duty / 2);
        delay.delay_ms(1000u16);
        pwm1.set_duty(max_duty / 8);
        delay.delay_ms(1000u16);
    }
}
