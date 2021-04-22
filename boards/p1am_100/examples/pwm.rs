#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate p1am_100 as hal;
extern crate panic_halt;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::pwm::Pwm4;

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

    let _led = pins.led.into_mode::<hal::gpio::v2::AlternateE>();

    let gclk0 = clocks.gclk0();
    let mut pwm4 = Pwm4::new(
        &clocks.tc4_tc5(&gclk0).unwrap(),
        1.khz(),
        peripherals.TC4,
        &mut peripherals.PM,
    );
    let max_duty = pwm4.get_max_duty();

    loop {
        pwm4.set_duty(max_duty / 2);
        delay.delay_ms(1000u16);
        pwm4.set_duty(max_duty / 8);
        delay.delay_ms(1000u16);
    }
}
