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

    let pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let (tx, rx) = (pins.d1, pins.d0);
    let mut uart = bsp::uart(
        &mut clocks,
        Hertz(19200),
        peripherals.SERCOM5,
        &mut peripherals.MCLK,
        rx,
        tx,
    );

    loop {
        for byte in b"Hello, world!" {
            nb::block!(uart.write(*byte)).unwrap();
        }
        delay.delay_ms(1000u16);
    }
}
