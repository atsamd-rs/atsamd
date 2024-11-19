//! This example showcases the i2c module, and uses DMA to perform I2C
//! transactions.

#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use grand_central_m4 as bsp;

use bsp::hal;
use bsp::pac;
use bsp::{entry, periph_alias, pin_alias};

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
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );

    let mclk = peripherals.mclk;
    let dmac = peripherals.dmac;
    let pins = bsp::Pins::new(peripherals.port);

    // Take SDA and SCL
    let (sda, scl) = (pin_alias!(pins.sda), pin_alias!(pins.scl));

    // Setup DMA channels for later use
    let mut dmac = DmaController::init(dmac, &mut peripherals.pm);
    let channels = dmac.split();
    let chan0 = channels.0.init(PriorityLevel::Lvl0);

    let gclk0 = clocks.gclk0();
    let sercom5_clock = &clocks.sercom5_core(&gclk0).unwrap();
    let pads = i2c::Pads::new(sda, scl);
    let i2c_sercom = periph_alias!(peripherals.i2c_sercom);
    let mut i2c = i2c::Config::new(&mclk, i2c_sercom, pads, sercom5_clock.freq())
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
