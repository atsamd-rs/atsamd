#![no_std]
#![no_main]

use bsp::hal;
use p1am_100 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use cortex_m_semihosting::hprintln;
use hal::adc::Adc;
use hal::clock::GenericClockController;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);
    let mut adc = Adc::adc(peripherals.ADC, &mut peripherals.PM, &mut clocks);
    let mut a0: bsp::A0 = pins.d15.into();

    loop {
        let data: u16 = adc.read(&mut a0).unwrap();
        hprintln!("{}", data).ok();
        delay.delay_ms(1000u16);
    }
}
