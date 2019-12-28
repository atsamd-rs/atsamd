#![no_std]
#![no_main]

extern crate panic_halt;
extern crate feather_m0 as hal;
extern crate cortex_m_rt;

use hal::clock::GenericClockController;
use hal::pac::{Peripherals, CorePeripherals};
use hal::prelude::*;
use hal::delay::Delay;
use hal::pwm::Pwm3;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut pins = hal::Pins::new(peripherals.PORT);

    let _d5 = pins.d5.into_function_e(&mut pins.port);

    let gclk0 = clocks.gclk0();
    let mut pwm3 = Pwm3::new(
        &clocks.tcc2_tc3(&gclk0).unwrap(),
        1.khz(),
        peripherals.TC3,
        &mut peripherals.PM,
    );
    let max_duty = pwm3.get_max_duty();

    loop {
        pwm3.set_duty(max_duty / 2);
        delay.delay_ms(1000u16);
        pwm3.set_duty(max_duty / 8);
        delay.delay_ms(1000u16);
    }
}
