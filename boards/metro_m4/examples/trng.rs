#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate embedded_hal;
extern crate metro_m4 as hal;
extern crate panic_semihosting;

use cortex_m_semihosting::hprintln;

use hal::clock::GenericClockController;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::trng::Trng;

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
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);
    let trng = Trng::new(&mut peripherals.MCLK, peripherals.TRNG);

    loop {
        hprintln!("{}", trng.random_u32()).unwrap();
        delay.delay_ms(1000u16);
    }
}
