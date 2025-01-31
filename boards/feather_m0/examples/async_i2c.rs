#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use bsp::hal;
use bsp::pac;
use feather_m0 as bsp;
use hal::ehal_async::i2c::I2c;
use hal::{
    clock::GenericClockController,
    dmac::{DmaController, PriorityLevel},
    fugit::MillisDuration,
    prelude::*,
    sercom::{i2c, Sercom3},
};
use rtic_monotonics::systick::Systick;

atsamd_hal::bind_interrupts!(struct Irqs {
    SERCOM3 => atsamd_hal::sercom::i2c::InterruptHandler<Sercom3>;
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

    // Take SDA and SCL
    let (sda, scl) = (pins.sda, pins.scl);
    let i2c_sercom = bsp::periph_alias!(peripherals.i2c_sercom);

    // Initialize DMA Controller
    let dmac = DmaController::init(peripherals.dmac, &mut peripherals.pm);

    // Turn dmac into an async controller
    let mut dmac = dmac.into_future(Irqs);
    // Get individual handles to DMA channels
    let channels = dmac.split();

    // Initialize DMA Channel 0
    let channel0 = channels.0.init(PriorityLevel::Lvl0);

    let gclk0 = clocks.gclk0();
    let sercom3_clock = &clocks.sercom3_core(&gclk0).unwrap();
    let pads = i2c::Pads::new(sda, scl);
    let mut i2c = i2c::Config::new(&peripherals.pm, i2c_sercom, pads, sercom3_clock.freq())
        .baud(100.kHz())
        .enable()
        .into_future(Irqs)
        .with_dma_channel(channel0);

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
