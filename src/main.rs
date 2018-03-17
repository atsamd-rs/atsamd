#![feature(used)]
#![no_std]

extern crate atsamd21g18a;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

use core::fmt::Write;
use cortex_m_semihosting::hio;

fn main() {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world!").unwrap();
}
