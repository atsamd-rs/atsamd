//! This example showcases the i2c module, and uses DMA to perform I2C
//! transactions.

#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use metro_m0 as bsp;

use bsp::entry;
use bsp::hal;
use bsp::pac;

use cortex_m::asm;
use pac::Peripherals;

use hal::clock::GenericClockController;
use hal::dmac::{DmaController, PriorityLevel};
use hal::ehal::i2c::I2c;
use hal::fugit::RateExtU32;
use hal::sercom::i2c;

// This example is based on the BMP388 pressure sensor. Adjust the device and
// register addresses to your liking
const ADDRESS: u8 = 0x76;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    let mut pm = peripherals.pm;
    let dmac = peripherals.dmac;
    let pins = bsp::Pins::new(peripherals.port);

    // Take SDA and SCL
    let (sda, scl) = (pins.sda, pins.scl);

    // Setup DMA channels for later use
    let mut dmac = DmaController::init(dmac, &mut pm);
    let channels = dmac.split();
    let chan0 = channels.0.init(PriorityLevel::Lvl0);

    let gclk0 = clocks.gclk0();
    let sercom3_clock = &clocks.sercom3_core(&gclk0).unwrap();
    let pads = i2c::Pads::new(sda, scl);
    let mut i2c = i2c::Config::new(&pm, peripherals.sercom3, pads, sercom3_clock.freq())
        .baud(100.kHz())
        .enable()
        .with_dma_channel(chan0);

    let mut received = [0x00; 1];

    // Test writing then reading from an I2C chip
    i2c.write_read(ADDRESS, &[0x00; 8], &mut received).unwrap();

    loop {
        // Go to sleep
        asm::wfi();
    }
}
