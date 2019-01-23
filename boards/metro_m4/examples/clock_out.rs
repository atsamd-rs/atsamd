#![no_std]
#![no_main]

extern crate panic_halt;
extern crate metro_m4 as hal;
extern crate embedded_hal;
extern crate cortex_m_rt;
extern crate nb;

use crate::hal::clock::GenericClockController;
use crate::hal::{Peripherals, CorePeripherals};
use crate::hal::gclk::genctrl::SRCR::{DFLL, DPLL0, XOSC32K, GCLKGEN1};
use crate::hal::gclk::pchctrl::GENR::{GCLK1, GCLK2};
use hal::prelude::*;
use cortex_m_rt::entry;

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
    let mut pins = hal::Pins::new(peripherals.PORT);

    let _gclk2 = clocks
        .configure_gclk_divider_and_source(GCLK2, 12, DPLL0, false)
        .unwrap();
    pins.d13.into_function_m(&mut pins.port);
    loop {}
}
