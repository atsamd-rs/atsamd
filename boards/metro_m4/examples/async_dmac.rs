//! This example shows a safe API to
//! execute a memory-to-memory DMA transfer

#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use bsp::hal;
use bsp::pac;
use hal::{
    clock::GenericClockController,
    dmac::{DmaController, PriorityLevel, TriggerAction, TriggerSource},
};
use metro_m4 as bsp;

atsamd_hal::bind_multiple_interrupts!(struct Irqs {
    DMAC: [DMAC_0, DMAC_1, DMAC_2, DMAC_OTHER] => atsamd_hal::dmac::InterruptHandler;
});

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

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
    let mut channel = channels.0.init(PriorityLevel::LVL0);

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
