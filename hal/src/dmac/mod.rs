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
//! or [here](https://blog.japaric.io/safe-dma/#leakpocalypse) for more information.
//! You may choose to forgo the `'static` lifetimes by using the unsafe API and
//! the [`Transfer::new_unchecked`](transfer::Transfer::new_unchecked) method.
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
//! unsafe [`Buffer`] trait.
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
//!
//! // (Optional) free the [`DmaController`] struct and return the underlying PAC struct
//! channels.0 = chan0.into();
//! let dmac = dmac.free(channels, &mut peripherals.PM);
//! ```
//!
//! # [`Transfer`] recycling
//!
//! A common use-case with DMAC transfers is to trigger a new transfer as soon
//! as the old transfer is completed. To avoid having to
//! [`stop`](Transfer::stop) a [`Transfer`], build a new [`Transfer`] (with
//! [`new`](Transfer::new) or [`new_from_arrays`](Transfer::new_from_arrays))
//! then call [`begin`](Transfer::begin), a [`Transfer::recycle`] method
//! is provided. If the buffer lengths match and the previous transfer is
//! completed, a new transfer will immediately be triggered using the provided
//! source and destination buffers. If the recycling operation is succesful,
//! `Ok((source, destination))` containing the old source and destination
//! buffers is returned. Otherwise, `Err(_)` is returned.
//!
//! ```
//! let new_source = produce_source();
//! let new_destination = produce_destination();
//!
//! // Assume xfer is a `Busy` `Transfer`
//! let (old_source, old_dest) = xfer.recycle(new_source, new_destination).unwrap();
//! ```
//!
//! # Waker operation
//!
//! A [`Transfer`] can also accept a function or closure that will be called on
//! completion of the transaction, acting like a waker.
//!
//! ```
//! fn wake_up() {
//!     //...
//! }
//!
//! fn use_waker<const N: usize>(dmac: DmaController,
//!     source: &'static mut [u8; N],
//!     destination: &'static mut [u8; N]
//! ){
//!     let chan0 = dmac.split().0;
//!     let xfer = Transfer::new_from_arrays(chan0, source, destination, false)
//!         .with_waker(wake_up)
//!         .begin();
//!     //...
//! }
//! ```
//!
//! ## RTIC example
//!
//! The [RTIC] framework provides a convenient way to store a `static`ally
//! allocated [`Transfer`], so that it can be accessed by both the interrupt
//! handlers and user code. The following example shows how [`Transfer`]s might
//! be used for a series of transactions. It uses features from the latest
//! release of [RTIC], `v0.6-alpha.4`.
//!
//! ```
//! use atsamd_hal::dmac::*;
//!
//! const LENGTH: usize = 50;
//! type TransferBuffer = &'static mut [u8; LENGTH];
//! type Xfer = Transfer<Channel<Ch0, Busy>, TransferBuffer, TransferBuffer>;
//!
//! #[resources]
//! struct Resources {
//!     #[lock_free]
//!     #[init(None)]
//!     opt_xfer: Option<Xfer>,
//!
//!     #[lock_free]
//!     #[init(None)]
//!     opt_channel: Option<Channel<Ch0, Ready>>,
//! }
//!
//! // Note: Assume interrupts have already been enabled for the concerned channel
//! #[task(resources = [opt_xfer, opt_channel])]
//! fn task(ctx: task::Context) {
//!     let task::Context { opt_xfer } = ctx;
//!     match opt_xfer {
//!         Some(xfer) => {
//!             if xfer.complete() {
//!                 let (chan0, _source, dest, _payload) = xfer.take().unwrap().stop();
//!                 *opt_channel = Some(chan0);
//!                 consume_data(buf);
//!             }
//!         }
//!         None => {
//!             if let Some(chan0) = opt_channel.take() {
//!                 let source: [u8; 50] = produce_source();
//!                 let dest: [u8; 50] = produce_destination();
//!                 let xfer = opt_xfer.get_or_insert(
//!                     Transfer::new_from_arrays(channel0, source, destination)
//!                         .with_waker(|| { task::spawn().ok(); })
//!                         .begin()
//!                 );
//!             }
//!         }
//!     }
//! }
//!
//! #[task(binds = DMAC, resources = [opt_future])]
//! fn tcmpl(ctx: tcmpl::Context) {
//!     ctx.resources.opt_xfer.as_mut().unwrap().callback();
//! }
//! ```
//! [RTIC]: https://rtic.rs

// This is necessary until modular_bitfield fixes all their identity_op warnings
#![allow(clippy::identity_op)]

use atsamd_hal_macros::hal_cfg;

pub use channel::*;
pub use dma_controller::*;
pub use transfer::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Runtime errors that may occur when dealing with DMA transfers.
pub enum Error {
    /// Supplied buffers both have lengths > 1 beat, but not equal to each other
    ///
    /// Buffers need to either have the same length in beats, or one should have
    /// length == 1.  In cases where one buffer is length 1, that buffer will be
    /// the source or destination of each beat in the transfer.  If both buffers
    /// had length >1, but not equal to each other, then it would not be clear
    /// how to structure the transfer.
    LengthMismatch,

    /// Operation is not valid in the current state of the object.
    InvalidState,
    /// Chip reported an error during transfer
    TransferError,
}

impl From<Error> for crate::sercom::spi::Error {
    fn from(value: Error) -> Self {
        crate::sercom::spi::Error::Dma(value)
    }
}

impl From<Error> for crate::sercom::i2c::Error {
    fn from(value: Error) -> Self {
        crate::sercom::i2c::Error::Dma(value)
    }
}

impl From<Error> for crate::sercom::uart::Error {
    fn from(value: Error) -> Self {
        crate::sercom::uart::Error::Dma(value)
    }
}

/// Result for DMAC operations
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(feature = "max-channels")]
#[hal_cfg("dmac-d11")]
#[macro_export]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro! {6}
    };
}

#[cfg(feature = "max-channels")]
#[hal_cfg("dmac-d21")]
#[macro_export]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro! {12}
    };
}

#[cfg(feature = "max-channels")]
#[hal_cfg("dmac-d5x")]
#[macro_export]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro! {32}
    };
}

#[cfg(not(feature = "max-channels"))]
#[hal_cfg("dmac-d11")]
#[macro_export]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro! {3}
    };
}

#[cfg(not(feature = "max-channels"))]
#[hal_cfg("dmac-d21")]
#[macro_export]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro! {6}
    };
}

#[cfg(not(feature = "max-channels"))]
#[hal_cfg("dmac-d5x")]
#[macro_export]
macro_rules! with_num_channels {
    ($some_macro:ident) => {
        $some_macro! {16}
    };
}

macro_rules! get {
    ($literal:literal) => {
        $literal
    };
}

/// Number of DMA channels used by the driver
pub const NUM_CHANNELS: usize = with_num_channels!(get);

/// DMAC SRAM registers
pub(crate) mod sram {
    #![allow(dead_code, unused_braces)]

    use core::cell::UnsafeCell;

    use super::{BeatSize, NUM_CHANNELS};

    use modular_bitfield::{
        bitfield,
        specifiers::{B2, B3},
    };

    /// Wrapper type around a [`DmacDescriptor`] to allow interior mutability
    /// while keeping them in static storage
    #[repr(transparent)]
    pub struct DescriptorCell(UnsafeCell<DmacDescriptor>);

    impl DescriptorCell {
        const fn default() -> Self {
            Self(UnsafeCell::new(DmacDescriptor::default()))
        }
    }

    // DescriptorCell is not not *really* sync; we must manually uphold the sync
    // guarantees on every access.
    unsafe impl Sync for DescriptorCell {}

    impl core::ops::Deref for DescriptorCell {
        type Target = UnsafeCell<DmacDescriptor>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl core::ops::DerefMut for DescriptorCell {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    /// Bitfield representing the BTCTRL SRAM DMAC register
    #[allow(unused_braces)]
    #[bitfield]
    #[derive(Clone, Copy)]
    #[repr(u16)]
    pub(super) struct BlockTransferControl {
        pub(super) valid: bool,
        pub(super) evosel: B2,
        pub(super) blockact: B2,
        #[skip]
        _reserved: B3,
        #[bits = 2]
        pub(super) beatsize: BeatSize,
        pub(super) srcinc: bool,
        pub(super) dstinc: bool,
        pub(super) stepsel: bool,
        pub(super) stepsize: B3,
    }

    impl Default for BlockTransferControl {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Descriptor representing a SRAM register. Datasheet section 19.8.2
    #[derive(Clone, Copy)]
    #[repr(C, align(16))]
    pub struct DmacDescriptor {
        pub(super) btctrl: BlockTransferControl,
        pub(super) btcnt: u16,
        pub(super) srcaddr: *const (),
        pub(super) dstaddr: *const (),
        pub(super) descaddr: *const DmacDescriptor,
    }

    impl DmacDescriptor {
        pub const fn default() -> Self {
            Self {
                btctrl: BlockTransferControl::new(),
                btcnt: 0,
                srcaddr: 0 as *mut _,
                dstaddr: 0 as *mut _,
                descaddr: 0 as *mut _,
            }
        }

        pub fn next_descriptor(&self) -> *const DmacDescriptor {
            self.descaddr
        }

        pub fn set_next_descriptor(&mut self, next: *mut DmacDescriptor) {
            self.descaddr = next;
        }

        pub fn beat_count(&self) -> u16 {
            self.btcnt
        }
    }

    /// Writeback section.
    ///
    /// # Safety
    ///
    /// This variable should never be accessed. The only thing we need
    /// to know about it is its starting address, given by
    /// [`writeback_addr`].
    static WRITEBACK: [DescriptorCell; NUM_CHANNELS] =
        [const { DescriptorCell::default() }; NUM_CHANNELS];

    // We only ever need to know its starting address.
    pub(super) fn writeback_addr() -> *mut DmacDescriptor {
        WRITEBACK[0].get()
    }

    /// Descriptor section.
    ///
    /// # Safety
    ///
    /// All accesses to this variable should be synchronized. Elements of the
    /// array should only ever be accessed using [`UnsafeCell::get`]. Any other
    /// access method, such as taking a reference to the [`UnsafeCell`] itself,
    /// is UB and *will* break DMA transfers - speaking from personal
    /// experience.
    static DESCRIPTOR_SECTION: [DescriptorCell; NUM_CHANNELS] =
        [const { DescriptorCell::default() }; NUM_CHANNELS];

    #[inline]
    pub(super) fn descriptor_section_addr() -> *mut DmacDescriptor {
        DESCRIPTOR_SECTION[0].get()
    }

    /// Get a mutable pointer to the specified channel's DMAC descriptor
    ///
    /// # Safety
    ///
    /// The caller must manually synchronize any access to the pointee
    /// [`DmacDescriptor`].
    ///
    /// Additionnally, if the pointer is used to create references to
    /// [`DmacDescriptor`], the caller must guarantee that there will **never**
    /// be overlapping `&mut` references (or overlapping `&mut` and `&`
    /// references) to the pointee *at any given time*, as it would be
    /// instantaneous undefined behaviour.
    #[inline]
    pub(super) unsafe fn get_descriptor(channel_id: usize) -> *mut DmacDescriptor {
        DESCRIPTOR_SECTION[channel_id].get()
    }
}

pub mod channel;
pub mod dma_controller;
pub mod transfer;

#[cfg(feature = "async")]
pub mod async_api;
#[cfg(feature = "async")]
pub use async_api::*;

#[cfg(feature = "async")]
mod waker {
    use embassy_sync::waitqueue::AtomicWaker;

    #[allow(clippy::declare_interior_mutable_const)]
    const NEW_WAKER: AtomicWaker = AtomicWaker::new();
    pub(super) static WAKERS: [AtomicWaker; with_num_channels!(get)] =
        [NEW_WAKER; with_num_channels!(get)];
}
