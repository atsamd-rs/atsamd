#![no_std]
#![no_main]

extern crate panic_halt;
extern crate metro_m4 as hal;
extern crate embedded_hal;
extern crate cortex_m_rt;
extern crate nb;

use crate::hal::clock::GenericClockController;
use crate::hal::pac::Peripherals;
use crate::hal::pac::gclk::genctrl::SRC_A::DPLL0;
use crate::hal::pac::gclk::pchctrl::GEN_A::GCLK2;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
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
