#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use bsp::pac;
use bsp::{hal, pin_alias};
use hal::{
    clock::{ClockGenId, ClockSource, GenericClockController},
    ehal::digital::StatefulOutputPin,
    eic::{
        self,
        pin::{ExtInt7, Sense},
    },
    gpio::{Pin, PullUpInterrupt},
};
use metro_m4 as bsp;

atsamd_hal::bind_interrupts!(struct Irqs {
    EIC_EXTINT_7 => atsamd_hal::eic::InterruptHandler;
});

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    let _internal_clock = clocks
        .configure_gclk_divider_and_source(ClockGenId::GCLK2, 1, ClockSource::OSCULP32K, false)
        .unwrap();
    clocks.configure_standby(ClockGenId::GCLK2, true);

    // Configure a clock for the EIC peripheral
    let gclk2 = clocks.get_gclk(ClockGenId::GCLK2).unwrap();
    let eic_clock = clocks.eic(&gclk2).unwrap();

    let mut eic =
        eic::init_with_ulp32k(&mut peripherals.MCLK, eic_clock, peripherals.EIC).finalize();
    // let mut eic = EIC::init(&mut peripherals.MCLK, eic_clock,
    // peripherals.EIC).into_future(Irqs);
    let button: Pin<_, PullUpInterrupt> = pins.d0.into();
    let mut extint = ExtInt7::new(button, &mut eic).into_future(Irqs);
    extint.enable_interrupt();

    loop {
        // Here we show straight falling edge detection without
        extint.wait(Sense::FALL).await;
        defmt::info!("Falling edge detected");
        red_led.toggle().unwrap();
    }
}
