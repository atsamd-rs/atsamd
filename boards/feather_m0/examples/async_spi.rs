#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

use bsp::hal;
use feather_m0 as bsp;
use fugit::MillisDuration;
use hal::{
    clock::GenericClockController,
    dmac::{Ch0, Ch1, DmaController, PriorityLevel},
    prelude::*,
    sercom::{
        spi::{Config, SpiFutureDuplexDma},
        Sercom4,
    },
};
use rtic_monotonics::systick::Systick;

atsamd_hal::bind_interrupts!(struct Irqs {
    SERCOM4 => atsamd_hal::sercom::spi::InterruptHandler<Sercom4>;
    DMAC => atsamd_hal::dmac::InterruptHandler;
});

#[rtic::app(device = bsp::pac, dispatchers = [I2S])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        spi: SpiFutureDuplexDma<Config<bsp::SpiPads>, Ch0, Ch1>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut peripherals = cx.device;
        let _core = cx.core;

        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );

        let pins = bsp::Pins::new(peripherals.PORT);

        // Take SPI pins
        let (miso, mosi, sclk) = (pins.miso, pins.mosi, pins.sclk);

        // Initialize DMA Controller
        let dmac = DmaController::init(peripherals.DMAC, &mut peripherals.PM);

        // Turn dmac into an async controller
        let mut dmac = dmac.into_future(Irqs);
        // Get individual handles to DMA channels
        let channels = dmac.split();

        // Initialize DMA Channels 0 and 1
        let channel0 = channels.0.init(PriorityLevel::LVL0);
        let channel1 = channels.1.init(PriorityLevel::LVL0);

        let spi = bsp::spi_master(
            &mut clocks,
            100.kHz(),
            peripherals.SERCOM4,
            &mut peripherals.PM,
            sclk,
            mosi,
            miso,
        )
        .into_future(Irqs)
        .with_dma_channels(channel0, channel1);

        async_task::spawn().ok();

        (Shared {}, Local { spi })
    }

    #[task(local = [spi])]
    async fn async_task(cx: async_task::Context) {
        let spi = cx.local.spi;

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
}
