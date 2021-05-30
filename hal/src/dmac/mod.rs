#![allow(unused_braces)]

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
//! The DMAC features 4 priority levels. Level 3 has the highest priority
//! and level 0 has the lowest. Each channel can be assigned to one priority
//! level. If two channels with the same priority level are requested to
//! execute a transfer at the same time, the lowest channel number will have
//! priority (in the default, ie static, arbitration scheme).
//!
//! By default, all priority levels are enabled when initializing the DMAC
//! (see [`DmaController::init`]). Levels
//! can be enabled or disabled through the
//! [`DmaController::enable_levels`] and
//! [`DmaController::disable_levels`] methods. These methods must be supplied a
//! [`PriorityLevelMask`].
//!
//! Round-Robin Arbitration can be enabled for multiple priority levels
//! simultaneously by using the
//! [`DmaController::round_robin_arbitration`] and
//! [`DmaController::static_arbitration`] methods. These methods must be
//! supplied a [`RoundRobinMask`]. By default, all priority levels are
//! initialized with a static arbitration scheme. See ATSAMD21 datasheet section
//! 19.6.2.4 for more information.
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
//! # Payloads
//!
//! You may choose to append a payload to a `Transfer` by using the
//! [`Transfer::with_payload`] method. This will take ownership of an aribtrary
//! type until the transfer is released (`stop`ped or `wait`ed). This is useful,
//! for instance, to take ownership of a peripheral until the transfer is
//! complete and prevent data races.
//!
//! # About static lifetimes
//!
//! The safe API this driver offers requires all buffers (source and
//! destination) to have `'static` lifetimes. This is because
//! [`mem::forget`](core::mem::forget) is a safe API, and therefore relying on
//! [`mem::drop`](core::mem::drop) to terminate or abort a transfer
//! does not guarantee the transfer will be terminated (specifically if
//! [`mem::forget`](core::mem::forget) is called on a `Transfer` containaing
//! a `Channel<Id, Busy>`). This could cause the compiler to reclaim
//! stack-allocated buffers for reuse while the DMAC is still writing to/reading
//! from them! Needless to say that is very unsafe.
//! Refer [here](https://docs.rust-embedded.org/embedonomicon/dma.html#memforget)
//! or [here](https://blog.japaric.io/safe-dma/) for more information. You may choose to forego
//! the `'static` lifetimes by using the unsafe API and the
//! [`Transfer::new_unchecked`](transfer::Transfer::new_unchecked) method.
//!
//! # Unsafe API
//!
//! This driver also offers an `unsafe` API through the
//! [`Transfer::new_unchecked`] method. It
//! does not enforce `'static` lifetimes, and allow using buffers of different
//! lengths. If you choose to use these methods, you MUST prove that
//! a `Transfer` containing a `Channel<Id, Busy>` will NEVER be dropped. You
//! *must* call `wait()` or `stop()` manually on every
//! `Transfer` that has been created using the unsafe API. No destructor or
//! `Drop` implementation is offered for `Transfer`s.
//!
//! Additionally, you can (unsafely) implement your own buffer types through the
//! unsafe [`Buffer`](transfer::Buffer) trait.
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
//! let xfer = Transfer::new(chan0, buf_src, buf_dest, false).begin(
//!     &mut dmac,
//!     TriggerSource::DISABLE,
//!     TriggerAction::BLOCK,
//! );
//!
//! // Wait for transfer to complete and grab resulting buffers
//! let (chan0, buf_src, buf_dest, _) = xfer.wait(&mut dmac);
//! ```

use modular_bitfield::prelude::*;

#[cfg(feature = "min-samd51g")]
pub use dma_controller::{BurstLength, FifoThreshold};
pub use dma_controller::{
    DmaController, PriorityLevel, PriorityLevelMask, RoundRobinMask, TriggerAction, TriggerSource,
};
use transfer::BeatSize;
pub use transfer::{Beat, Buffer, Transfer};

#[cfg(all(feature = "samd11", feature = "max-channels"))]
#[macro_export]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro!(6);
    };
}

#[cfg(all(feature = "samd21", feature = "max-channels"))]
#[macro_export]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro!(12);
    };
}

#[cfg(all(feature = "min-samd51g", feature = "max-channels"))]
#[macro_export]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro!(32);
    };
}

#[cfg(all(feature = "samd11", not(feature = "max-channels")))]
#[macro_export]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro!(3);
    };
}

#[cfg(all(feature = "samd21", not(feature = "max-channels")))]
#[macro_export]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro!(6);
    };
}

#[cfg(all(feature = "min-samd51g", not(feature = "max-channels")))]
#[macro_export]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro!(16);
    };
}

macro_rules! get {
    ($literal:literal) => {
        $literal
    };
}

/// Number of DMA channels used by the driver
pub const NUM_CHANNELS: usize = with_num_channels!(get);

#[bitfield]
#[derive(Clone, Copy)]
#[repr(u16)]
pub struct BlockTransferControl {
    #[allow(dead_code)]
    valid: bool,
    #[allow(dead_code)]
    evosel: B2,
    #[allow(dead_code)]
    blockact: B2,
    #[skip]
    _reserved: B3,
    #[bits = 2]
    #[allow(dead_code)]
    beatsize: BeatSize,
    #[allow(dead_code)]
    srcinc: bool,
    #[allow(dead_code)]
    dstinc: bool,
    #[allow(dead_code)]
    stepsel: bool,
    #[allow(dead_code)]
    stepsize: B3,
}

// ----- DMAC SRAM registers ----- //
/// Descriptor representing a SRAM register. Datasheet section 19.8.2
#[derive(Clone, Copy)]
#[repr(C, align(16))]
#[doc(hidden)]
pub struct DmacDescriptor {
    btctrl: BlockTransferControl,
    btcnt: u16,
    srcaddr: *const (),
    dstaddr: *const (),
    descaddr: *const DmacDescriptor,
}

#[doc(hidden)]
pub const DEFAULT_DESCRIPTOR: DmacDescriptor = DmacDescriptor {
    btctrl: BlockTransferControl::new(),
    btcnt: 0,
    srcaddr: 0 as *mut _,
    dstaddr: 0 as *mut _,
    descaddr: 0 as *mut _,
};

// Writeback section. This static variable should never be written to in an
// interrupt or thread context.
#[doc(hidden)]
static mut WRITEBACK: [DmacDescriptor; NUM_CHANNELS] = [DEFAULT_DESCRIPTOR; NUM_CHANNELS];
// Descriptor section. This static variable should never be written to in an
// interrupt or thread context.
#[doc(hidden)]
static mut DESCRIPTOR_SECTION: [DmacDescriptor; NUM_CHANNELS] = [DEFAULT_DESCRIPTOR; NUM_CHANNELS];

pub mod channel;
pub mod dma_controller;
pub mod transfer;
