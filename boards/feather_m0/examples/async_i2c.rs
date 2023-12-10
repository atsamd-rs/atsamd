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
    dmac::{Ch0, DmaController, PriorityLevel},
    prelude::*,
    sercom::{
        i2c::{self, Config, I2cFutureDma},
        Sercom3,
    },
};
use rtic_monotonics::systick::Systick;

atsamd_hal::bind_interrupts!(struct Irqs {
    SERCOM3 => atsamd_hal::sercom::i2c::InterruptHandler<Sercom3>;
    DMAC => atsamd_hal::dmac::InterruptHandler;
});

#[rtic::app(device = bsp::pac, dispatchers = [I2S])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        i2c: I2cFutureDma<Config<bsp::I2cPads>, Ch0>,
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

        // Take SDA and SCL
        let (sda, scl) = (pins.sda, pins.scl);

        // Initialize DMA Controller
        let dmac = DmaController::init(peripherals.DMAC, &mut peripherals.PM);

        // Turn dmac into an async controller
        let mut dmac = dmac.into_future(Irqs);
        // Get individual handles to DMA channels
        let channels = dmac.split();

        // Initialize DMA Channel 0
        let channel0 = channels.0.init(PriorityLevel::LVL0);

        let gclk0 = clocks.gclk0();
        let sercom3_clock = &clocks.sercom3_core(&gclk0).unwrap();
        let pads = i2c::Pads::new(sda, scl);
        let i2c = i2c::Config::new(
            &peripherals.PM,
            peripherals.SERCOM3,
            pads,
            sercom3_clock.freq(),
        )
        .baud(100.kHz())
        .enable()
        .into_future(Irqs)
        .with_dma_channel(channel0);

        async_task::spawn().ok();

        (Shared {}, Local { i2c })
    }

    #[task(local = [i2c])]
    async fn async_task(cx: async_task::Context) {
        let i2c = cx.local.i2c;

        loop {
            defmt::info!("Sending 0x00 to I2C device...");
            // This test is based on the BMP388 barometer. Feel free to use any I2C
            // peripheral you have on hand.
            i2c.write(0x76, &[0x00]).await.unwrap();

            let mut buffer = [0xff; 4];
            i2c.read(0x76, &mut buffer).await.unwrap();
            defmt::info!("Read buffer: {:#x}", buffer);
            Systick::delay(MillisDuration::<u32>::from_ticks(500).convert()).await;
        }
    }
}
