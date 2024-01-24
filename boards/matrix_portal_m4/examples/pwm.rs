#![no_std]
#![no_main]

use matrix_portal_m4::{entry, hal, Pins, RedLedPwm};
use panic_halt as _;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::fugit::RateExtU32;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::pwm::{Channel, TCC1Pinout, Tcc1Pwm};

/// Entry point for the PWM example on Matrix Portal M4.
///
/// This function sets up the necessary peripherals for PWM control of the
/// onboard LED. It uses TCC1 for PWM, setting different duty cycles to vary the
/// brightness of the LED. The LED brightness alternates between maximum and
/// minimum (1/8th of max) every second.
///
/// Note: The LED is connected to channel 2 of TCC1 as per the Adafruit Matrix
/// Portal M4 pinout.
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
    let pins = Pins::new(peripherals.PORT);
    let red_led: RedLedPwm = pins.led.into();

    let gclk0 = clocks.gclk0();
    let mut tcc1pwm = Tcc1Pwm::new(
        &clocks.tcc0_tcc1(&gclk0).unwrap(),
        1.kHz(),
        peripherals.TCC1,
        TCC1Pinout::Pa14(red_led),
        &mut peripherals.MCLK,
    );
    let max_duty = tcc1pwm.get_max_duty();
    let min_duty = max_duty / 8;

    loop {
        tcc1pwm.set_duty(Channel::_2, max_duty);
        delay.delay_ms(1000u16);
        tcc1pwm.set_duty(Channel::_2, min_duty);
        delay.delay_ms(1000u16);
    }
}
