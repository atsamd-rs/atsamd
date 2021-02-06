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

use hal::dmac::{
    BufferPair, DmaController, PriorityLevel, TransferConfiguration, TriggerAction, TriggerSource,
};

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
    let buffers = BufferPair {
        source: buf_src,
        destination: buf_dest,
    };
    let xfer = buffers.setup_xfer(chan0, false, ());
    // Begin transfer
    let xfer = xfer.begin(&mut dmac, TriggerSource::DISABLE, TriggerAction::BLOCK);

    // Wait for transfer to complete and grab resulting buffers
    let (buffers, chan0, _) = xfer.wait(&mut dmac);

    // Read the returned buffers
    let _a = buffers.source[LENGTH - 1];
    let _b = buffers.destination[LENGTH - 1];

    let const_16: &'static mut u16 = cortex_m::singleton!(: u16 = 0xADDE).unwrap();
    let buf_16: &'static mut [u16; LENGTH] =
        cortex_m::singleton!(:[u16; LENGTH] = [0x0000; LENGTH]).unwrap();

    // Setup a DMA transfer (memory-to-memory -> fixed source, incrementing
    // destination) with a 16-bit beat size
    let buffers = BufferPair {
        source: const_16,
        destination: buf_16.as_mut(),
    };
    let xfer = buffers.setup_xfer(chan0, false, ());
    let xfer = xfer.begin(&mut dmac, TriggerSource::DISABLE, TriggerAction::BLOCK);

    let (buffers, chan0, _) = xfer.wait(&mut dmac);

    // Read the returned buffers
    let _a = *buffers.source;
    let _b = buffers.destination[LENGTH - 1];

    let const_16 = buffers.source;
    let buf_16 = buffers.destination;

    // Manipulate the returned buffer for fun
    for i in 0..LENGTH {
        buf_16[i] = i as u16;
    }

    // Setup a DMA transfer (memory-to-memory -> incrementing source, fixed
    // destination) with a 16-bit beat size
    let buffers = BufferPair {
        source: buf_16.as_mut(),
        destination: const_16,
    };
    let xfer = buffers.setup_xfer(chan0, false, ());
    let xfer = xfer.begin(&mut dmac, TriggerSource::DISABLE, TriggerAction::BLOCK);

    let (buffers, _chan0, _) = xfer.wait(&mut dmac);

    // Read the returned buffers
    let _a = *buffers.destination; // We expect the value "LENGTH - 1" to end up here
    let _b = buffers.source[LENGTH - 1];

    loop {
        asm::nop();
    }
}
