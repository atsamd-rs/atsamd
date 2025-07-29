#![no_std]
#![no_main]

use atsamd_hal::adc::AdcBuilder;
use feather_m4 as bsp;

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
    clock::v2::{clock_system_at_reset, pclk::Pclk},
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let _core = CorePeripherals::take().unwrap();

    let pins = Pins::new(peripherals.port);

    let (mut buses, clocks, tokens) = clock_system_at_reset(
        peripherals.oscctrl,
        peripherals.osc32kctrl,
        peripherals.gclk,
        peripherals.mclk,
    );

    // Enable the ADC0 ABP clock...
    let apb_adc0 = buses.apb.enable(tokens.apbs.adc0);
    // ...and enable the ADC0 PCLK. Both of these are required for the
    // ADC to run.
    let (pclk_adc0, _gclk0) = Pclk::enable(tokens.pclks.adc0, clocks.gclk0);

    let mut adc = AdcBuilder::new(Accumulation::single(atsamd_hal::adc::AdcResolution::_12))
        .with_clock_cycles_per_sample(5)
        // Overruns if clock divider < 32 in debug mode
        .with_clock_divider(Prescaler::Div32)
        .with_vref(atsamd_hal::adc::Reference::Arefa)
        .enable(peripherals.adc0, apb_adc0, &pclk_adc0)
        .unwrap();
    let mut adc_pin = pins.a0.into_alternate();

    loop {
        let res = adc.read(&mut adc_pin);
        #[cfg(feature = "use_semihosting")]
        cortex_m_semihosting::hprintln!("ADC value: {}", res);
    }
}
