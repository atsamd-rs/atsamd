#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(device = bsp::pac, dispatchers = [I2S])]
mod app {
    use bsp::{hal, pac};
    use feather_m0 as bsp;
    use fugit::MillisDuration;
    use hal::{
        clock::{enable_internal_32kosc, ClockGenId, ClockSource, GenericClockController},
        dmac::{self, Ch0, DmaController, PriorityLevel},
        prelude::*,
        rtc::{Count32Mode, Rtc},
        sercom::i2c::{self, Config, I2cFutureDma},
    };

    #[monotonic(binds = RTC, default = true)]
    type Monotonic = Rtc<Count32Mode>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        i2c: I2cFutureDma<Config<bsp::I2cPads>, Ch0>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
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

        let sercom3_irq = cortex_m_interrupt::take_nvic_interrupt!(pac::Interrupt::SERCOM3, 2);
        // tc4_irq.set_priority(2);

        enable_internal_32kosc(&mut peripherals.SYSCTRL);
        let timer_clock = clocks
            .configure_gclk_divider_and_source(ClockGenId::GCLK2, 1, ClockSource::OSC32K, false)
            .unwrap();
        clocks.configure_standby(ClockGenId::GCLK2, true);

        // Setup RTC monotonic
        let rtc_clock = clocks.rtc(&timer_clock).unwrap();
        let rtc = Rtc::count32_mode(peripherals.RTC, rtc_clock.freq(), &mut peripherals.PM);

        // Initialize DMA Controller
        let dmac = DmaController::init(peripherals.DMAC, &mut peripherals.PM);
        // Get handle to IRQ
        let dmac_irq = dmac::Interrupts::new(cortex_m_interrupt::take_nvic_interrupt!(
            pac::Interrupt::DMAC,
            2
        ));
        // Turn dmac into an async controller
        let mut dmac = dmac.into_future(dmac_irq);
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
        .baud(100.khz())
        .enable()
        .into_future(sercom3_irq)
        .with_dma_channel(channel0);

        async_task::spawn().ok();

        (Shared {}, Local { i2c }, init::Monotonics(rtc))
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
            crate::app::monotonics::delay(MillisDuration::<u32>::from_ticks(500).convert()).await;
        }
    }
}
