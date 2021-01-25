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
//! transfer configurations, including multi-buffer
//! (linked-list descriptor) transfers, are not currently supported.
//!
//! Transfers are supported for `i8`, `u8`, `i16`, `u16`, `i32`, `u32` and `f32`
//! beat sizes.
//!
//! # Enabling DMA support
//!
//! You must enable the `dma` feature in your board support crate
//! or final executable.
//!
//! Add this to your `Cargo.toml`:
//! ```
//! [features]
//! dma = ["atsamd-hal/dma"]
//! ```
//!
//! # Channels and RAM
//!
//! Using DMA channels require a certain amount of RAM - 32 bytes per channel,
//! to be exact. RAM will be not allocated unless the `dma` feature is enabled
//! for the HAL. By default, half the channels available on the chip are
//! enabled. If you need all DMA channels enabled, enable the `max-channels`
//! feature in your board support crate or final executable.
//!
//! `Cargo.toml`
//! ```
//! [features]
//! dma = ["atsamd-hal/dma"]
//! max-channels = ["dma", "atsamd-hal/max-channels"]
//! ```
//!
//! RAM usage per chip family:
//!
//! * `ATSAMD11` - 3 channels (default): 96 bytes
//!
//! * `ATSAMD11` - 6 channels (max): 192 bytes
//!
//! * `ATSAMD21` - 6 channels (default): 192 bytes
//!
//! * `ATSAMD21`: - 12 channels (max): 384 bytes
//!
//! * `ATSAMD51/ATSAME5x`: - 16 channels (default): 512 bytes
//!
//! * `ATSAMD51/ATSAME5x`: - 32 channels (max): 1024 bytes
//!
//! # Priority levels and Arbitration
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
//! Round-Robin Arbitration can be enabled for individual priority levels by
//! using the various `level_x_round_robin` methods. By default, all priority
//! levels are initialized with a fixed arbitration scheme. See ATSAMD21
//! datasheet section 19.6.2.4 for more information.
//!
//! # Interrupts
//!
//! This driver does not use or manage interrupts issued by the DMAC. Individual
//! channels can be configured to generate interrupts when the transfer is
//! complete, an error is detected or the channel is suspended. However, these
//! interrupts will not be triggered unless the DMAC interrupt is unmasked in
//! the NVIC. You will be responsible for clearing the interrupt flags in the
//! ISR.
//!
//! # About static lifetimes
//!
//! The safe API this driver offers requires all buffers (source and
//! destination) to have `'static` lifetimes. This is because
//! [`mem::forget`](core::mem::forget) is a safe API, and therefore relying on
//! [`mem::drop`](core::mem::drop) to terminate or abort a transfer
//! does not guarantee the transfer will be terminated (specifically if
//! [`mem::forget`](core::mem::forget) is called on a `DmaTransfer` containaing
//! a `Channel<Busy, ID>`). This could cause the compiler to reclaim
//! stack-allocated buffers for reuse while the DMAC is still writing to/reading
//! from them! Needless to say that is very unsafe. Refer [here](https://docs.rust-embedded.org/embedonomicon/dma.html#memforget)
//! or [here](https://blog.japaric.io/safe-dma/) for more information. You may choose to forego
//! the `'static` lifetimes by using the unsafe API and the various
//! `x_src_x_dest_unchecked` methods.
//!
//! # Unsafe API
//!
//! This driver also offers an `unsafe` API through the `x_src_x_dest_unchecked`
//! methods. These do not enforce `'static` lifetimes, and allow using `*mut T`
//! pointers as buffers. If you choose to use these methods, you MUST prove that
//! a `DmaTransfer` containing a `Channel<Busy, ID>` will NEVER be dropped. You
//! *must* call `wait()` or `stop()` manually on every
//! [`UnsafeTransfer`](transfer::UnsafeTransfer) to convert their underlying
//! channels back into `Channel<Ready, ID>`. No destructor or `Drop`
//! implementation is offered for `DmaTransfers` or `UnsafeTransfer`s.
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

pub mod channel;
pub mod dma_controller;
pub mod transfer;

#[cfg(any(
    feature = "samd51",
    feature = "same51",
    feature = "same53",
    feature = "same54"
))]
pub use dma_controller::{BurstLength, FifoThreshold};
pub use dma_controller::{DmaController, PriorityLevel, TriggerAction, TriggerSource};
pub use static_assertions::const_assert;
pub use transfer::{DmaTransfer, UnsafeTransfer};

/// Maximum number of DMA channels supported by SAMD11 chips
#[cfg(feature = "samd11")]
const MAX_CHANNELS: usize = 6;

/// Maximum number of DMA channels supported by SAMD21 chips
#[cfg(feature = "samd21")]
const MAX_CHANNELS: usize = 12;

/// Maximum number of DMA channels supported by SAMD51, SAME51, SAME53 and
/// SAME54 chips
#[cfg(any(
    feature = "samd51",
    feature = "same51",
    feature = "same53",
    feature = "same54"
))]
const MAX_CHANNELS: usize = 32;

/// Number of DMA channels used by the driver
#[cfg(feature = "max-channels")]
pub const NUM_CHANNELS: usize = MAX_CHANNELS;

/// Number of channels used by the driver
#[cfg(not(feature = "max-channels"))]
pub const NUM_CHANNELS: usize = MAX_CHANNELS / 2;

// ----- DMAC SRAM registers ----- //
/// Descriptor representing a SRAM register. Datasheet section 19.8.2
#[derive(Clone, Copy)]
#[repr(C, align(16))]
#[doc(hidden)]
pub struct DmacDescriptor {
    btctrl: u16,
    btcnt: u16,
    srcaddr: u32,
    dstaddr: u32,
    descaddr: u32,
}

#[doc(hidden)]
pub const DEFAULT_DESCRIPTOR: DmacDescriptor = DmacDescriptor {
    btctrl: 0,
    btcnt: 0,
    srcaddr: 0,
    dstaddr: 0,
    descaddr: 0,
};

// Writeback section. This static variable should never be written to in an
// interrupt or thread context.
#[doc(hidden)]
static mut WRITEBACK: [DmacDescriptor; NUM_CHANNELS] = [DEFAULT_DESCRIPTOR; NUM_CHANNELS];
// Descriptor section. This static variable should never be written to in an
// interrupt or thread context.
#[doc(hidden)]
static mut DESCRIPTOR_SECTION: [DmacDescriptor; NUM_CHANNELS] = [DEFAULT_DESCRIPTOR; NUM_CHANNELS];
