//! # Direct Memory Access Controller
//!
//! This library provides a type-safe API with compile-time guarantees
//! that the peripheral and individual DMA channels are correctly configured
//! before launching a DMA transfer.
//!
//! This module currently supports most basic DMA
//! functions, including memory-to-memory,
//! memory-to-peripheral, peripheral-to-memory,
//! and peripheral-to-peripheral transfers.
//! One-shot and circular transfers are supported. More complex
//! transfer configurations, including multi-descriptor
//! (linked-list) transfers, are not currently supported.
//! Advances arbitration schemes (eg. round-robin) are not currently
//! supported.
//!
//! # Priority levels
//!
//! The DMAC features 4 priority levels. Level 0 has the highest priority
//! and level 3 has the lowest. Each channel can be assigned to one priority
//! level. If two channels with the same priority level are requested to
//! execute a transfer at the same time, the lowest channel number will have
//! priority (in the default arbitration scheme, eg. not round-robin).
//!
//! By default, all priority levels are enabled when initializing the DMAC
//! (see [`DmaController::init`](dma_controller::DmaController::init)). Levels
//! can be enabled or disabled through the various `level_x_enabled` methods.
//!
//! # Interrupts
//!
//! This driver does not use or manage interrupts issued by the DMAC. Individual channels
//! can be configured to generate interrupts when the transfer is complete, an error is
//! detected or the channel is suspended. However, these interrupts will not be triggered
//! unless the DMAC interrupt is unmasked in the NVIC. You will be responsible for clearing
//! the interrupt flags in the ISR.
//!
//! # Example
//! ```
//! let mut peripherals = Peripherals::take().unwrap();
//! let mut dmac = DmaController::init(peripherals.DMAC, &mut peripherals.PM);
//! // Get individual handles to DMA channels
//! let channels = dmac.split();
//!
//! // Initialize DMA Channel 0
//! let chan0 = channels.0.init(PriorityLevel::LVL0, false, &mut dmac);
//!
//! // Setup a DMA transfer (memory-to-memory -> incrementing source, incrementing destination)
//! // NOTE: buf_src and buf_dest should be either:
//! // &'static mut T, &'static mut [T], or &'static mut [T; N] where T: BeatSize
//! let xfer = DmaTransfer::inc_src_inc_dest(chan0, buf_src, buf_dest, false, ());
//! // Begin transfer with a software trigger
//! let xfer = xfer.begin(&mut dmac, TriggerSource::DISABLE, TriggerAction::BLOCK);
//!
//! // Wait for transfer to complete and grab resulting buffers
//! let (buf_src, buf_dest, chan0, _) = xfer.wait(&mut dmac);
//! ```

#![warn(missing_docs)]

pub mod channel;
pub mod dma_controller;
pub mod transfer;

pub use dma_controller::{DmaController, PriorityLevel, TriggerAction, TriggerSource};
pub use transfer::DmaTransfer;

// TODO dynamically change at compile time?
// Maximum number of channels is 12 for ATSAMD21
/// Number of DMA channels available for the chip
pub const NUM_CHANNELS: usize = 12;

// ----- DMAC SRAM registers ----- //
/// Descriptor representing a SRAM register. Datasheet section 19.8.2
#[derive(Clone, Copy)]
#[repr(C, align(16))]
#[doc(hidden)]
struct DmacDescriptor {
    btctrl: u16,
    btcnt: u16,
    srcaddr: u32,
    dstaddr: u32,
    descaddr: u32,
}

#[doc(hidden)]
const DEFAULT_DESCRIPTOR: DmacDescriptor = DmacDescriptor {
    btctrl: 0,
    btcnt: 0,
    srcaddr: 0,
    dstaddr: 0,
    descaddr: 0,
};

// Writeback section. This static variable should never be written to in an interrupt
// or thread context.
#[doc(hidden)]
static mut WRITEBACK: [DmacDescriptor; NUM_CHANNELS] = [DEFAULT_DESCRIPTOR; NUM_CHANNELS];
// Descriptor section. This static variable should never be written to in an interrupt
// or thread context.
#[doc(hidden)]
static mut DESCRIPTOR_SECTION: [DmacDescriptor; NUM_CHANNELS] = [DEFAULT_DESCRIPTOR; NUM_CHANNELS];
