#![no_std]
#![no_main]

use atsamd_hal::{dmac::{self, DmaController}, ehal::digital::OutputPin, fugit::{ExtU32, RateExtU32}, sercom::spi, timer::TimerCounter};
use defmt;
use defmt_rtt as _;
use panic_probe as _;

use atsame54_xpro as bsp;
use bsp::hal;
use hal::{
    pac,
    clock::{ClockGenId, ClockSource, GenericClockController},
    ehal::digital::StatefulOutputPin,
    eic::{Eic, Sense},
    gpio::{Pin, PullUpInterrupt},
};

atsamd_hal::bind_interrupts!(struct Irqs {
    TC3 => atsamd_hal::timer::InterruptHandler<pac::Tc3>;
});

atsamd_hal::bind_multiple_interrupts!(pub struct DmacIrqs {
    DMAC: [DMAC_0, DMAC_1, DMAC_2, DMAC_3, DMAC_OTHER] => atsamd_hal::dmac::InterruptHandler;
});

atsamd_hal::bind_multiple_interrupts!(pub struct SercomIrqs {
    SERCOM4: [SERCOM4_0, SERCOM4_1, SERCOM4_2, SERCOM4_OTHER] => atsamd_hal::sercom::spi::InterruptHandler<pac::Sercom4>;
});


#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    let mut p = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        p.gclk,
        &mut p.mclk,
        &mut p.osc32kctrl,
        &mut p.oscctrl,
        &mut p.nvmctrl,
    );

    let pins = bsp::Pins::new(p.port);
    let mut led = bsp::pin_alias!(pins.led).into_push_pull_output();

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, p.tc3, &mut p.mclk).into_future(Irqs);

    let sercom4_clk = clocks.sercom4_core(&gclk0).unwrap();
    
    let dmac = DmaController::init(p.dmac, &mut p.pm);
    let mut dmac = dmac.into_future(DmacIrqs);
    let channels: dmac::FutureChannels = dmac.split();

    let channel0 = channels.0.init(dmac::PriorityLevel::Lvl3);
    let channel1 = channels.1.init(dmac::PriorityLevel::Lvl3);

    // Set up a SPI peripheral that uses DMA
    let mut spi = spi::Config::new(
        &mut p.mclk,
        p.sercom4,
        spi::Pads::default().data_in(pins.pb29).sclk(pins.pb26)
        // .data_out(pins.pb20) /* Commenting out mosi  */
        , 
        sercom4_clk.freq(),
    )
    .length::<spi::lengths::U1>()
    .spi_mode(atsamd_hal::ehal::spi::MODE_2)
    .baud(1_u32.MHz())
    .enable()
    .into_future(SercomIrqs)
    .with_dma_channels(channel0, channel1)
    ;

    loop {
        let mut buf = [0_u8; 10];
        let _ = spi.read_dma_master(&mut buf).await;
        
        timer.delay(100_u32.millis()).await;
        led.set_high().unwrap();
        timer.delay(100_u32.millis()).await;
        led.set_low().unwrap();
    }
}
