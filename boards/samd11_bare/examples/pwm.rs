#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use samd11_bare as bsp;

use bsp::hal;

use bsp::entry;
use cortex_m::peripheral::Peripherals as CorePeripherals;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::gpio::*;
use hal::pac::Peripherals;
use hal::prelude::*;
use hal::pwm::{Channel, Pwm0};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let pins = bsp::Pins::new(peripherals.port);

    let _d1: Pin<_, AlternateF> = pins.d1.into_mode();
    let _d14: Pin<_, AlternateF> = pins.d14.into_mode();

    let gclk0 = clocks.gclk0();
    let mut pwm0 = Pwm0::new(
        &clocks.tcc0(&gclk0).unwrap(),
        1.kHz(),
        peripherals.tcc0,
        &mut peripherals.pm,
    );
    let max_duty = pwm0.get_max_duty();

    loop {
        pwm0.set_duty(Channel::_0, max_duty / 2);
        pwm0.set_duty(Channel::_1, max_duty / 8);
        delay.delay_ms(1000u16);

        pwm0.set_duty(Channel::_0, max_duty / 8);
        pwm0.set_duty(Channel::_1, max_duty / 2);
        delay.delay_ms(1000u16);
    }
}
