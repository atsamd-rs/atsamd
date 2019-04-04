#![no_std]
#![no_main]

extern crate panic_halt;
extern crate feather_m0 as hal;
extern crate cortex_m_rt;

use hal::clock::GenericClockController;
use hal::{Peripherals, CorePeripherals};
use hal::prelude::*;
use hal::delay::Delay;
use hal::pwm::{Pwm3, Pwm3Wrapper};

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

    let mut pins = hal::Pins::new(peripherals.PORT);
    let d5 = pins.d5.into_function_e(&mut pins.port);
    let gclk0 = clocks.gclk0();
    let pwm3 = Pwm3::new(
        &clocks.tcc2_tc3(&gclk0).unwrap(),
        1.khz(),
        peripherals.TC3,
        hal::pwm::TC3Pinout::Pa15(d5),
        &mut peripherals.PM,
    );
    let mut pwm3 = Pwm3Wrapper { pwm: pwm3 };

    let max_duty = pwm3.get_max_duty();

    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        pwm3.set_duty(max_duty/2);
        delay.delay_ms(1000u16);
        pwm3.set_duty(max_duty/8);
        delay.delay_ms(1000u16);
    }
}
