#![no_std]
#![no_main]

use bsp::hal;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;
use wio_lite_mg126 as bsp;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
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
    let mut pins = bsp::Pins::new(peripherals.PORT);

    let _d9 = pins.d9.into_function_e(&mut pins.port);

    let gclk0 = clocks.gclk0();
    let mut pwm1 = Pwm1::new(
        &clocks.tcc0_tcc1(&gclk0).unwrap(),
        1.khz(),
        peripherals.TCC1,
        &mut peripherals.PM,
    );
    let max_duty = pwm1.get_max_duty();

    loop {
        pwm1.set_duty(Channel::_1, max_duty / 2);
        delay.delay_ms(1000u16);
        pwm1.set_duty(Channel::_1, max_duty / 8);
        delay.delay_ms(1000u16);
    }
}
