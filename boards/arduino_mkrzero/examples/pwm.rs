#![no_std]
#![no_main]

use arduino_mkrzero as bsp;
use bsp::hal;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::{GenericClockController, Tcc0Tcc1Clock};
use hal::delay::Delay;
use hal::gpio::AlternateE;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::pwm::{Channel, Pwm0};

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
    let pins = bsp::pins::Pins::new(peripherals.PORT);

    // PWM0_CH1 is A4 on the board - pin 19 or PA05
    // see: https://github.com/arduino/ArduinoCore-samd/blob/master/variants/mkrzero/variant.cpp
    let _a4 = pins.pa04.into_mode::<AlternateE>();
    let gclk0 = clocks.gclk0();

    let tcc0_tcc1_clock: &Tcc0Tcc1Clock = &clocks.tcc0_tcc1(&gclk0).unwrap();

    let mut pwm0 = Pwm0::new(
        &tcc0_tcc1_clock,
        1u32.kHz(),
        peripherals.TCC0,
        &mut peripherals.PM,
    );
    let max_duty = pwm0.get_max_duty();
    pwm0.enable(Channel::_1);
    loop {
        pwm0.set_duty(Channel::_1, max_duty);
        delay.delay_ms(500u16);
        pwm0.set_duty(Channel::_1, max_duty / 4);
        delay.delay_ms(500u16);
    }
}
