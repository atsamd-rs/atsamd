#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use bsp::hal;
use bsp::pac;
use hal::ehal_async::i2c::I2c;
use hal::fugit::Hertz;
use hal::fugit::MillisDuration;
use hal::{
    clock::GenericClockController,
    dmac::{DmaController, PriorityLevel},
    prelude::*,
    sercom::{i2c, Sercom5},
};
use metro_m4 as bsp;
use rtic_monotonics::Monotonic;

rtic_monotonics::systick_monotonic!(Mono, 10000);

atsamd_hal::bind_multiple_interrupts!(struct DmacIrqs {
    DMAC: [DMAC_0, DMAC_1, DMAC_2, DMAC_OTHER] => atsamd_hal::dmac::InterruptHandler;
});

atsamd_hal::bind_multiple_interrupts!(struct I2cIrqs {
    SERCOM5: [SERCOM5_0, SERCOM5_1, SERCOM5_2, SERCOM5_3, SERCOM5_OTHER] => atsamd_hal::sercom::i2c::InterruptHandler<Sercom5>;
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

    // Take SDA and SCL
    let (sda, scl) = (pins.sda, pins.scl);
    let i2c_sercom = bsp::periph_alias!(peripherals.i2c_sercom);

    // Initialize DMA Controller
    let dmac = DmaController::init(peripherals.dmac, &mut peripherals.pm);

    // Turn dmac into an async controller
    let mut dmac = dmac.into_future(DmacIrqs);
    // Get individual handles to DMA channels
    let channels = dmac.split();

    // Initialize DMA Channel 0
    let channel0 = channels.0.init(PriorityLevel::Lvl0);

    let gclk0 = clocks.gclk0();
    let sercom5_clock = &clocks.sercom5_core(&gclk0).unwrap();
    let pads = i2c::Pads::new(sda, scl);
    let mut i2c = i2c::Config::new(&peripherals.mclk, i2c_sercom, pads, sercom5_clock.freq())
        .baud(100.kHz())
        .enable()
        .into_future(I2cIrqs)
        .with_dma_channel(channel0);

    loop {
        defmt::info!("Sending 0x00 to I2C device...");
        // This test is based on the BMP388 barometer. Feel free to use any I2C
        // peripheral you have on hand.
        i2c.write(0x76, &[0x00]).await.unwrap();

        let mut buffer = [0xff; 4];
        i2c.read(0x76, &mut buffer).await.unwrap();
        defmt::info!("Read buffer: {:#x}", buffer);
        Mono::delay(MillisDuration::<u32>::from_ticks(500).convert()).await;
    }
}
