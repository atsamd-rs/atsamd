//! This example showcases the spi module, and uses DMA to perform SPI
//! transactions.

#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use feather_m0 as bsp;

use bsp::entry;
use bsp::hal;
use bsp::pac;

use pac::Peripherals;

use hal::clock::GenericClockController;
use hal::dmac::{DmaController, PriorityLevel};
use hal::ehal::spi::SpiBus;
use hal::fugit::RateExtU32;

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

    // Take SPI pins
    let (miso, mosi, sclk) = (pins.miso, pins.mosi, pins.sclk);

    // Setup DMA channels for later use
    let mut dmac = DmaController::init(dmac, &mut pm);
    let channels = dmac.split();
    let chan0 = channels.0.init(PriorityLevel::Lvl0);
    let chan1 = channels.1.init(PriorityLevel::Lvl0);

    // Create a Spi with DMA enabled
    let mut spi = bsp::spi_master(
        &mut clocks,
        100.kHz(),
        peripherals.sercom4,
        &mut pm,
        sclk,
        mosi,
        miso,
    )
    .with_dma_channels(chan0, chan1);

    loop {
        let mut source = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut dest = [0xff; 16];

        // Read words into a buffer. The words sent will be be NOP word
        // (by default, 0x00).
        spi.read(&mut dest).unwrap();

        // Send words from a buffer
        spi.write(&source).unwrap();

        // Simultaneously read and write from different buffers.
        //
        // If the source is longer than the destination, the words read
        // in excess will be discarded.
        //
        // If the destination is longer than the source, the excess words
        // sent will be the NOP word (by default, 0x00).
        spi.transfer(&mut dest, &source).unwrap();

        // Simultaneously read and write from the same buffer
        spi.transfer_in_place(&mut source).unwrap();
    }
}
