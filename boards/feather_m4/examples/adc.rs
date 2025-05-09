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
    adc::{Accumulation, Adc, Prescaler, Resolution},
    clock::v2::{clock_system_at_reset, pclk::Pclk},
};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let _core = CorePeripherals::take().unwrap();

    let pins = Pins::new(peripherals.port);

    // TODO: currently this example only runs at the chip's
    // 48 MHz CPU frequency at reset. There is currently a
    // bug with the clock::v2 module that affects speeding
    // up the CPU clock to a nominal 100 or 120 MHz. The bug
    // is currently under investigation, and this example should
    // be updated accordingly when it's fixed.
    let (mut buses, clocks, tokens) = clock_system_at_reset(
        peripherals.oscctrl,
        peripherals.osc32kctrl,
        peripherals.gclk,
        peripherals.mclk,
        &mut peripherals.nvmctrl,
    );

    // Enable the ADC0 ABP clock...
    let apb_adc0 = buses.apb.enable(tokens.apbs.adc0);
    // ...and enable the ADC0 PCLK. Both of these are required for the
    // ADC to run.
    let (pclk_adc0, _gclk0) = Pclk::enable(tokens.pclks.adc0, clocks.gclk0);

    let mut adc = AdcBuilder::new(Accumulation::single(atsamd_hal::adc::AdcResolution::_12))
        .with_clock_cycles_per_sample(5)
        .with_clock_divider(Prescaler::Div32)
        .with_vref(atsamd_hal::adc::Reference::Arefa)
        .enable(peripherals.adc0, apb_adc0, &pclk_adc0)
        .unwrap();
    let mut adc_pin = pins.a0.into_alternate();

    loop {
        let mut buffer = [0; 16];
        let res = adc.read_buffer_blocking(&mut adc_pin, &mut buffer).unwrap();
        #[cfg(feature = "use_semihosting")]
        cortex_m_semihosting::hprintln!("Result: {:?}", res);
    }
}
