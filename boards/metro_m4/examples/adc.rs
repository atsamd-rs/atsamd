#![no_std]
#![no_main]

extern crate panic_semihosting;
extern crate cortex_m_rt;
extern crate metro_m4 as hal;
extern crate embedded_hal;

use core::fmt::Write;

use hal::prelude::*;
use hal::clock::GenericClockController;
use hal::{Peripherals, CorePeripherals};
use hal::adc::Adc;
use hal::sercom::{PadPin, Sercom3Pad0, Sercom3Pad1, UART3Pinout, UART3};

use embedded_hal::adc::OneShot;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);
    let mut adc0 = Adc::new(peripherals.ADC0, &mut peripherals.MCLK, &mut clocks);
    let mut a0 = pins.a0.into_function_b(&mut pins.port);

    let gclk0 = clocks.gclk0();
    let rx: Sercom3Pad1 = pins
        .d0
        .into_pull_down_input(&mut pins.port)
        .into_pad(&mut pins.port);
    let tx: Sercom3Pad0 = pins
        .d1
        .into_pull_down_input(&mut pins.port)
        .into_pad(&mut pins.port);
    let uart_clk = clocks.sercom3_core(&gclk0)
        .expect("Could not configure sercom3 clock");

    let mut uart = UART3::new(
        &uart_clk,
        9600.hz(),
        peripherals.SERCOM3,
        &mut core.NVIC,
        &mut peripherals.MCLK,
        UART3Pinout::Rx1Tx0 {
            rx: rx,
            tx: tx,
        },
    );

    loop {
        let data: u16 = adc0.read(&mut a0).unwrap();
        uart.write_fmt(format_args!("{}\n", data)).unwrap();
        delay.delay_ms(1000u16);
    }
}
