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

use pac::gclk::genctrl::Srcselect::Dpll0;
use pac::gclk::pchctrl::Genselect::Gclk2;
use pac::Peripherals;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port);

    // assuming 120MHz main clock, output a 10MHz clock on d7
    let _ = clocks
        .configure_gclk_divider_and_source(Gclk2, 12, Dpll0, false)
        .unwrap();
    let _ = pins.d7.into_alternate::<M>();
    loop {
        cortex_m::asm::wfi();
    }
}
