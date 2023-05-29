#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use bsp::hal;
use bsp::pac;
use metro_m4 as bsp;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::gpio::E;
use hal::prelude::*;
use hal::pwm::{Pwm2, TC2Pinout};
use pac::{CorePeripherals, Peripherals};

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
    let pins = bsp::Pins::new(peripherals.PORT);

    let d12 = pins.d12.into_alternate::<E>();

    let gclk0 = clocks.gclk0();
    let mut pwm2 = Pwm2::new(
        &clocks.tc2_tc3(&gclk0).unwrap(),
        1.kHz(),
        peripherals.TC2,
        TC2Pinout::Pa17(d12),
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
