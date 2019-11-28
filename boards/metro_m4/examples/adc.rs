#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate embedded_hal;
extern crate metro_m4 as hal;
extern crate panic_semihosting;

use core::fmt::Write;

use hal::adc::Adc;
use hal::clock::GenericClockController;
use hal::entry;
use hal::pac::gclk::pchctrl::GEN_A::GCLK11;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::sercom::{PadPin, Sercom3Pad0, Sercom3Pad1, UART3};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);
    let mut adc0 = Adc::adc0(peripherals.ADC0, &mut peripherals.MCLK, &mut clocks, GCLK11);
    let mut a0 = pins.a0.into_function_b(&mut pins.port);

    let gclk0 = clocks.gclk0();
    let rx: Sercom3Pad1<_> = pins
        .d0
        .into_pull_down_input(&mut pins.port)
        .into_pad(&mut pins.port);
    let tx: Sercom3Pad0<_> = pins
        .d1
        .into_pull_down_input(&mut pins.port)
        .into_pad(&mut pins.port);
    let uart_clk = clocks
        .sercom3_core(&gclk0)
        .expect("Could not configure sercom3 clock");

    let mut uart = UART3::new(
        &uart_clk,
        9600.hz(),
        peripherals.SERCOM3,
        &mut peripherals.MCLK,
        (rx, tx),
    );

    loop {
        let data: u16 = adc0.read(&mut a0).unwrap();
        uart.write_fmt(format_args!("{}\n", data)).unwrap();
        delay.delay_ms(1000u16);
    }
}
