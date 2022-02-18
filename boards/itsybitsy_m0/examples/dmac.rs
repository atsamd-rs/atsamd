//! This example shows a safe API to
//! execute a memory-to-memory DMA transfer

#![no_std]
#![no_main]

use cortex_m::asm;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use bsp::pac;
use itsybitsy_m0 as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::dmac::{DmaController, PriorityLevel, Transfer, TriggerAction, TriggerSource};
use pac::Peripherals;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let _clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pm = peripherals.PM;
    let dmac = peripherals.DMAC;

    // Initialize buffers
    const LENGTH: usize = 50;
    let buf_src: &'static mut [u8; LENGTH] =
        cortex_m::singleton!(: [u8; LENGTH] = [0xff; LENGTH]).unwrap();
    let buf_dest: &'static mut [u8; LENGTH] =
        cortex_m::singleton!(: [u8; LENGTH] = [0x00; LENGTH]).unwrap();

    // Initialize DMA Controller
    let mut dmac = DmaController::init(dmac, &mut pm);
    // Get individual handles to DMA channels
    let mut channels = dmac.split();

    // Initialize DMA Channel 0
    let chan0 = channels.0.init(PriorityLevel::LVL0);

    // Setup a DMA transfer (memory-to-memory -> incrementing source, incrementing
    // destination) with a 8-bit beat size
    let xfer = Transfer::new_from_arrays(chan0, buf_src, buf_dest, false)
        .with_waker(|_status| asm::nop())
        .begin(TriggerSource::DISABLE, TriggerAction::BLOCK);
    // Wait for transfer to complete and grab resulting buffers
    let (chan0, buf_src, buf_dest) = xfer.wait();

    // Read the returned buffers
    let _a = buf_src[LENGTH - 1];
    let _b = buf_dest[LENGTH - 1];

    let const_16: &'static mut u16 = cortex_m::singleton!(: u16 = 0xADDE).unwrap();
    let buf_16: &'static mut [u16; LENGTH] =
        cortex_m::singleton!(:[u16; LENGTH] = [0x0000; LENGTH]).unwrap();

    // Setup a DMA transfer (memory-to-memory -> fixed source, incrementing
    // destination) with a 16-bit beat size.
    let xfer = Transfer::new(chan0, const_16, buf_16, false)
        .unwrap()
        .begin(TriggerSource::DISABLE, TriggerAction::BLOCK);

    let (chan0, const_16, buf_16) = xfer.wait();

    // Read the returned buffers
    let _a = *const_16;
    let _b = buf_16[LENGTH - 1];

    // Manipulate the returned buffer for fun
    for (i, c) in buf_16.iter_mut().enumerate() {
        *c = i as u16;
    }

    // Setup a DMA transfer (memory-to-memory -> incrementing source, fixed
    // destination) with a 16-bit beat size
    let xfer = Transfer::new(chan0, buf_16, const_16, false)
        .unwrap()
        .begin(TriggerSource::DISABLE, TriggerAction::BLOCK);

    let (chan0, buf_16, const_16) = xfer.wait();

    // Read the returned buffers
    let _a = *const_16; // We expect the value "LENGTH - 1" to end up here
    let _b = buf_16[LENGTH - 1];

    // Move split channels back into the Channels struct
    channels.0 = chan0.into();
    // Free the DmaController and return the PAC DMAC struct
    let _dmac = dmac.free(channels, &mut pm);

    loop {
        asm::nop();
    }
}
