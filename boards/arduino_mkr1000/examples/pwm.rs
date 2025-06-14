#![no_std]
#![no_main]

use arduino_mkr1000 as bsp;
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
    let pins = bsp::pins::Pins::new(peripherals.PORT);

    // PWM0_CH0 is D11 on the board - pin PA08
    let _d11 = pins.pa08.into_mode::<AlternateE>();
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let gclk0 = clocks.gclk0();
    let tcc0_tcc1_clock: &Tcc0Tcc1Clock = &clocks.tcc0_tcc1(&gclk0).unwrap();

    let mut pwm0 = Pwm0::new(
        &tcc0_tcc1_clock,
        1u32.kHz(),
        peripherals.TCC0,
        &mut peripherals.PM,
    );
    let max_duty = pwm0.get_max_duty();
    let channel = Channel::_0;
    pwm0.enable(channel);
    loop {
        pwm0.set_duty(channel, max_duty);
        delay.delay_ms(500u16);
        pwm0.set_duty(channel, max_duty / 4);
        delay.delay_ms(500u16);
    }
}
