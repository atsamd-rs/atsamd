//! This example shows a safe API to
//! execute a memory-to-memory DMA transfer

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

atsamd_hal::bind_interrupts!(struct Irqs {
    DMAC: [DMAC_0, DMAC_1, DMAC_2, DMAC_OTHER] => atsamd_hal::dmac::InterruptHandler;
});

#[rtic::app(device = bsp::pac, dispatchers = [I2S])]
mod app {
    use bsp::hal;
    use metro_m4 as bsp;
    use hal::{
        clock::GenericClockController,
        dmac::{
            Ch0, Channel, DmaController, PriorityLevel, ReadyFuture, TriggerAction, TriggerSource,
        },
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        channel: Channel<Ch0, ReadyFuture>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut peripherals = cx.device;
        let _core = cx.core;

        let _clocks = GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.MCLK,
            &mut peripherals.OSC32KCTRL,
            &mut peripherals.OSCCTRL,
            &mut peripherals.NVMCTRL,
        );

        // Initialize DMA Controller
        let dmac = DmaController::init(peripherals.DMAC, &mut peripherals.PM);

        // Turn dmac into an async controller
        let mut dmac = dmac.into_future(crate::Irqs);
        // Get individual handles to DMA channels
        let channels = dmac.split();

        // Initialize DMA Channel 0
        let channel = channels.0.init(PriorityLevel::LVL0);

        async_task::spawn().ok();
        (Shared {}, Local { channel })
    }

    #[task(local = [channel])]
    async fn async_task(cx: async_task::Context) {
        let channel = cx.local.channel;

        let mut source = [0xff; 1000];
        let mut dest = [0x0; 1000];

        defmt::info!(
            "Launching a DMA transfer.\n\tSource: {}\n\tDestination: {}",
            &source,
            &dest
        );

        channel
            .transfer_future(
                &mut source,
                &mut dest,
                TriggerSource::DISABLE,
                TriggerAction::BLOCK,
            )
            .await
            .unwrap();

        defmt::info!(
            "Finished DMA transfer.\n\tSource: {}\n\tDestination: {}",
            &source,
            &dest
        );

        loop {
            cortex_m::asm::wfi();
        }
    }
}
