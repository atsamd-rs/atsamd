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
//! A transfer is started by calling [`Transfer::begin`]. You will be
//! required to supply a trigger source and a trigger action.
//!
//! # Waiting for a transfer to complete
//!
//! A transfer can waited upon by calling [`wait`](Transfer::wait). This is a
//! _blocking_ method, meaning it will busy-wait until the transfer is
//! completed. When it returns, it will release the source and destination
//! buffers, as well as the DMA channel.
//!
//! # Interrupting (stopping) a transfer
//!
//! A transfer can be stopped (regardless of whether it has completed or not) by
//! calling [`stop`](Transfer::stop). This is _not_ a blocking method,
//! meaning it will stop the transfer and immediately return. When it returns,
//! it will release the source and destination buffers, as well as the DMA
//! channel.
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

use super::{
    channel::{AnyChannel, Busy, CallbackStatus, Channel, ChannelId, InterruptFlags, Ready},
    dma_controller::{ChId, TriggerAction, TriggerSource},
    BlockTransferControl, DmacDescriptor, Error, Result, DESCRIPTOR_SECTION,
};
use crate::typelevel::{Is, Sealed};
use core::{ptr::null_mut, sync::atomic};
use modular_bitfield::prelude::*;

//==============================================================================
// Beat
//==============================================================================

/// Useable beat sizes for DMA transfers
#[derive(Clone, Copy, BitfieldSpecifier)]
#[bits = 2]
pub enum BeatSize {
    /// Byte = [`u8`](core::u8)
    Byte = 0x00,
    /// Half word = [`u16`](core::u16)
    HalfWord = 0x01,
    /// Word = [`u32`](core::u32)
    Word = 0x02,
}

/// Convert 8, 16 and 32 bit types
/// into [`BeatSize`](BeatSize)
///
/// # Safety
///
/// This trait should not be implemented outside of the crate-provided
/// implementations
pub unsafe trait Beat: Sealed {
    /// Convert to BeatSize enum
    const BEATSIZE: BeatSize;
}

macro_rules! impl_beat {
    ( $( ($Type:ty, $Size:ident) ),+ ) => {
        $(
            unsafe impl Beat for $Type {
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
///
/// # Safety
///
/// This trait should only be implemented for valid DMAC sources/sinks. That is,
/// you need to make sure that:
/// * `dma_ptr` points to a valid memory location useable by the DMAC
/// * `incrementing` is correct for the source/sink. For example, an `&[u8]` of
///   size one is not incrementing.
/// * `buffer_len` is correct for the source/sink.
pub unsafe trait Buffer {
    /// DMAC beat size
    type Beat: Beat;
    /// Pointer to the buffer. If the buffer is incrementing, the address should
    /// point to one past the last beat transfer in the block.
    fn dma_ptr(&mut self) -> *mut Self::Beat;
    /// Return whether the buffer pointer should be incrementing or not
    fn incrementing(&self) -> bool;
    /// Buffer length in beats
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
/// [`Transfer`].
pub struct BufferPair<S, D = S>
where
    S: Buffer,
    D: Buffer<Beat = S::Beat>,
{
    /// Source buffer
    pub source: S,
    /// Destination buffer
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

impl<S, D> AsRef<Self> for BufferPair<S, D>
where
    S: Buffer,
    D: Buffer<Beat = S::Beat>,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<S, D> AsMut<Self> for BufferPair<S, D>
where
    S: Buffer,
    D: Buffer<Beat = S::Beat>,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

// TODO change source and dest types to Pin? (see https://docs.rust-embedded.org/embedonomicon/dma.html#immovable-buffers)
/// DMA transfer, owning the resources until the transfer is done and
/// [`Transfer::wait`] is called.
pub struct Transfer<Chan, Buf, W = ()>
where
    Buf: AnyBufferPair,
    Chan: AnyChannel,
{
    chan: Chan,
    buffers: Buf,
    waker: Option<W>,
    complete: bool,
}

impl<C, S, D> Transfer<C, BufferPair<S, D>>
where
    S: Buffer + 'static,
    D: Buffer<Beat = S::Beat> + 'static,
    C: AnyChannel<Status = Ready>,
{
    /// Safely construct a new `Transfer`. To guarantee memory safety, both
    /// buffers are required to be `'static`.
    /// Refer [here](https://docs.rust-embedded.org/embedonomicon/dma.html#memforget) or
    /// [here](https://blog.japaric.io/safe-dma/) for more information.
    ///
    /// If two array references can be used as source and destination buffers
    /// (as opposed to slices), then it is recommended to use the
    /// [`Transfer::new_from_arrays`] method instead.
    ///
    /// # Panics
    ///
    /// Panics if both buffers have a length > 1 and are not of equal length.
    #[allow(clippy::new_ret_no_self)]
    #[inline]
    pub fn new(
        chan: C,
        source: S,
        destination: D,
        circular: bool,
    ) -> Result<Transfer<C, BufferPair<S, D>>> {
        Self::check_buffer_pair(&source, &destination)?;

        // SAFETY: The safety checks are done by the function signature and the buffer
        // length verification
        Ok(unsafe { Self::new_unchecked(chan, source, destination, circular) })
    }
}

impl<S, D, C, W> Transfer<C, BufferPair<S, D>, W>
where
    S: Buffer,
    D: Buffer<Beat = S::Beat>,
    C: AnyChannel,
{
    #[inline]
    fn check_buffer_pair(source: &S, destination: &D) -> Result<()> {
        let src_len = source.buffer_len();
        let dst_len = destination.buffer_len();

        if src_len > 1 && dst_len > 1 && src_len != dst_len {
            Err(Error::LengthMismatch)
        } else {
            Ok(())
        }
    }

    #[inline]
    unsafe fn fill_descriptor(source: &mut S, destination: &mut D, circular: bool) {
        let id = <C as AnyChannel>::Id::USIZE;

        // Enable support for circular transfers. If circular_xfer is true,
        // we set the address of the "next" block descriptor to actually
        // be the same address as the current block descriptor.
        // Otherwise we set it to NULL, which terminates the transaction.
        // TODO: Enable support for linked lists (?)
        let descaddr = if circular {
            // SAFETY This is safe as we are only reading the descriptor's address,
            // and not actually writing any data to it. We also assume the descriptor
            // will never be moved.
            &mut DESCRIPTOR_SECTION[id] as *mut _
        } else {
            null_mut()
        };

        let src_ptr = source.dma_ptr();
        let src_inc = source.incrementing();
        let src_len = source.buffer_len();

        let dst_ptr = destination.dma_ptr();
        let dst_inc = destination.incrementing();
        let dst_len = destination.buffer_len();

        let length = core::cmp::max(src_len, dst_len);

        // Channel::xfer_complete() tests the channel enable bit, which indicates
        // that a transfer has completed iff the blockact field in btctrl is not
        // set to SUSPEND.  We implicitly leave blockact set to NOACT here; if
        // that changes Channel::xfer_complete() may need to be modified.
        let btctrl = BlockTransferControl::new()
            .with_srcinc(src_inc)
            .with_dstinc(dst_inc)
            .with_beatsize(S::Beat::BEATSIZE)
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
        DESCRIPTOR_SECTION[id] = xfer_descriptor;
    }
}

impl<C, S, D> Transfer<C, BufferPair<S, D>>
where
    S: Buffer,
    D: Buffer<Beat = S::Beat>,
    C: AnyChannel<Status = Ready>,
{
    /// Construct a new `Transfer` without checking for memory safety.
    ///
    /// # Safety
    ///
    /// To guarantee the safety of creating a `Transfer` using this method, you
    /// must uphold some invariants:
    ///
    /// * A `Transfer` holding a `Channel<Id, Running>` must *never* be dropped.
    ///   It should *always* be explicitly be `wait`ed upon or `stop`ped.
    ///
    /// * The size in bytes or the source and destination buffers should be
    ///   exacly the same, unless one or both buffers are of length 1. The
    ///   transfer length will be set to the longest of both buffers if they are
    ///   not of equal size.
    #[inline]
    pub unsafe fn new_unchecked(
        chan: C,
        mut source: S,
        mut destination: D,
        circular: bool,
    ) -> Transfer<C, BufferPair<S, D>> {
        Self::fill_descriptor(&mut source, &mut destination, circular);

        let buffers = BufferPair {
            source,
            destination,
        };

        Transfer {
            buffers,
            chan,
            waker: None,
            complete: false,
        }
    }
}

impl<C, S, D> Transfer<C, BufferPair<S, D>>
where
    S: Buffer,
    D: Buffer<Beat = S::Beat>,
    C: AnyChannel<Status = Ready>,
{
    /// Append a waker to the transfer. This will be called when the DMAC
    /// interrupt is called.
    #[inline]
    pub fn with_waker<W: FnOnce(CallbackStatus) + 'static>(
        self,
        waker: W,
    ) -> Transfer<C, BufferPair<S, D>, W> {
        Transfer {
            buffers: self.buffers,
            chan: self.chan,
            complete: self.complete,
            waker: Some(waker),
        }
    }
}

impl<C, S, D, W> Transfer<C, BufferPair<S, D>, W>
where
    S: Buffer,
    D: Buffer<Beat = S::Beat>,
    C: AnyChannel<Status = Ready>,
{
    /// Begin DMA transfer. If [TriggerSource::DISABLE](TriggerSource::DISABLE)
    /// is used, a software trigger will be issued to the DMA channel to
    /// launch the transfer. Is is therefore not necessary, in most cases,
    /// to manually issue a software trigger to the channel.
    #[inline]
    pub fn begin(
        mut self,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
    ) -> Transfer<Channel<ChannelId<C>, Busy>, BufferPair<S, D>, W> {
        // Reset the complete flag before triggering the transfer.
        // This way an interrupt handler could set complete to true
        // before this function returns.
        self.complete = false;

        // Memory barrier to prevent the compiler/CPU from re-ordering read/write
        // operations beyond this fence.
        // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
        atomic::fence(atomic::Ordering::Release); //  ▲
        let chan = self.chan.into().start(trig_src, trig_act);

        Transfer {
            buffers: self.buffers,
            chan,
            waker: self.waker,
            complete: self.complete,
        }
    }
}

impl<B, C, const N: usize> Transfer<C, BufferPair<&'static mut [B; N]>>
where
    B: 'static + Beat,
    C: AnyChannel<Status = Ready>,
{
    /// Create a new `Transfer` from static array references of the same type
    /// and length. When two array references are available (instead of slice
    /// references), it is recommended to use this function over
    /// [`Transfer::new`](Transfer::new), because it provides compile-time
    /// checking that the array lengths match. It therefore does not panic, and
    /// saves some runtime checking of the array lengths.
    #[inline]
    pub fn new_from_arrays(
        chan: C,
        source: &'static mut [B; N],
        destination: &'static mut [B; N],
        circular: bool,
    ) -> Self {
        unsafe { Self::new_unchecked(chan, source, destination, circular) }
    }
}

impl<S, D, C, W> Transfer<C, BufferPair<S, D>, W>
where
    S: Buffer,
    D: Buffer<Beat = S::Beat>,
    C: AnyChannel<Status = Busy>,
{
    /// Issue a software trigger request to the corresponding channel.
    /// Note that is not guaranteed that the trigger request will register,
    /// if a trigger request is already pending for the channel.
    #[inline]
    pub fn software_trigger(&mut self) {
        self.chan.as_mut().software_trigger();
    }

    /// Unsafely and mutably borrow the source buffer
    ///
    /// # Safety
    ///
    /// The source buffer should never be borrowed when a transfer is in
    /// progress, as it is getting mutated or read in another context (ie,
    /// the DMAC hardware "thread").
    #[inline]
    pub(crate) unsafe fn borrow_source(&mut self) -> &mut S {
        &mut self.buffers.source
    }

    /// Unsafely and mutably borrow the destination buffer.
    /// # Safety
    ///
    /// The destination buffer should never be borrowed when a transfer is in
    /// progress, as it is getting mutated or read in another context (ie,
    /// the DMAC hardware "thread").
    #[inline]
    pub(crate) unsafe fn borrow_destination(&mut self) -> &mut D {
        &mut self.buffers.destination
    }

    /// Wait for the DMA transfer to complete and release all owned
    /// resources
    ///
    /// # Blocking: This method may block
    #[inline]
    pub fn wait(mut self) -> (Channel<ChannelId<C>, Ready>, S, D) {
        // Wait for transfer to complete
        while !self.complete() {}
        self.stop()
    }

    /// Check if the transfer has completed
    #[inline]
    pub fn complete(&mut self) -> bool {
        if !self.complete {
            let chan = self.chan.as_mut();
            let complete = chan.xfer_complete();
            self.complete = complete;
        }
        self.complete
    }

    /// Checks and clears the block transfer complete interrupt flag
    #[inline]
    pub fn block_transfer_interrupt(&mut self) -> bool {
        self.chan
            .as_mut()
            .check_and_clear_interrupts(InterruptFlags::new().with_tcmpl(true))
            .tcmpl()
    }

    /// Modify a completed transfer with new `source` and `destination`, then
    /// restart.
    ///
    /// Returns a Result containing the source and destination from the
    /// completed transfer. Returns `Err(_)` if the buffer lengths are
    /// mismatched or if the previous transfer has not yet completed.
    #[inline]
    pub fn recycle(&mut self, mut source: S, mut destination: D) -> Result<(S, D)> {
        Self::check_buffer_pair(&source, &destination)?;

        if !self.complete() {
            return Err(Error::InvalidState);
        }

        // Circular transfers won't ever complete, so never re-fill as one
        unsafe {
            Self::fill_descriptor(&mut source, &mut destination, false);
        }

        let new_buffers = BufferPair {
            source,
            destination,
        };

        let old_buffers = core::mem::replace(&mut self.buffers, new_buffers);

        self.chan.as_mut().restart();

        Ok((old_buffers.source, old_buffers.destination))
    }

    /// Modify a completed transfer with a new `destination`, then restart.

    /// Returns a Result containing the destination from the
    /// completed transfer. Returns `Err(_)` if the buffer lengths are
    /// mismatched or if the previous transfer has not yet completed.
    #[inline]
    pub fn recycle_source(&mut self, mut destination: D) -> Result<D> {
        Self::check_buffer_pair(&self.buffers.source, &destination)?;

        if !self.complete() {
            return Err(Error::InvalidState);
        }

        // Circular transfers won't ever complete, so never re-fill as one
        unsafe {
            Self::fill_descriptor(&mut self.buffers.source, &mut destination, false);
        }

        let old_destination = core::mem::replace(&mut self.buffers.destination, destination);

        self.chan.as_mut().restart();

        Ok(old_destination)
    }

    /// Modify a completed transfer with a new `source`, then restart.

    /// Returns a Result containing the source from the
    /// completed transfer. Returns `Err(_)` if the buffer lengths are
    /// mismatched or if the previous transfer has not yet completed.
    #[inline]
    pub fn recycle_destination(&mut self, mut source: S) -> Result<S> {
        Self::check_buffer_pair(&source, &self.buffers.destination)?;

        if !self.complete() {
            return Err(Error::InvalidState);
        }

        // Circular transfers won't ever complete, so never re-fill as one
        unsafe {
            Self::fill_descriptor(&mut source, &mut self.buffers.destination, false);
        }

        let old_source = core::mem::replace(&mut self.buffers.source, source);

        self.chan.as_mut().restart();

        Ok(old_source)
    }

    /// Non-blocking; Immediately stop the DMA transfer and release all owned
    /// resources
    #[inline]
    pub fn stop(self) -> (Channel<ChannelId<C>, Ready>, S, D) {
        let chan = self.chan.into().free();

        // Memory barrier to prevent the compiler/CPU from re-ordering read/write
        // operations beyond this fence.
        // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
        atomic::fence(atomic::Ordering::Acquire); // ▼

        (chan, self.buffers.source, self.buffers.destination)
    }
}

impl<S, D, C, W> Transfer<C, BufferPair<S, D>, W>
where
    S: Buffer,
    D: Buffer<Beat = S::Beat>,
    C: AnyChannel<Status = Busy>,
    W: FnOnce(CallbackStatus) + 'static,
{
    /// This function should be put inside the DMAC interrupt handler.
    /// It will take care of calling the [`Transfer`]'s waker (if it exists).
    #[inline]
    pub fn callback(&mut self) {
        let status = self.chan.as_mut().callback();

        if let CallbackStatus::TransferComplete = status {
            self.complete = true;
        }

        if let Some(w) = self.waker.take() {
            w(status)
        }
    }
}
