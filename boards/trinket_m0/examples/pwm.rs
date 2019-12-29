#![no_std]
#![no_main]

extern crate panic_halt;
extern crate trinket_m0 as hal;
extern crate cortex_m_rt;

use hal::clock::GenericClockController;
use hal::pac::{Peripherals, CorePeripherals};
use hal::prelude::*;
use hal::delay::Delay;
use hal::pwm::{Channel, Pwm1};

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

    let _d13 = pins.d13.into_function_e(&mut pins.port);

    let gclk0 = clocks.gclk0();
    let mut pwm1 = Pwm1::new(
        &clocks.tcc0_tcc1(&gclk0).unwrap(),
        1.khz(),
        peripherals.TCC1,
        &mut peripherals.PM,
    );
    let max_duty = pwm1.get_max_duty();

    loop {
        pwm1.set_duty(Channel::_0, max_duty / 2);
        delay.delay_ms(1000u16);
        pwm1.set_duty(Channel::_0, max_duty / 8);
        delay.delay_ms(1000u16);
    }
}
