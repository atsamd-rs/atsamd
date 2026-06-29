#![no_std]
#![no_main]

use atsamd_hal::adc::AdcBuilder;
use atsamd_hal::clock::v2::clock_system_at_reset;
use atsamd_hal::clock::v2::pclk::Pclk;
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
    adc::{Accumulation, Prescaler},
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let _core = CorePeripherals::take().unwrap();

    let pins = Pins::new(peripherals.port);


    let (_buses, clocks, tokens) = clock_system_at_reset(peripherals.gclk, peripherals.pm, peripherals.sysctrl);
    let (adc_pclk, _gclk0) = Pclk::enable(tokens.pclks.adc, clocks.gclk0);
    let adc_apb = clocks.apbs.adc0;
    
    let mut adc = AdcBuilder::new(Accumulation::single(atsamd_hal::adc::AdcResolution::_12))
        .with_clock_cycles_per_sample(5)
        .with_clock_divider(Prescaler::Div128)
        .with_vref(atsamd_hal::adc::Reference::Arefa)
        .enable(peripherals.adc, adc_apb, &adc_pclk)
        .unwrap();
    let mut adc_pin = pins.d1.into_alternate();

    loop {
        let res = adc.read(&mut adc_pin);
        #[cfg(feature = "use_semihosting")]
        cortex_m_semihosting::hprintln!("ADC value: {}", res);
    }
}
