//! # DMA transfer abstractions
//!
//! # Transfer types
//!
//! Four basic transfer types are supported:
//!
//! * Incrementing-source to incrementing-destination
//! (normally used for memory-to-memory transfers)
//!
//! * Incrementing-source to fixed-destination (normally used
//! for memory-to-peripheral transfers)
//!
//! * Fixed-source to incrementing-destination (normally used for
//! peripheral-to-memory transfers)
//!
//! * Fixed-source to fixed-destination (normally used for
//! peripheral-to-peripheral transfers)
//!
//! # Beat sizes
//!
//! A beat is an atomic, uninterruptible transfer size.Three beat sizes are
//! supported:
//!
//! * Byte-per-byte (8 bit beats);
//!
//! * Halfword-per-halfword (16 bit beats);
//!
//! * Word-per-word (32 bit beats);
//!
//! The correct beat size will automatically be selected in function of the type
//! of the source and destination buffers.
//!
//! # One-shot vs circular transfers
//!
//! If the transfer is setup as one-shot (`circular == false`), the
//! transfer will run once and stop. Otherwise, if `circular == true`, then the
//! transfer will be set as circular, meaning it will restart a new, identical
//! block transfer when the current block transfer is complete. This is
//! particularly useful when combined with a TC/TCC trigger source, for instance
//! to periodically retreive a sample from an ADC and send it to a circular
//! buffer, or send a sample to a DAC.
//!
//! # Starting a transfer
//!
//! A transfer is started by calling [`begin`](Transfer::begin). You will be
//! required to supply a trigger source and a trigger action.
//!
//! # Waiting for a transfer to complete
//!
//! A transfer can waited upon by calling [`wait`](Transfer::wait). This is a
//! _blocking_ method, meaning it will busy-wait until the transfer is
//! completed. When it returns, it will release the source and destination
//! buffers, as well as the DMA channel and the payload.
//!
//! # Interrupting (stopping) a transfer
//!
//! A transfer can be stopped (regardless of whether it has completed or not) by
//! calling [`stop`](Transfer::stop). This is _not_ a blocking method,
//! meaning it will stop the transfer and immediately return. When it returns,
//! it will release the source and destination buffers, as well as the DMA
//! channel and the payload.
//!
//! # Trigger sources
//!
//! Most peripherals can issue triggers to a DMA channel. A software trigger is
//! also available (see [`trigger`](Transfer::software_trigger)). See
//! ATSAMD21 datasheet, table 19-8 for all available trigger sources.
//!
//! # Trigger actions
//!
//! Three trigger actions are available:
//!
//! * BLOCK: One trigger required for each block transfer. In the context of
//!   this driver,
//! one Transfer is equivalent to one Block transfer.
//!
//! * BEAT: One trigger required for each beat transfer. In the context of this
//!   driver, the beat
//! size will depend on the type of buffer used (8, 16 or 32 bits).
//!
//! * TRANSACTION: One trigger required for a full DMA transaction. this is
//!   useful for circular
//! transfers in the context of this driver. One trigger will set off the
//! transaction, that will now run uninterrupted until it is stopped.

use core::sync::atomic;
use modular_bitfield::prelude::*;

use super::{
    channel::{Busy, Channel, Ready, Status},
    dma_controller::{DmaController, TriggerAction, TriggerSource},
    BlockTransferControl, DmacDescriptor, DESCRIPTOR_SECTION,
};
use crate::typelevel::{Is, Sealed};
use core::mem;

//==============================================================================
// Beat
//==============================================================================

/// Useable beat sizes for DMA transfers
#[derive(Clone, Copy, BitfieldSpecifier)]
pub enum BeatSize {
    /// Byte = [`u8`](core::u8)
    Byte = 0x00,
    /// Half word = [`u16`](core::u16)
    HalfWord = 0x01,
    /// Word = [`u32`](core::u32)
    Word = 0x02,
    _RESERVED = 0x03,
}
/// Convert 8, 16 and 32 bit types
/// into [`BeatSize`](BeatSize)
pub trait Beat: Sealed {
    /// Convert to BeatSize enum
    const BEATSIZE: BeatSize;
}

macro_rules! impl_beat {
    ( $( ($Type:ty, $Size:ident) ),+ ) => {
        $(
            impl Sealed for $Type {}
            impl Beat for $Type {
                const BEATSIZE: BeatSize = BeatSize::$Size;
            }
        )+
    };
}

impl_beat!(
    (u8, Byte),
    (i8, Byte),
    (u16, HalfWord),
    (i16, HalfWord),
    (u32, Word),
    (i32, Word),
    (f32, Word)
);

//==============================================================================
// Buffer
//==============================================================================

/// Buffer useable by the DMAC.
pub unsafe trait Buffer {
    type Beat: Beat;
    /// Pointer to the buffer. If the buffer is incrementing, the address should
    /// point to the last beat transfer in the block.
    fn dma_ptr(&mut self) -> *mut Self::Beat;
    /// Return whether the buffer pointer should be incrementing or not
    fn incrementing(&self) -> bool;
    /// Buffer length
    fn buffer_len(&self) -> usize;
}

unsafe impl<T: Beat, const N: usize> Buffer for &mut [T; N] {
    type Beat = T;
    #[inline]
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        let ptrs = self.as_mut_ptr_range();
        if self.incrementing() {
            ptrs.end
        } else {
            ptrs.start
        }
    }

    #[inline]
    fn incrementing(&self) -> bool {
        N > 1
    }

    #[inline]
    fn buffer_len(&self) -> usize {
        N
    }
}

unsafe impl<T: Beat> Buffer for &mut [T] {
    type Beat = T;
    #[inline]
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        let ptrs = self.as_mut_ptr_range();
        if self.incrementing() {
            ptrs.end
        } else {
            ptrs.start
        }
    }

    #[inline]
    fn incrementing(&self) -> bool {
        self.len() > 1
    }

    #[inline]
    fn buffer_len(&self) -> usize {
        self.len()
    }
}

unsafe impl<T: Beat> Buffer for &mut T {
    type Beat = T;
    #[inline]
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        *self as *mut T
    }

    #[inline]
    fn incrementing(&self) -> bool {
        false
    }

    #[inline]
    fn buffer_len(&self) -> usize {
        1
    }
}

//==============================================================================
// BufferPair
//==============================================================================

/// Struct holding the source and destination buffers of a
/// [`Transfer`](Transfer).
pub struct BufferPair<S, D = S>
where
    S: Buffer,
    D: Buffer<Beat = S::Beat>,
{
    pub source: S,
    pub destination: D,
}

//==============================================================================
// AnyBufferPair
//==============================================================================

pub trait AnyBufferPair: Sealed + Is<Type = SpecificBufferPair<Self>> {
    type Src: Buffer;
    type Dst: Buffer<Beat = BufferPairBeat<Self>>;
}

pub type SpecificBufferPair<C> = BufferPair<<C as AnyBufferPair>::Src, <C as AnyBufferPair>::Dst>;

pub type BufferPairSrc<B> = <B as AnyBufferPair>::Src;
pub type BufferPairDst<B> = <B as AnyBufferPair>::Dst;
pub type BufferPairBeat<B> = <BufferPairSrc<B> as Buffer>::Beat;

impl<S, D> Sealed for BufferPair<S, D>
where
    S: Buffer,
    D: Buffer<Beat = S::Beat>,
{
}

impl<S, D> AnyBufferPair for BufferPair<S, D>
where
    S: Buffer,
    D: Buffer<Beat = S::Beat>,
{
    type Src = S;
    type Dst = D;
}

impl<B: AnyBufferPair> AsRef<B> for SpecificBufferPair<B> {
    #[inline]
    fn as_ref(&self) -> &B {
        // SAFETY: This is guaranteed to be safe, because B == SpecificBufferPair<B>
        unsafe { mem::transmute(self) }
    }
}

impl<B: AnyBufferPair> AsMut<B> for SpecificBufferPair<B> {
    #[inline]
    fn as_mut(&mut self) -> &mut B {
        // SAFETY: This is guaranteed to be safe, because B == SpecificBufferPair<B>
        unsafe { mem::transmute(self) }
    }
}

// TODO change source and dest types to Pin? (see https://docs.rust-embedded.org/embedonomicon/dma.html#immovable-buffers)
/// DMA transfer, owning the resources until the transfer is done and
/// [`wait`](Transfer::wait) is called.
pub struct Transfer<B, Pld, ChanStatus, const ID: u8>
where
    B: AnyBufferPair,
    ChanStatus: Status,
{
    buffers: B,
    chan: Channel<ChanStatus, ID>,
    payload: Pld,
}

/// These methods are available to an `Transfer` holding a `Ready` channel
impl<B, P, const ID: u8> Transfer<B, P, Ready, ID>
where
    B: AnyBufferPair,
{
    /// Safely construct a new `Transfer`. To guarantee memory safety, both
    /// buffers are required to be `'static`.
    /// Refer [here](https://docs.rust-embedded.org/embedonomicon/dma.html#memforget) or
    /// [here](https://blog.japaric.io/safe-dma/) for more information.
    ///
    /// # Panics
    ///
    /// Panics if both buffers have a length > 1 and are not of equal length.
    pub fn new(buffers: B, chan: Channel<Ready, ID>, circular: bool, payload: P) -> Self
    where
        B::Src: 'static,
        B::Dst: 'static,
    {
        let src_len = buffers.as_ref().source.buffer_len();
        let dst_len = buffers.as_ref().destination.buffer_len();

        if src_len > 1 && dst_len > 1 {
            assert_eq!(src_len, dst_len);
        }

        // SAFETY: The safety checks are done by the function signature and the buffer
        // length verification
        unsafe { Self::new_unchecked(buffers, chan, circular, payload) }
    }

    /// Construct a new `Transfer` without checking for memory safety.
    ///
    /// # Safety
    ///
    /// To guarantee the safety of creating a `Transfer` using this method, you
    /// must uphold some invariants:
    ///
    /// * A `Transfer` holding a `Channel<Running, ID>` must *never* be dropped.
    ///   It should *always* be explicitly be `wait`ed upon or `stop`ped.
    ///
    /// * The size in bytes or the source and destination buffers should be
    ///   exacly the same, unless one or both buffers are of length 1. The
    ///   transfer length will be set to the longest of both buffers if they are
    ///   not of equal size.
    pub unsafe fn new_unchecked(
        mut buffers: B,
        chan: Channel<Ready, ID>,
        circular: bool,
        payload: P,
    ) -> Self {
        // Enable support for circular transfers. If circular_xfer is true,
        // we set the address of the "next" block descriptor to actually
        // be the same address as the current block descriptor.
        // Otherwise we set it to 0 (terminates the transaction)
        // TODO: Enable support for linked lists (?)
        #[allow(clippy::zero_ptr)]
        let descaddr = if circular {
            // SAFETY This is safe as we are only reading the descriptor's address,
            // and not actually writing any data to it. We also assume the descriptor
            // will never be moved.
            &mut DESCRIPTOR_SECTION[ID as usize] as *mut _
        } else {
            0 as *mut _
        };

        let source = &mut buffers.as_mut().source;
        let src_ptr = source.dma_ptr();
        let src_inc = source.incrementing();
        let src_len = source.buffer_len();

        let dest = &mut buffers.as_mut().destination;
        let dst_ptr = dest.dma_ptr();
        let dst_inc = dest.incrementing();
        let dst_len = dest.buffer_len();

        let length = core::cmp::max(src_len, dst_len);

        let btctrl = BlockTransferControl::new()
            .with_srcinc(src_inc)
            .with_dstinc(dst_inc)
            .with_beatsize(BufferPairBeat::<B>::BEATSIZE)
            .with_valid(true);

        let xfer_descriptor = DmacDescriptor {
            // Next descriptor address:  0x0 terminates the transaction (no linked list),
            // any other address points to the next block descriptor
            descaddr,
            // Source address: address of the last beat transfer source in block
            srcaddr: src_ptr as *mut _,
            // Destination address: address of the last beat transfer destination in block
            dstaddr: dst_ptr as *mut _,
            // Block transfer count: number of beats in block transfer
            btcnt: length as u16,
            // Block transfer control: Datasheet  section 19.8.2.1 p.329
            btctrl,
        };

        // SAFETY this is safe as long as we ONLY write to the descriptor
        // belonging to OUR channel. We assume this is the only place
        // in the entire library that this section or the array
        // will be written to.
        DESCRIPTOR_SECTION[ID as usize] = xfer_descriptor;

        Transfer {
            buffers,
            chan,
            payload,
        }
    }

    /// Begin DMA transfer. If [TriggerSource::DISABLE](TriggerSource::DISABLE)
    /// is used, a sowftware trigger will be issued to the DMA channel to
    /// launch the transfer. Is is therefore not necessary, in most cases,
    /// to manually issue a software trigger to the channel.
    pub fn begin(
        self,
        dmac: &mut DmaController,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
    ) -> Transfer<B, P, Busy, ID> {
        // Memory barrier to prevent the compiler/CPU from re-ordering read/write
        // operations beyond this fence.
        // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
        atomic::fence(atomic::Ordering::Release); //  ▲

        // SAFETY: This is safe because we only borrow dmac once
        let dmac = unsafe { dmac.dmac_mut() };
        let chan = self.chan.start(dmac, trig_src, trig_act);

        Transfer {
            buffers: self.buffers,
            chan,
            payload: self.payload,
        }
    }
}

/// These methods are available to a `Transfer` holding a `Ready` channel and a
/// `BufferPair` holding two arrays of equal type and length
impl<B, P, const N: usize, const ID: u8> Transfer<BufferPair<&'static mut [B; N]>, P, Ready, ID>
where
    B: 'static + Beat,
{
    /// Create a new `Transfer` from static array references of the same type
    /// and length. When two array references are available (instead of slice
    /// references), it is recommended to use this function over
    /// [`Transfer::new`](Transfer::new), because it provides compile-time
    /// checking that the array lengths match. It therefore does not panic, and
    /// saves some runtime checking of the array lengths.
    pub fn new_from_arrays(
        buffers: BufferPair<&'static mut [B; N]>,
        chan: Channel<Ready, ID>,
        circular: bool,
        payload: P,
    ) -> Self {
        unsafe { Self::new_unchecked(buffers, chan, circular, payload) }
    }
}

/// These methods are available to a `Transfer` holding a `Busy` channel
impl<B, P, const ID: u8> Transfer<B, P, Busy, ID>
where
    B: AnyBufferPair,
{
    /// Issue a software trigger request to the corresponding channel.
    /// Note that is not guaranteed that the trigger request will register,
    /// if a trigger request is already pending for the channel.
    #[inline]
    pub fn software_trigger(&mut self, dmac: &mut DmaController) {
        // SAFETY: This is safe because we only borrow dmac once.
        let dmac = unsafe { dmac.dmac_mut() };
        self.chan.software_trigger(dmac);
    }
    /// Blocking; Wait for the DMA transfer to complete and release all owned
    /// resources
    pub fn wait(self, dmac: &mut DmaController) -> (B, Channel<Ready, ID>, P) {
        // SAFETY: This is safe because we only borrow dmac once.
        let dmac = unsafe { dmac.dmac_mut() };
        let chan = self.chan.free(dmac);

        // Memory barrier to prevent the compiler/CPU from re-ordering read/write
        // operations beyond this fence.
        // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
        atomic::fence(atomic::Ordering::Acquire); // ▼

        (self.buffers, chan, self.payload)
    }

    /// Non-blocking; Immediately stop the DMA transfer and release all owned
    /// resources
    pub fn stop(self, dmac: &mut DmaController) -> (B, Channel<Ready, ID>, P) {
        // SAFETY: This is safe because we only borrow dmac once.
        let dmac = unsafe { dmac.dmac_mut() };
        let chan = self.chan.stop(dmac);

        // Memory barrier to prevent the compiler/CPU from re-ordering read/write
        // operations beyond this fence.
        // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
        atomic::fence(atomic::Ordering::Acquire); // ▼

        (self.buffers, chan, self.payload)
    }
}
