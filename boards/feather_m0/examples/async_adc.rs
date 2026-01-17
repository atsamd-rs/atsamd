#![no_std]
#![no_main]

use atsamd_hal::adc::AdcBuilder;
use feather_m0 as bsp;

use bsp::hal;
use bsp::pac;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::Pins;
use pac::{CorePeripherals, Peripherals};

use hal::{
    adc::{Accumulation, Adc0, Prescaler},
    clock::v2::{
        self as clock,
        dfll::Dfll,
        gclk::{Gclk, GclkDiv8},
        pclk::Pclk,
    },
};

atsamd_hal::bind_interrupts!(struct Irqs {
    ADC => atsamd_hal::adc::InterruptHandler<Adc0>;
});

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) -> ! {
    let device = Peripherals::take().unwrap();
    let _core = CorePeripherals::take().unwrap();

    let pins = Pins::new(device.port);

    // IMPORTANT - If reclocking the CPU itself, set wait state to half to avoid
    // hard faults.
    device
        .nvmctrl
        .ctrlb()
        .modify(|_, w| w.rws().variant(pac::nvmctrl::ctrlb::Rwsselect::Half));

    // --- Clocks setup ---
    let (_buses, clocks, tokens) =
        clock::clock_system_at_reset(device.gclk, device.pm, device.sysctrl);

    // We will use the internal 8 MHz oscillator on GCLK3 to clock the CPU
    //
    // Clock GCLK3 down from 1Mhz to 10000Hz (Div 100), this gives us a
    // clean factor of 48Mhz for DFLL to use
    let (gclk3, osc) = Gclk::from_source(tokens.gclks.gclk3, clocks.osc);
    let gclk3_10k = gclk3.div(GclkDiv8::Div(100)).enable();

    let (pclk_dfll, _gclk3_10k) = Pclk::enable(tokens.pclks.dfll, gclk3_10k);
    // Start the DFLL at 48Mhz
    let dfll_48m = Dfll::from_pclk(tokens.dfll, pclk_dfll).enable();
    // Swap CPU clock source
    let (gclk0_48, _osc, _dfll_48m) = clocks.gclk0.swap_sources(osc, dfll_48m);

    // --- ADC Configuration ---
    let (adc_pclk, _gclk0_48) = Pclk::enable(tokens.pclks.adc, gclk0_48);
    let adc_apb = clocks.apbs.adc0;

    let mut adc = AdcBuilder::new(Accumulation::single(atsamd_hal::adc::AdcResolution::_12))
        .with_clock_cycles_per_sample(5)
        .with_clock_divider(Prescaler::Div128)
        .with_vref(atsamd_hal::adc::Reference::Intvcc0)
        .enable(device.adc, adc_apb, adc_pclk)
        .unwrap()
        .into_future(Irqs);
    let mut adc_pin = pins.a0.into_alternate();

    loop {
        let _res = adc.read(&mut adc_pin).await;
        #[cfg(feature = "use_semihosting")]
        cortex_m_semihosting::hprintln!("ADC Result: {}", _res).unwrap();
    }
}
