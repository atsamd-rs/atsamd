//! This example showcases the i2c module.

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
use hal::ehal::blocking::i2c::WriteRead;
use hal::prelude::*;
use hal::sercom::i2c;

const LENGTH: usize = 1;
const ADDRESS: u8 = 0x77;

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

    let buf_src: &'static mut [u8; LENGTH] =
        cortex_m::singleton!(: [u8; LENGTH] = [0x00; LENGTH]).unwrap();
    let buf_dest: &'static mut [u8; LENGTH] =
        cortex_m::singleton!(: [u8; LENGTH] = [0x00; LENGTH]).unwrap();

    let gclk0 = clocks.gclk0();
    let sercom3_clock = &clocks.sercom3_core(&gclk0).unwrap();
    let pads = i2c::Pads::new(sda, scl);
    let mut i2c = i2c::Config::new(&pm, peripherals.SERCOM3, pads, sercom3_clock.freq())
        .baud(100.khz())
        .enable();

    let mut buffer = [0x00; 1];

    // Test writing then reading from an I2C chip
    i2c.write_read(ADDRESS, &[0x00], &mut buffer).unwrap();

    // Test writing then reading using DMA
    let init_token = i2c.init_dma_transfer().unwrap();
    let xfer = i2c.send_with_dma(ADDRESS, init_token, buf_src, chan0, |_| {});
    let (chan0, _buf_src, mut i2c) = xfer.wait();

    let init_token = i2c.init_dma_transfer().unwrap();
    let xfer = i2c.receive_with_dma(ADDRESS, init_token, buf_dest, chan0, |_| {});
    let (_chan0, _i2c, _buf_dest) = xfer.wait();

    loop {
        // Go to sleep
        asm::wfi();
    }
}
