#![no_std]
#![no_main]

use metro_m4 as bsp;

use bsp::hal;
use bsp::pac;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::gpio::M;

use pac::gclk::genctrl::SRC_A::DPLL0;
use pac::gclk::pchctrl::GEN_A::GCLK2;
use pac::Peripherals;

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
    let pins = bsp::Pins::new(peripherals.PORT);

    // assuming 120MHz main clock, output a 10MHz clock on d7
    let _ = clocks
        .configure_gclk_divider_and_source(GCLK2, 12, DPLL0, false)
        .unwrap();
    let _ = pins.d7.into_alternate::<M>();
    loop {
        cortex_m::asm::wfi();
    }
}
