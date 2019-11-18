#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate embedded_hal;
extern crate nb;
extern crate panic_halt;
extern crate pygamer as hal;

use crate::hal::clock::GenericClockController;
use crate::hal::pac::gclk::genctrl::SRC_A::DPLL0;
use crate::hal::pac::gclk::pchctrl::GEN_A::GCLK2;
use crate::hal::pac::Peripherals;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);

    //3mhz
    let _gclk2 = clocks
        .configure_gclk_divider_and_source(GCLK2, 40, DPLL0, false)
        .unwrap();
    pins.d5.into_function_m(&mut pins.port);
    loop {}
}
