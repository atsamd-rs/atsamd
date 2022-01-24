#![no_std]
#![no_main]

use cortex_m_rt::entry;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use bsp::pac;
use feather_m0 as bsp;

use bsp::pin_alias;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::pwm::Pwm3;
use pac::{CorePeripherals, Peripherals};

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
    let pins = bsp::Pins::new(peripherals.PORT);

    let _d5: bsp::D5Pwm = pin_alias!(pins.d5_pwm).into();

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
