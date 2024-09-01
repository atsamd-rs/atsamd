#![no_std]
#![no_main]

// Pulse Width Modulation

use bsp::hal;
use feather_m4 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use cortex_m_rt::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::fugit::RateExtU32;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::pwm::Pwm4;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);
    let pins = bsp::Pins::new(peripherals.port);
    let red_led: bsp::RedLedPwm = pins.d13.into();

    let gclk0 = clocks.gclk0();
    let mut pwm4 = Pwm4::new(
        &clocks.tc4_tc5(&gclk0).unwrap(),
        1.kHz(),
        peripherals.tc4,
        hal::pwm::TC4Pinout::Pa23(red_led),
        &mut peripherals.mclk,
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
