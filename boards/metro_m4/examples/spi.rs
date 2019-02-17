#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate metro_m4 as hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;
extern crate nb;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::{entry, CorePeripherals, Peripherals};
use hal::sercom::PadPin;
use nb::block;

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
    let mut pins = hal::Pins::new(peripherals.PORT);
    let gclk = clocks.gclk0();

    let spi_pinout = hal::sercom::SPI3Pinout::Dipo1Dopo3 {
        miso: pins.d0.into_pad(&mut pins.port),
        mosi: pins.d1.into_pad(&mut pins.port),
        sck: pins.d8.into_pad(&mut pins.port),
    };

    let mut spi = hal::sercom::SPIMaster3::new(
        &clocks.sercom3_core(&gclk).unwrap(),
        3_000_000u32.hz(),
        embedded_hal::spi::Mode {
            polarity: embedded_hal::spi::Polarity::IdleLow,
            phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
        },
        peripherals.SERCOM3,
        &mut peripherals.MCLK,
        spi_pinout,
    );

    loop {
        for byte in b"Hello, world!" {
            block!(spi.send(*byte)).unwrap();
        }
        delay.delay_ms(1000u16);
    }

}
