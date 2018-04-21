#![feature(used)]
#![no_std]

extern crate atsamd21_hal as hal;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate sx1509;

#[cfg(feature = "use_semihosting")]
extern crate cortex_m_semihosting;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_abort;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

#[cfg(feature = "use_semihosting")]
macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use cortex_m_semihosting::hio;
            use core::fmt::Write;
            let mut stdout = hio::hstdout().unwrap();
            writeln!(stdout, $($arg)*).ok();
        }
    };
}

#[cfg(not(feature = "use_semihosting"))]
macro_rules! dbgprint {
    ($($fmt:expr),+) => {};
}

use hal::atsamd21g18a::Peripherals;
use hal::prelude::*;
use hal::sercom::{I2CMaster3, Sercom3Pad0, Sercom3Pad1};

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

    dbgprint!("init i2c");
    let mut i2c = I2CMaster3::new(
        &clocks,
        400.khz(),
        peripherals.SERCOM3,
        &mut peripherals.PM,
        &mut peripherals.GCLK,
        // Metro M0 express has I2C on pins PA22, PA23
        Sercom3Pad0::Pa22(pins.pa22.into_function_c(&mut pins.port)),
        Sercom3Pad1::Pa23(pins.pa23.into_function_c(&mut pins.port)),
    );
    let mut expander = sx1509::Sx1509::new(&mut i2c, sx1509::DEFAULT_ADDRESS);

    dbgprint!("do first write");
    // Let's try to init an sx1509 attached to the i2c bus
    let res1 = expander.software_reset();
    dbgprint!("send reset {:?}", res1.is_ok());

    let res3 = expander.read_16(sx1509::Register::RegInterruptMaskA);
    match res3 {
        Err(e) => dbgprint!("read intmaska fail {:?}", e),
        Ok(val) => dbgprint!("read intmaska {:x}", val),
    };

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
