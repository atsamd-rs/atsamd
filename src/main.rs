#![feature(used)]
#![no_std]

extern crate atsamd21_hal as hal;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

use core::fmt::Write;
use cortex_m_semihosting::hio;

use hal::atsamd21g18a::Peripherals;
use hal::prelude::*;

fn main() {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world!").unwrap();

    let mut peripherals = Peripherals::take().unwrap();

    let clock_config = hal::clock::Configuration::new();
    let _clocks = clock_config.freeze(
        &mut peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = peripherals.PORT.split();

    // PA17 is wired to arduino digital pin 13 and is attached
    // to an LED on the adafruit boards.
    let mut red_led = pins.pa17.into_open_drain_output(&mut pins.port);
    red_led.set_high();
}
