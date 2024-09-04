//! Blink an led without using the BSP split() method.

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, Pins};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use core::f32::consts::FRAC_PI_2;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::gpio::v2::AlternateG;
use hal::prelude::*;
use hal::pwm::{Channel, TCC0Pinout, Tcc0Pwm};
use micromath::F32Ext;
use pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);
    delay.delay_ms(400u16);

    let pins = Pins::new(peripherals.PORT);

    let gclk = clocks.gclk0();

    let mut pwm0 = Tcc0Pwm::new(
        &clocks.tcc0_tcc1(&gclk).unwrap(),
        1.khz(),
        peripherals.TCC0,
        TCC0Pinout::Pa23(pins.d13.into_mode::<AlternateG>()),
        &mut peripherals.MCLK,
    );
    let max_duty = pwm0.get_max_duty();

    loop {
        for j in 0..255 {
            pwm0.set_duty(
                Channel::_3,
                sine_ease_in(j as f32, 0.0, max_duty as f32, 255.0) as u32,
            );
            delay.delay_ms(10u16);
        }
        for j in (0..255).rev() {
            pwm0.set_duty(
                Channel::_3,
                sine_ease_in(j as f32, 0.0, max_duty as f32, 255.0) as u32,
            );
            delay.delay_ms(10u16);
        }
    }
}

#[inline]
// current step, where oputput starts, where output ends, last step
fn sine_ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    -c * (t / d * FRAC_PI_2).cos() + c + b
}
