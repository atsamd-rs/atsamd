#![no_std]
#![no_main]

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
    adc::{Accumulation, Adc, Adc0, Config, Prescaler, Resolution},
    clock::GenericClockController,
};

atsamd_hal::bind_interrupts!(struct Irqs {
    ADC => atsamd_hal::adc::InterruptHandler<Adc0>;
});

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let _core = CorePeripherals::take().unwrap();

    let pins = Pins::new(peripherals.port);

    // SAMD21 targets currently don't support clock::v2
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let gclk0 = clocks.gclk0();
    let adc_clock = clocks.adc(&gclk0).unwrap();

    let adc_settings = Config::new()
        .clock_cycles_per_sample(5)
        // Overruns if clock divider < 128 in debug mode
        .clock_divider(Prescaler::Div128)
        .sample_resolution(Resolution::_12bit)
        .accumulation_method(Accumulation::Single);

    let mut adc = Adc::new(
        peripherals.adc,
        adc_settings,
        &mut peripherals.pm,
        &adc_clock,
    )
    .unwrap()
    .into_future(Irqs);
    let mut adc_pin = pins.a0.into_alternate();

    loop {
        let mut buffer = [0; 16];
        let res = adc.read_buffer(&mut adc_pin, &mut buffer).await.unwrap();
        #[cfg(feature = "use_semihosting")]
        cortex_m_semihosting::hprintln!("Result: {:?}", res).unwrap();
    }
}
