#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use bsp::pac;
use bsp::{hal, pin_alias};
use feather_m0 as bsp;
use hal::{
    clock::{enable_internal_32kosc, ClockGenId, ClockSource, GenericClockController},
    ehal::digital::StatefulOutputPin,
    eic::{Eic, Sense},
    gpio::{Pin, PullUpInterrupt},
};

atsamd_hal::bind_interrupts!(struct Irqs {
    EIC => atsamd_hal::eic::InterruptHandler;
});

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    let _internal_clock = clocks
        .configure_gclk_divider_and_source(ClockGenId::Gclk2, 1, ClockSource::Osc32k, false)
        .unwrap();
    clocks.configure_standby(ClockGenId::Gclk2, true);

    enable_internal_32kosc(&mut peripherals.sysctrl);

    // Configure a clock for the EIC peripheral
    let gclk2 = clocks.get_gclk(ClockGenId::Gclk2).unwrap();
    let eic_clock = clocks.eic(&gclk2).unwrap();

    let eic_channels = Eic::new(&mut peripherals.pm, eic_clock, peripherals.eic)
        .into_future(Irqs)
        .split();

    let button: Pin<_, PullUpInterrupt> = pins.d10.into();
    let mut extint = eic_channels.2.with_pin(button);

    loop {
        // Here we show straight falling edge detection without
        extint.wait(Sense::Fall).await;
        defmt::info!("Falling edge detected");
        red_led.toggle().unwrap();
    }
}
