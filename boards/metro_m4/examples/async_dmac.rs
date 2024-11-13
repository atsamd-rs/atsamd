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
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );

    // Initialize DMA Controller
    let dmac = DmaController::init(peripherals.dmac, &mut peripherals.pm);

    // Turn dmac into an async controller
    let mut dmac = dmac.into_future(crate::Irqs);
    // Get individual handles to DMA channels
    let channels = dmac.split();

    // Initialize DMA Channel 0
    let mut channel = channels.0.init(PriorityLevel::Lvl0);

    let mut source = [0xff; 100];
    let mut dest = [0x0; 100];

    defmt::info!(
        "Launching a DMA transfer.\n\tSource: {:#x}\n\tDestination: {:#x}",
        &source,
        &dest
    );

    channel
        .transfer_future(
            &mut source,
            &mut dest,
            TriggerSource::Disable,
            TriggerAction::Block,
        )
        .await
        .unwrap();

    defmt::info!(
        "Finished DMA transfer.\n\tSource: {:#x}\n\tDestination: {:#x}",
        &source,
        &dest
    );

    loop {
        cortex_m::asm::wfi();
    }
}
