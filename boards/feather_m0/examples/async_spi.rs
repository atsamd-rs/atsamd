#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use bsp::hal;
use bsp::pac;
use feather_m0 as bsp;
use hal::ehal_async::spi::SpiBus;
use hal::{
    clock::GenericClockController,
    dmac::{DmaController, PriorityLevel},
    fugit::MillisDuration,
    prelude::*,
    sercom::Sercom4,
};
use rtic_monotonics::systick::Systick;

atsamd_hal::bind_interrupts!(struct Irqs {
    SERCOM4 => atsamd_hal::sercom::spi::InterruptHandler<Sercom4>;
    DMAC => atsamd_hal::dmac::InterruptHandler;
});

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    let pins = bsp::Pins::new(peripherals.port);

    // Take SPI pins
    let (miso, mosi, sclk) = (pins.miso, pins.mosi, pins.sclk);
    let spi_sercom = bsp::periph_alias!(peripherals.spi_sercom);

    // Initialize DMA Controller
    let dmac = DmaController::init(peripherals.dmac, &mut peripherals.pm);

    // Turn dmac into an async controller
    let mut dmac = dmac.into_future(Irqs);
    // Get individual handles to DMA channels
    let channels = dmac.split();

    // Initialize DMA Channels 0 and 1
    let channel0 = channels.0.init(PriorityLevel::Lvl0);
    let channel1 = channels.1.init(PriorityLevel::Lvl0);

    let mut spi = bsp::spi_master(
        &mut clocks,
        100.kHz(),
        spi_sercom,
        &mut peripherals.pm,
        sclk,
        mosi,
        miso,
    )
    .into_future(Irqs)
    .with_dma_channels(channel0, channel1);

    loop {
        defmt::info!("Sending 0x00 to SPI device...");
        // This test is based on the BMP388 barometer. Feel free to use any I2C
        // peripheral you have on hand.
        spi.write(&[0x00]).await.unwrap();

        defmt::info!("Sent 0x00.");

        let mut buffer = [0xff; 4];
        spi.read(&mut buffer).await.unwrap();
        defmt::info!("Read buffer: {:#x}", buffer);
        Systick::delay(MillisDuration::<u32>::from_ticks(500).convert()).await;
    }
}
