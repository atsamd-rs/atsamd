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
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use pac::{CorePeripherals, Peripherals};

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
    let mut pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);
    let mut adc0 = Adc::adc0(peripherals.ADC0, &mut peripherals.MCLK, &mut clocks, GCLK11);
    let mut a0 = pins.a0.into_function_b(&mut pins.port);

    loop {
        let data: u16 = adc0.read(&mut a0).unwrap();
        hprintln!("{}", data).ok();
        delay.delay_ms(1000u16);
    }
}
