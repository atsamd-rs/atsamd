//! Use a hardware timer to wiggle a pin in the background.

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, GclkOut, Pins};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use hal::clock::GenericClockController;
use pac::gclk::genctrl::SRCSELECT_A::DPLL0;
use pac::gclk::pchctrl::GENSELECT_A::GCLK2;
use pac::Peripherals;

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
    let pins = Pins::new(peripherals.PORT);

    // Output 3 MHz clock on pin d5
    let _gclk2 = clocks
        .configure_gclk_divider_and_source(GCLK2, 40, DPLL0, false)
        .unwrap();
    let _clock_out_pin: GclkOut = pins.d5.into();
    loop {}
}
