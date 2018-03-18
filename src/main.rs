#![feature(used)]
#![no_std]

extern crate atsamd21_hal;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

use core::fmt::Write;
use cortex_m_semihosting::hio;

use atsamd21_hal::atsamd21g18a::Peripherals;

fn main() {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world!").unwrap();

    let peripherals = Peripherals::take().unwrap();

    // Bit mask for the red LED.  It is attached to
    // pin A17 and is known as digital 13 in arduino land.
    const DIGITAL_13: u32 = 1 << 17;

    // Set the LED to output mode
    peripherals.PORT.dirset0.write(|bits| unsafe {
        bits.bits(DIGITAL_13);
        bits
    });

    // Turn on the LED
    peripherals.PORT.outset0.write(|bits| unsafe {
        bits.bits(DIGITAL_13);
        bits
    });
}
