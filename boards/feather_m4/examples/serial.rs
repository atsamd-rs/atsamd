#![no_std]
#![no_main]

use bsp::hal;
use feather_m4 as bsp;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::gclk::genctrl::SRC_A;
use hal::pac::gclk::pchctrl::GEN_A;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::sercom::{PadPin, Sercom5Pad0, Sercom5Pad1, UART5};
use hal::time::Hertz;

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
    clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL, false);
    let gclk2 = clocks
        .get_gclk(GEN_A::GCLK2)
        .expect("Could not get clock 2");

    let mut pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let tx: Sercom5Pad0<_> = pins
        .d1
        .into_pull_down_input(&mut pins.port)
        .into_pad(&mut pins.port);
    let rx: Sercom5Pad1<_> = pins
        .d0
        .into_pull_down_input(&mut pins.port)
        .into_pad(&mut pins.port);
    let uart_clk = clocks
        .sercom5_core(&gclk2)
        .expect("Could not configure sercom5 clock");

    let mut uart = UART5::new(
        &uart_clk,
        Hertz(19200),
        peripherals.SERCOM5,
        &mut peripherals.MCLK,
        (rx, tx),
    );

    loop {
        for byte in b"Hello, world!" {
            nb::block!(uart.write(*byte)).unwrap();
        }
        delay.delay_ms(1000u16);
    }
}
