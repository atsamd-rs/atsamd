#![no_std]
#![no_main]

use atsamd_hal::adc::AdcBuilder;
use samd11_bare as bsp;

use bsp::hal;
use bsp::pac;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use bsp::Pins;
use pac::{CorePeripherals, Peripherals};

use hal::{
    adc::{Accumulation, Adc, Prescaler, Resolution},
    clock::GenericClockController,
};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let _core = CorePeripherals::take().unwrap();

    let pins = Pins::new(peripherals.port);

    // SAMD11 targets currently don't support clock::v2
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let gclk0 = clocks.gclk0();
    let adc_clock = clocks.adc(&gclk0).unwrap();

    let mut adc = AdcBuilder::new(Accumulation::single(atsamd_hal::adc::AdcResolution::_12))
        .with_clock_cycles_per_sample(5)
        // Overruns if clock divider < 128 in debug mode
        .with_clock_divider(Prescaler::Div128)
        .enable(peripherals.adc,&mut peripherals.pm,&adc_clock)
        .unwrap();

    let mut adc_pin = pins.d1.into_alternate();

    loop {
        let mut buffer = [0; 16];
        let res = adc.read_buffer(&mut adc_pin, &mut buffer).unwrap();
        #[cfg(feature = "use_semihosting")]
        cortex_m_semihosting::hprintln!("Result: {:?}", res);
    }
}
