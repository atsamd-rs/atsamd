#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use bsp::hal;
use bsp::pac;
use hal::ehal_async::spi::SpiBus;
use hal::fugit::Hertz;
use hal::fugit::MillisDuration;
use hal::{
    clock::GenericClockController,
    dmac::{DmaController, PriorityLevel},
    prelude::*,
    sercom::Sercom2,
};
use metro_m4 as bsp;
use rtic_monotonics::Monotonic;

rtic_monotonics::systick_monotonic!(Mono, 10000);

atsamd_hal::bind_multiple_interrupts!(struct DmacIrqs {
    DMAC: [DMAC_0, DMAC_1, DMAC_2, DMAC_OTHER] => atsamd_hal::dmac::InterruptHandler;
});

atsamd_hal::bind_multiple_interrupts!(struct SpiIrqs {
    SERCOM2: [SERCOM2_0, SERCOM2_1, SERCOM2_2, SERCOM2_3, SERCOM2_OTHER] => atsamd_hal::sercom::spi::InterruptHandler<Sercom2>;
});

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );

    let freq: Hertz<u32> = clocks.gclk0().into();
    Mono::start(_core.SYST, freq.to_Hz());

    let pins = bsp::Pins::new(peripherals.port);

    // Take SPI pins
    let (miso, mosi, sclk) = (pins.miso, pins.mosi, pins.sclk);
    let spi_sercom = bsp::periph_alias!(peripherals.spi_sercom);

    // Initialize DMA Controller
    let dmac = DmaController::init(peripherals.dmac, &mut peripherals.pm);

    // Turn dmac into an async controller
    let mut dmac = dmac.into_future(DmacIrqs);
    // Get individual handles to DMA channels
    let channels = dmac.split();

    // Initialize DMA Channels 0 and 1
    let channel0 = channels.0.init(PriorityLevel::Lvl0);
    let channel1 = channels.1.init(PriorityLevel::Lvl0);

    let mut spi = bsp::spi_master(
        &mut clocks,
        100.kHz(),
        spi_sercom,
        &mut peripherals.mclk,
        sclk,
        mosi,
        miso,
    )
    .into_future(SpiIrqs)
    .with_dma_channels(channel0, channel1);

    loop {
        defmt::info!("Sending 0x00 to SPI device...");
        spi.write(&[0x00]).await.unwrap();

        defmt::info!("Sent 0x00.");

        let mut buffer = [0xff; 4];
        spi.read(&mut buffer).await.unwrap();
        defmt::info!("Read buffer: {:#x}", buffer);
        Mono::delay(MillisDuration::<u32>::from_ticks(1000).convert()).await;
    }
}
