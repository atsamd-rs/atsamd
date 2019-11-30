#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate embedded_hal;
extern crate samd11_bare as hal;

#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

use cortex_m_semihosting::hprintln;
use hal::adc::Adc;
use hal::clock::GenericClockController;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);
    let mut pins = hal::Pins::new(peripherals.PORT);

    let mut adc = Adc::adc(peripherals.ADC, &mut peripherals.PM, &mut clocks);
    let mut a0 = pins.d1.into_function_b(&mut pins.port);

    loop {
        let data: u16 = adc.read(&mut a0).unwrap();
        hprintln!("{}", data).unwrap();
        delay.delay_ms(1000u16);
    }
}
