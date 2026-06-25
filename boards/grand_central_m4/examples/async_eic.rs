//! Uses an external interrupt to blink an LED.
//!
//! You need to connect a button between D46 and ground. Each time the button
//! is pressed, the LED will toggle.
#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use bsp::hal;
use bsp::pac;
use bsp::pin_alias;
use grand_central_m4 as bsp;

use hal::{
    clock::{ClockGenId, ClockSource, GenericClockController},
    eic::{Eic, Sense},
    gpio::{Pin, PullUpInterrupt},
    prelude::*,
};
use pac::{CorePeripherals, Peripherals};

atsamd_hal::bind_interrupts!(struct Irqs {
    EIC_EXTINT_6 => atsamd_hal::eic::InterruptHandler;
});

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut _core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    let _internal_clock = clocks
        .configure_gclk_divider_and_source(ClockGenId::Gclk2, 1, ClockSource::Osculp32k, false)
        .unwrap();
    clocks.configure_standby(ClockGenId::Gclk2, true);

    // Configure a clock for the EIC peripheral
    let gclk2 = clocks.get_gclk(ClockGenId::Gclk2).unwrap();
    let eic_clock = clocks.eic(&gclk2).unwrap();

    let eic_channels = Eic::new(&mut peripherals.mclk, eic_clock, peripherals.eic).split();

    let button: Pin<_, PullUpInterrupt> = pins.d46.into();
    let mut extint = eic_channels.6.with_pin(button).into_future(Irqs);
    extint.enable_interrupt();

    loop {
        // Here we show straight falling edge detection without
        extint.wait(Sense::Fall).await;
        defmt::info!("Falling edge detected");
        red_led.toggle().unwrap();
    }
}
