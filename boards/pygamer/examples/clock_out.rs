//! Use a hardware timer to wiggle a pin in the background.

#![no_std]
#![no_main]

use panic_halt as _;
use pygamer as hal;

use hal::clock::GenericClockController;
use hal::entry;
use hal::pac::gclk::genctrl::SRC_A::DPLL0;
use hal::pac::gclk::pchctrl::GEN_A::GCLK2;
use hal::pac::Peripherals;

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
