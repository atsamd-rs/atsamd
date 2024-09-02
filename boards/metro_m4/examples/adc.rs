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
use hal::adc::Adc;
use hal::clock::GenericClockController;
use hal::gpio::B;
use hal::prelude::*;
use pac::gclk::pchctrl::Genselect::Gclk11;
use pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port);
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);
    let mut adc0 = Adc::adc0(peripherals.adc0, &mut peripherals.mclk, &mut clocks, Gclk11);
    let mut a0 = pins.a0.into_alternate::<B>();

    loop {
        let data: u16 = adc0.read(&mut a0).unwrap();
        hprintln!("{}", data).ok();
        delay.delay_ms(1000u16);
    }
}
