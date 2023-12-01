//! This example shows a safe API to
//! execute a memory-to-memory DMA transfer

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

atsamd_hal::bind_interrupts!(struct Irqs {
    DMAC => atsamd_hal::dmac::async_api::InterruptHandler;
});

use bsp::hal;
use feather_m0 as bsp;
use hal::{
    clock::GenericClockController,
    dmac::{DmaController, PriorityLevel, TriggerAction, TriggerSource},
    async_hal::interrupts::{Interrupt, Priority, DMAC},
};

#[embassy_executor::main]
async fn main(_p: embassy_executor::Spawner) {
    let mut peripherals = hal::pac::Peripherals::take().unwrap();

    let _clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    // Initialize DMA Controller
    let dmac = DmaController::init(peripherals.DMAC, &mut peripherals.PM);
    DMAC::set_priority(Priority::P0);

    // Turn dmac into an async controller
    let mut dmac = dmac.into_future(crate::Irqs);
    // Get individual handles to DMA channels
    let channels = dmac.split();

    // Initialize DMA Channel 0
    let mut channel = channels.0.init(PriorityLevel::LVL0);

    let mut source = [0xff; 50];
    let mut dest = [0x0; 50];

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
        core::future::pending::<()>().await;
    }
}
