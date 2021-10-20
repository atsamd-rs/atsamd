#![no_std]
#![no_main]

use metro_m4 as bsp;

use bsp::hal;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

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

    let pins = bsp::Pins::new(peripherals.PORT);
    let (rx_pin, tx_pin) = (pins.d0, pins.d1);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut uart = bsp::uart(
        &mut clocks,
        9600.hz(),
        peripherals.SERCOM3,
        &mut peripherals.MCLK,
        rx_pin,
        tx_pin,
    );

    loop {
        for byte in b"Hello, world!" {
            // NOTE `block!` blocks until `uart.write()` completes and returns
            // `Result<(), Error>`
            nb::block!(uart.write(*byte)).unwrap();
        }
        delay.delay_ms(1000u16);
    }
}
