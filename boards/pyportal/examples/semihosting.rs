#![no_main]
#![no_std]

use bsp::hal;
use pyportal as bsp;

use panic_semihosting as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!");

    loop {}
}
