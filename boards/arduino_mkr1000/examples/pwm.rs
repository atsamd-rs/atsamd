#![no_std]
#![no_main]

use arduino_mkr1000 as bsp;
use bsp::hal;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
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
    let gclk0 = clocks.gclk0();

    // configure target pin
    let _pin: bsp::pins::D6PwmF = pins.led.into();

    let mut pwm0 = Pwm0::new(
        &clocks.tcc0_tcc1(&gclk0).unwrap(),
        1u32.kHz(),
        peripherals.TCC0,
        &mut peripherals.PM,
    );
    let channel = Channel::_2;
    pwm0.enable(channel);

    let max_duty = pwm0.get_max_duty();
    loop {
        pwm0.set_duty(channel, max_duty);
        delay.delay_ms(500u16);
        pwm0.set_duty(channel, max_duty / 4);
        delay.delay_ms(500u16);
    }
}
