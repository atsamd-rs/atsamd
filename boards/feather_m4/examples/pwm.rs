#![no_std]
#![no_main]

// Pulse Width Modulation
//
// cargo build --features="unproven"

use bsp::hal;
use feather_m4 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use cortex_m_rt::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::pwm::Pwm4;

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
    let red_led: bsp::RedLedPwm = pins.d13.into();

    let gclk0 = clocks.gclk0();
    let mut pwm4 = Pwm4::new(
        &clocks.tc4_tc5(&gclk0).unwrap(),
        1.khz(),
        peripherals.TC4,
        hal::pwm::TC4Pinout::Pa23(red_led),
        &mut peripherals.MCLK,
    );
    let max_duty = pwm4.get_max_duty();

    // The red led will light up and toggle between 2 brightness
    // levels every second.
    loop {
        pwm4.set_duty(max_duty / 2);
        delay.delay_ms(1000u16);
        pwm4.set_duty(max_duty / 8);
        delay.delay_ms(1000u16);
    }
}
