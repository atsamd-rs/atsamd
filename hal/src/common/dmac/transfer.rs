//! # DMA transfer abstractions
//!
//! # Transfer types
//!
//! Four transfer types are supported:
//!
//! * Incrementing-source to incrementing-destination
//! (normally used for memory-to-memory transfers) - see
//! [`inc_src_inc_dest`](Transfer::inc_src_inc_dest)
//!
//! * Incrementing-source to fixed-destination (normally used
//! for memory-to-peripheral transfers) - see
//! [`inc_src_fixed_dest`](Transfer::inc_src_fixed_dest)
//!
//! * Fixed-source to incrementing-destination (normally used for
//! peripheral-to-memory transfers) - see
//! [`fixed_src_inc_dest`](Transfer::fixed_src_inc_dest)
//!
//! * Fixed-source to fixed-destination (normally used for
//! peripheral-to-peripheral transfers) - see
//! [`fixed_src_fixed_dest`](Transfer::fixed_src_fixed_dest)
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
//! If the transfer is setup as one-shot (`circular_xfer == false`), the
//! transfer will run once and stop. Otherwise, if it is setup as "circular"
//! `circular_xfer == true`, then the transfer will be set as circular, meaning
//! it will restart a new, identical block transfer when the current block
//! transfer is complete. This is particularly useful when combined with a
//! TC/TCC trigger source, for instance to periodically retreive a sample from
//! an ADC and send it to a circular buffer, or send a sample to a DAC.
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

use core::{marker::PhantomData, sync::atomic};

use super::{
    channel::{Busy, Channel, Ready, Status},
    dma_controller::{DmaController, TriggerAction, TriggerSource},
    DmacDescriptor, DESCRIPTOR_SECTION,
};
use crate::typelevel::Sealed;

// TODO change source and dest types to Pin (see https://docs.rust-embedded.org/embedonomicon/dma.html#immovable-buffers)
/// DMA transfer, owning the resources until the transfer is done and
/// [`wait`](Transfer::wait) is called.
pub struct Transfer<C, Pld, ChanStatus, const ID: u8>
where
    C: TransferConfiguration,
    ChanStatus: Status,
{
    buffers: C,
    chan: Channel<ChanStatus, ID>,
    payload: Pld,
}

pub struct BufferPair<B, S, D>
where
    B: Beat,
    S: Buffer<B>,
    D: Buffer<B>,
{
    pub source: S,
    pub destination: D,
    pub _b: PhantomData<B>,
}

/// Configuration for a DMA transfer.
pub unsafe trait TransferConfiguration: Sized {
    type Beat: Beat;

    /// Length of DMA transfer in Beat sizes
    fn xfer_length(&self) -> usize;
    /// Pointer to DMA source buffer.
    fn src_ptr(&mut self) -> (*mut Self::Beat, bool);
    /// Pointer to DMA destination buffer.
    fn dest_ptr(&mut self) -> (*mut Self::Beat, bool);

    /// Setup a DMA transfer.
    ///
    /// If `circular_xfer == false`, the transfer will run once
    /// and stop. If `circular_xfer == true`, then the transfer will be set as
    /// circular, meaning it will restart a new, identical block transfer
    /// when the current block transfer is complete.
    ///
    /// `payload` is just a way for the transfer to own resources
    /// while the transfer is ongoing. For instance, a SERCOM instance could be
    /// moved into the transfer to prevent data races until the transfer is
    /// complete. Calling [`wait`](Transfer::wait) will release the
    /// payload for reuse.
    #[inline]
    fn setup_xfer<P, const ID: u8>(
        self,
        chan: Channel<Ready, ID>,
        circular: bool,
        payload: P,
    ) -> Transfer<Self, P, Ready, ID> {
        Transfer::setup(self, chan, circular, payload)
    }
}

/// Incrementing source to fixed destination. Useful for Memory -> Peripheral
/// transfers
unsafe impl<B> TransferConfiguration for BufferPair<B, &'static mut [B], &'static mut B>
where
    B: 'static + Beat,
{
    type Beat = B;

    #[inline]
    fn xfer_length(&self) -> usize {
        self.source.buffer_len()
    }

    #[inline]
    fn src_ptr(&mut self) -> (*mut Self::Beat, bool) {
        (self.source.to_dma_ptr(), self.source.incrementing())
    }

    #[inline]
    fn dest_ptr(&mut self) -> (*mut Self::Beat, bool) {
        (
            self.destination.to_dma_ptr(),
            self.destination.incrementing(),
        )
    }
}

/// Fixed source to incrementing destination. Useful for Peripheral -> Memory
/// transfers
unsafe impl<B> TransferConfiguration for BufferPair<B, &'static mut B, &'static mut [B]>
where
    B: 'static + Beat,
{
    type Beat = B;

    #[inline]
    fn xfer_length(&self) -> usize {
        self.destination.buffer_len()
    }

    #[inline]
    fn src_ptr(&mut self) -> (*mut Self::Beat, bool) {
        (self.source.to_dma_ptr(), self.source.incrementing())
    }

    #[inline]
    fn dest_ptr(&mut self) -> (*mut Self::Beat, bool) {
        (
            self.destination.to_dma_ptr(),
            self.destination.incrementing(),
        )
    }
}

/// Fixed source to fixed destination. Useful for Peripheral -> Peripheral
/// transfers
unsafe impl<B> TransferConfiguration for BufferPair<B, &'static mut B, &'static mut B>
where
    B: 'static + Beat,
{
    type Beat = B;

    #[inline]
    fn xfer_length(&self) -> usize {
        self.source.buffer_len()
    }

    #[inline]
    fn src_ptr(&mut self) -> (*mut Self::Beat, bool) {
        (self.source.to_dma_ptr(), self.source.incrementing())
    }

    #[inline]
    fn dest_ptr(&mut self) -> (*mut Self::Beat, bool) {
        (
            self.destination.to_dma_ptr(),
            self.destination.incrementing(),
        )
    }
}

/// Incrementing source to incrementing destination. Useful for Memory -> Memory
/// transfers.
///
/// This takes array references (`[B; N]`) instead of slices (`&[B]`) to
/// *statically* ensure that the source buffer cannot be longer than the
/// destination buffer, which would introduce undefined behaviour by overwriting
/// some memory not owned.
unsafe impl<B, const N: usize> TransferConfiguration
    for BufferPair<B, &'static mut [B; N], &'static mut [B; N]>
where
    B: 'static + Beat,
{
    type Beat = B;

    #[inline]
    fn xfer_length(&self) -> usize {
        self.source.buffer_len()
    }

    #[inline]
    fn src_ptr(&mut self) -> (*mut Self::Beat, bool) {
        (self.source.to_dma_ptr(), self.source.incrementing())
    }

    #[inline]
    fn dest_ptr(&mut self) -> (*mut Self::Beat, bool) {
        (
            self.destination.to_dma_ptr(),
            self.destination.incrementing(),
        )
    }
}

/// Incrementing source to incrementing destination. Useful for Memory -> Memory
/// transfers.
///
/// This takes array slices (`&[B]`) to
/// *dynamically* ensure that the source buffer is as the same length as the
/// destination buffer.
///
/// # Panics
///
/// `xfer_length` will panic is the source length is not equal to the
/// destination length.
unsafe impl<B> TransferConfiguration for BufferPair<B, &'static mut [B], &'static mut [B]>
where
    B: 'static + Beat,
{
    type Beat = B;

    #[inline]
    fn xfer_length(&self) -> usize {
        if self.source.buffer_len() == self.destination.buffer_len() {
            self.source.buffer_len()
        } else {
            panic!("Source buffer length is not equal to destination buffer length.")
        }
    }

    #[inline]
    fn src_ptr(&mut self) -> (*mut Self::Beat, bool) {
        (self.source.to_dma_ptr(), self.source.incrementing())
    }

    #[inline]
    fn dest_ptr(&mut self) -> (*mut Self::Beat, bool) {
        (
            self.destination.to_dma_ptr(),
            self.destination.incrementing(),
        )
    }
}

/// These methods are available to an `Transfer` holding a `Ready` channel
impl<C, P, const ID: u8> Transfer<C, P, Ready, ID>
where
    C: TransferConfiguration,
{
    fn setup(mut buffers: C, chan: Channel<Ready, ID>, circular_xfer: bool, payload: P) -> Self {
        // Enable support for circular transfers. If circular_xfer is true,
        // we set the address of the "next" block descriptor to actually
        // be the same address as the current block descriptor.
        // Otherwise we set it to 0 (terminates the transaction)
        // TODO: Enable support for linked lists (?)
        let descaddr = if circular_xfer {
            // SAFETY: This is safe as we are only reading the descriptor's address,
            // and not actually writing any data to it. We also assume the descriptor
            // will never be moved.
            unsafe { &DESCRIPTOR_SECTION[ID as usize] as *const _ as u32 }
        } else {
            0
        };

        let (src, src_inc) = buffers.src_ptr();
        let (dst, dst_inc) = buffers.dest_ptr();
        let length = buffers.xfer_length() as u16;

        let xfer_descriptor = DmacDescriptor {
            // Next descriptor address:  0x0 terminates the transaction (no linked list),
            // any other address points to the next block descriptor
            descaddr,
            // Source address: address of the last beat transfer source in block
            srcaddr: src as u32,
            // Destination address: address of the last beat transfer destination in block
            dstaddr: dst as u32,
            // Block transfer count: number of beats in block transfer
            btcnt: length,
            // Block transfer control: Datasheet  section 19.8.2.1 p.329
            btctrl: ((src_inc as u16) << 10) // Increment source address?
                | ((dst_inc as u16) << 11)   // Tncrement dest address?
                | ((C::Beat::BEATSIZE as u16) << 8)           // Beatsize
                | (1 << 0), // Validate descriptor
        };

        // SAFETY this is safe as long as we ONLY write to the descriptor
        // belonging to OUR channel. We assume this is the only place
        // in the entire library that this section or the array
        // will be written to.
        unsafe {
            DESCRIPTOR_SECTION[ID as usize] = xfer_descriptor;
        }

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
    ) -> Transfer<C, P, Busy, ID> {
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

/// These methods are available to a `Transfer` holding a `Busy` channel
impl<C, P, const ID: u8> Transfer<C, P, Busy, ID>
where
    C: TransferConfiguration,
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
    pub fn wait<B, S, D>(self, dmac: &mut DmaController) -> (C, Channel<Ready, ID>, P)
    where
        B: Beat,
        S: Buffer<B>,
        D: Buffer<B>,
    {
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
    pub fn stop<B, S, D>(self, dmac: &mut DmaController) -> (C, Channel<Ready, ID>, P)
    where
        B: Beat,
        S: Buffer<B>,
        D: Buffer<B>,
    {
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

/// Useable beat sizes for DMA transfers
#[derive(Clone, Copy)]
pub enum BeatSize {
    /// Byte = [`u8`](core::u8)
    Byte = 0x00,
    /// Half word = [`u16`](core::u16)
    HalfWord = 0x01,
    /// Word = [`u32`](core::u32)
    Word = 0x02,
}

// Trait enabling taking `*mut` references to references to a single `T`,
/// references to arrays of `T` or slices of `T`, where `T: Beat`,
/// as well as required transfer length
pub unsafe trait Buffer<T: Beat> {
    /// Take a `*mut` reference to the last object
    /// in the buffer needed by the DMAC. The pointer
    /// changes depending on whether the address should
    fn to_dma_ptr(&mut self) -> *mut T;
    /// Check if the DMA address should be incrementing or not
    fn incrementing(&self) -> bool;
    /// Buffer length
    fn buffer_len(&self) -> usize;
}

unsafe impl<T: Beat, const N: usize> Buffer<T> for &mut [T; N] {
    #[inline]
    fn to_dma_ptr(&mut self) -> *mut T {
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

unsafe impl<T: Beat> Buffer<T> for &mut [T] {
    #[inline]
    fn to_dma_ptr(&mut self) -> *mut T {
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

unsafe impl<T: Beat> Buffer<T> for &mut T {
    #[inline]
    fn to_dma_ptr(&mut self) -> *mut T {
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

unsafe impl<T: Beat> Buffer<T> for *mut T {
    #[inline]
    fn to_dma_ptr(&mut self) -> *mut T {
        *self
    }

    fn incrementing(&self) -> bool {
        false
    }

    #[inline]
    fn buffer_len(&self) -> usize {
        1
    }
}

/// Convert 8, 16 and 32 bit types
/// into [`BeatSize`](BeatSize)
pub trait Beat: Sealed {
    /// Convert to BeatSize enum
    const BEATSIZE: BeatSize;
}

impl Sealed for u8 {}
impl Beat for u8 {
    const BEATSIZE: BeatSize = BeatSize::Byte;
}

impl Sealed for i8 {}
impl Beat for i8 {
    const BEATSIZE: BeatSize = BeatSize::Byte;
}

impl Sealed for u16 {}
impl Beat for u16 {
    const BEATSIZE: BeatSize = BeatSize::HalfWord;
}

impl Sealed for i16 {}
impl Beat for i16 {
    const BEATSIZE: BeatSize = BeatSize::HalfWord;
}

impl Sealed for u32 {}
impl Beat for u32 {
    const BEATSIZE: BeatSize = BeatSize::Word;
}

impl Sealed for i32 {}
impl Beat for i32 {
    const BEATSIZE: BeatSize = BeatSize::Word;
}

impl Sealed for f32 {}
impl Beat for f32 {
    const BEATSIZE: BeatSize = BeatSize::Word;
}
