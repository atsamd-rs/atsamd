//! This example shows a safe API to
//! execute a memory-to-memory DMA transfer

#![no_std]
#![no_main]

use cortex_m::asm;
use feather_m0 as hal;
use panic_halt as _;

use hal::{
    clock::GenericClockController,
    entry,
    pac::{CorePeripherals, Peripherals},
};

use hal::dmac::{DmaController, DmaTransfer, PriorityLevel, TriggerAction, TriggerSource};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let _clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pm = peripherals.PM;
    let dmac = peripherals.DMAC;
    let _nvic = core.NVIC;

    // Initialize buffers
    const LENGTH: usize = 50;
    let buf_src: &'static mut [u8; LENGTH] =
        cortex_m::singleton!(: [u8; LENGTH] = [0xff; LENGTH]).unwrap();
    let buf_dest: &'static mut [u8; LENGTH] =
        cortex_m::singleton!(: [u8; LENGTH] = [0x00; LENGTH]).unwrap();

    // Initialize DMA Controller
    let mut dmac = DmaController::init(dmac, &mut pm);
    // Get individual handles to DMA channels
    let channels = dmac.split();

    // Initialize DMA Channel 0
    let chan0 = channels.0.init(&mut dmac, PriorityLevel::LVL0, false);

    // Setup a DMA transfer (memory-to-memory -> incrementing source, incrementing
    // destination) with a 8-bit beat size
    let xfer = DmaTransfer::inc_src_inc_dest(chan0, buf_src, buf_dest, false, ());
    // Begin transfer
    let xfer = xfer.begin(&mut dmac, TriggerSource::DISABLE, TriggerAction::BLOCK);

    // Wait for transfer to complete and grab resulting buffers
    let (buf_src, buf_dest, chan0, _) = xfer.wait(&mut dmac);

    // Read the returned buffers
    let _a = buf_src[LENGTH - 1];
    let _b = buf_dest[LENGTH - 1];

    let const_16: &mut u16 = cortex_m::singleton!(: u16 = 0xADDE).unwrap();
    let buf_16: &'static mut [u16; LENGTH] =
        cortex_m::singleton!(:[u16; LENGTH] = [0x0000; LENGTH]).unwrap();

    // Setup a DMA transfer (memory-to-memory -> fixed source, incrementing
    // destination) with a 16-bit beat size
    let xfer = DmaTransfer::fixed_src_inc_dest(chan0, const_16, buf_16, false, ());
    let xfer = xfer.begin(&mut dmac, TriggerSource::DISABLE, TriggerAction::BLOCK);

    let (const_16, buf_16, chan0, _) = xfer.wait(&mut dmac);

    // Read the returned buffers
    let _a = *const_16;
    let _b = buf_16[LENGTH - 1];

    // Manipulate the returned buffer for fun
    for i in 0..LENGTH {
        buf_16[i] = i as u16;
    }

    // Setup a DMA transfer (memory-to-memory -> incrementing source, fixed
    // destination) with a 16-bit beat size
    let xfer = DmaTransfer::inc_src_fixed_dest(chan0, buf_16, const_16, false, ());
    let xfer = xfer.begin(&mut dmac, TriggerSource::DISABLE, TriggerAction::BLOCK);

    let (buf_16, const_16, _chan0, _) = xfer.wait(&mut dmac);

    // Read the returned buffers
    let _a = *const_16; // We expect the value "LENGTH - 1" to end up here
    let _b = buf_16[LENGTH - 1];

    loop {
        asm::nop();
    }
}
