//! Use a hardware timer to wiggle a pin in the background.

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, GclkOut, Pins};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use hal::clock::GenericClockController;
use pac::gclk::genctrl::Srcselect::Dpll0;
use pac::gclk::pchctrl::Genselect::Gclk2;
use pac::Peripherals;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = Pins::new(peripherals.port);

    // Output 3 MHz clock on pin d5
    let _gclk2 = clocks
        .configure_gclk_divider_and_source(Gclk2, 40, Dpll0, false)
        .unwrap();
    let _clock_out_pin: GclkOut = pins.d5.into();
    loop {
        cortex_m::asm::wfi();
    }
}
