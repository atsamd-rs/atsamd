#![no_std]
#![no_main]

use bsp::hal;
use metro_m4 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use crate::hal::clock::GenericClockController;
use crate::hal::pac::gclk::genctrl::SRC_A::DPLL0;
use crate::hal::pac::gclk::pchctrl::GEN_A::GCLK2;
use crate::hal::pac::Peripherals;
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
    let mut pins = bsp::Pins::new(peripherals.PORT);

    let _gclk2 = clocks
        .configure_gclk_divider_and_source(GCLK2, 12, DPLL0, false)
        .unwrap();
    pins.d13.into_function_m(&mut pins.port);
    loop {}
}
