#![no_std]
#![no_main]

use metro_m4 as bsp;

use bsp::hal;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{entry, periph_alias};
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

    let mut delay = Delay::new(core.SYST, &mut clocks);
    let pins = bsp::Pins::new(peripherals.PORT);

    let miso = pins.miso;
    let mosi = pins.mosi;
    let sclk = pins.sclk;
    let spi_sercom = periph_alias!(peripherals.spi_sercom);
    let mclk = &mut peripherals.MCLK;

    let mut spi = bsp::spi_master(&mut clocks, 3.mhz(), spi_sercom, mclk, sclk, mosi, miso);

    loop {
        for byte in b"Hello, world!" {
            nb::block!(spi.send(*byte)).unwrap();
        }
        delay.delay_ms(1000u16);
    }
}
