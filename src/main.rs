#![feature(used)]
#![no_std]

extern crate atsamd21_hal as hal;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;

use core::fmt::Write;
use cortex_m_semihosting::hio;

use hal::atsamd21g18a::Peripherals;
use hal::prelude::*;

fn main() {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world!").unwrap();

    let mut peripherals = Peripherals::take().unwrap();

    let clock_config = hal::clock::Configuration::new();
    let clocks = clock_config.freeze(
        &mut peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = peripherals.PORT.split();

    // PA17 is wired to arduino digital pin 13 and is attached
    // to an LED on the adafruit boards.
    let mut red_led = pins.pa17.into_open_drain_output(&mut pins.port);

    writeln!(stdout, "configure timer").unwrap();
    let mut tc3 = hal::timer::TimerCounter3_16::new(clocks, peripherals.TC3);
    writeln!(stdout, "start timer").unwrap();
    tc3.start(5.hz());
    writeln!(stdout, "begin loop").unwrap();

    loop {
        if tc3.wait().is_ok() {
            red_led.toggle();
        }
    }
}
