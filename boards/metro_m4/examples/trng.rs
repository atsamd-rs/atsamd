#![no_std]
#![no_main]

use metro_m4 as bsp;

use bsp::hal;
use bsp::pac;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use cortex_m_semihosting::hprintln;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::trng::Trng;
use pac::{CorePeripherals, Peripherals};

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
        hprintln!("{}", trng.random_u32()).ok();
        delay.delay_ms(1000u16);
    }
}
