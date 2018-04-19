#![feature(used)]
#![no_std]

extern crate atsamd21_hal as hal;
extern crate cortex_m;
extern crate cortex_m_rt;

#[cfg(feature = "use_semihosting")]
extern crate cortex_m_semihosting;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_abort;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

#[cfg(feature = "use_semihosting")]
macro_rules! dbgprint {
    ($($expr:expr),+) => {
        {
            use cortex_m_semihosting::hio;
            use core::fmt::Write;
            let mut stdout = hio::hstdout().unwrap();
            writeln!(stdout, $($expr)*).unwrap();
        }
    };
}

#[cfg(not(feature = "use_semihosting"))]
macro_rules! dbgprint {
    ($($expr:expr),+) => {};
}

use hal::atsamd21g18a::Peripherals;
use hal::prelude::*;

fn main() {
    dbgprint!("Hello, world!");

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

    dbgprint!("configure timer");
    let mut tc3 = hal::timer::TimerCounter3::new(clocks, peripherals.TC3);
    dbgprint!("start timer");
    tc3.start(5.hz());
    dbgprint!("begin loop");

    loop {
        if tc3.wait().is_ok() {
            red_led.toggle();
        }
    }
}
