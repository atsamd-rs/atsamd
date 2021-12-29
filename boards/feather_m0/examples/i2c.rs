//! This example showcases the i2c::v2 module.

#![no_std]
#![no_main]

use cortex_m::asm;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use feather_m0 as bsp;

use bsp::entry;
use bsp::hal;
use bsp::pac;

use hal::clock::GenericClockController;
use hal::dmac::{DmaController, PriorityLevel};
use hal::ehal::blocking::i2c::WriteRead;
use hal::prelude::*;
use hal::sercom::v2::{i2c, Sercom3};

use pac::Peripherals;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pm = peripherals.PM;
    let dmac = peripherals.DMAC;
    let pins = bsp::Pins::new(peripherals.PORT);

    // Take SDA and SCL
    let (sda, scl) = (pins.sda, pins.scl);

    // Setup DMA channels for later use
    let mut dmac = DmaController::init(dmac, &mut pm);
    let channels = dmac.split();

    let chan0 = channels.0.init(PriorityLevel::LVL0);
    let chan1 = channels.1.init(PriorityLevel::LVL0);

    type Pads = i2c::Pads<Sercom3, bsp::Sda, bsp::Scl>;

    let gclk0 = clocks.gclk0();
    let sercom3_clock = &clocks.sercom3_core(&gclk0).unwrap();
    let pads: Pads = i2c::Pads::new(sda, scl);
    let mut i2c = i2c::Config::new(&pm, peripherals.SERCOM3, pads, sercom3_clock.freq())
        .baud(100.khz())
        .enable();

    let mut buffer = [0; 1];

    // Test writing then reading from an I2C chip
    i2c.write_read(0x77, &[0x00], &mut buffer).unwrap();

    loop {
        // Go to sleep
        asm::wfi();
    }
}
