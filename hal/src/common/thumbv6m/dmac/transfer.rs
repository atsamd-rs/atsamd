//! # DMA transfer abstractions
//!
//! ## Transfer types
//!
//! Four transfer types are supported:
//!
//! * Incrementing-source to incrementing-destination
//! (normally used for memory-to-memory transfers) - see `inc_src_inc_dest`
//!
//! * Incrementing-source to fixed-destination (normally used
//! for memory-to-peripheral transfers) - see `inc_src_fixed_dest`
//!
//! * Fixed-source to incrementing-destination (normally used for
//! peripheral-to-memory transfers) - see `fixed_src_inc_dest`
//!
//! * Fixed-source to fixed-destination (normally used for
//! peripheral-to-peripheral transfers) - see `fixed_src_fixed_dest`
//!
//! ## Beat sizes
//!
//! Three beat sizes are supported:
//!
//! * Byte-per-byte (8 bit beats);
//!
//! * Halfword-per-halfword (16 bit beats);
//!
//! * Word-per-word (32 bit beats);

use core::{
    marker::PhantomData,
    ops::DerefMut,
    sync::atomic::{compiler_fence, Ordering},
};

use super::{
    channel::{Busy, Channel, Ready},
    dma_controller::{DmaController, TriggerAction, TriggerSource},
    DmacDescriptor, DESCRIPTOR_SECTION,
};
use crate::typelevel::Sealed;
use core::mem;

// TODO change source and dest types to Pin (see https://docs.rust-embedded.org/embedonomicon/dma.html#immovable-buffers)
/// DMA transfer, owning the resources until the transfer is done and [wait()](DmaTransfer::wait) is called.
pub struct DmaTransfer<B, Pld, Src, Dst, ChanStatus, const ID: u8>
where
    B: 'static + DmaBeat,
    Src: DmaBuffer<B>,
    Dst: DmaBuffer<B>,
{
    config: TransferConfiguration<B, Src, Dst>,
    chan: Channel<ChanStatus, ID>,
    payload: Pld,
}

/// Transfer configuration abstraction
struct TransferConfiguration<B, Src, Dst>
where
    B: 'static + DmaBeat,
    Src: DmaBuffer<B>,
    Dst: DmaBuffer<B>,
{
    source: Src,
    dest: Dst,
    length: u16,
    _t: PhantomData<B>,
}

// Seal the DmaTransfer struct. Therefore users of the crate won't be able to
// add their own `impl`s.
impl<B, Pld, Src, Dst, ChanStatus, const ID: u8> Sealed
    for DmaTransfer<B, Pld, Src, Dst, ChanStatus, ID>
where
    B: 'static + DmaBeat,
    Src: DmaBuffer<B>,
    Dst: DmaBuffer<B>,
{
}

impl<B, Pld, const ID: u8> DmaTransfer<B, Pld, &'static mut [B], &'static mut B, Ready, ID>
where
    B: 'static + DmaBeat,
{
    /// Incrementing source to fixed destination
    #[inline(always)]
    pub fn inc_src_fixed_dest(
        chan: Channel<Ready, ID>,
        source: &'static mut [B],
        dest: &'static mut B,
        circular_xfer: bool,
        payload: Pld,
    ) -> Self {
        let length = source.len() as u16;
        let config = TransferConfiguration {
            source,
            dest,
            length,
            _t: PhantomData,
        };
        Self::setup(chan, config, circular_xfer, payload)
    }
}

impl<B, Pld, const ID: u8> DmaTransfer<B, Pld, &'static mut B, &'static mut [B], Ready, ID>
where
    B: 'static + DmaBeat,
{
    /// Fixed source to incrementing destination
    #[inline(always)]
    pub fn fixed_src_inc_dest(
        chan: Channel<Ready, ID>,
        source: &'static mut B,
        dest: &'static mut [B],
        circular_xfer: bool,
        payload: Pld,
    ) -> Self {
        let length = dest.len() as u16;
        let config = TransferConfiguration {
            source,
            dest,
            length,
            _t: PhantomData,
        };
        Self::setup(chan, config, circular_xfer, payload)
    }
}

impl<B: 'static + DmaBeat, P, const ID: u8, const N: usize>
    DmaTransfer<B, P, &'static mut [B; N], &'static mut [B; N], Ready, ID>
{
    /// Incrementing source to incrementing destination. Here,
    /// we use array references instead of slices to *statically* ensure
    /// they are the same length. If the two buffers weren't the same length,
    /// we could run into undefined behaviour, especially if the destination
    /// buffer is shorter than the source buffer. In that specific case, the
    /// DMAC would end up overrunning the destination buffer and would write
    /// into a memory section we don't own.
    #[inline(always)]
    pub fn inc_src_inc_dest(
        chan: Channel<Ready, ID>,
        source: &'static mut [B; N],
        dest: &'static mut [B; N],
        circular_xfer: bool,
        payload: P,
    ) -> Self {
        let length = source.len() as u16;
        let config = TransferConfiguration {
            source,
            dest,
            length,
            _t: PhantomData,
        };
        Self::setup(chan, config, circular_xfer, payload)
    }
}

impl<B: 'static + DmaBeat, P, const ID: u8>
    DmaTransfer<B, P, &'static mut B, &'static mut B, Ready, ID>
{
    /// Fixed source to fixed destination
    #[inline(always)]
    pub fn fixed_src_fixed_dest(
        chan: Channel<Ready, ID>,
        source: &'static mut B,
        dest: &'static mut B,
        circular_xfer: bool,
        payload: P,
    ) -> Self {
        let length = 1;
        let config = TransferConfiguration {
            source,
            dest,
            length,
            _t: PhantomData,
        };
        Self::setup(chan, config, circular_xfer, payload)
    }
}

/// These methods are available to a `DmaTransfer` holding a `Ready` channel
impl<B, P, Src, Dst, const ID: u8> DmaTransfer<B, P, Src, Dst, Ready, ID>
where
    B: 'static + DmaBeat,
    Src: DmaBuffer<B>,
    Dst: DmaBuffer<B>,
{
    /// Setup a DMA transfer. If `circular_xfer == false`, the transfer will run once
    /// and stop. If `circular_xfer == true`, then the transfer will be set as circular,
    /// meaning it will restart a new, identical block transfer when the current block
    /// transfer is complete. `payload` is just a way for the transfer to own resources
    /// while the transfer is ongoing. For instance, a SERCOM instance could be moved
    /// into the transfer to prevent data races until the transfer is complete. Calling
    /// (`wait()`)[DmaTransfer::wait] will release the payload for reuse.
    fn setup(
        chan: Channel<Ready, ID>,
        mut config: TransferConfiguration<B, Src, Dst>,
        circular_xfer: bool,
        payload: P,
    ) -> Self {
        // Enable support for circular transfers. If circular_xfer is true,
        // we set the address of the "next" block descriptor to actually
        // be the same address as the current block descriptor.
        // Otherwise we set it to 0 (terminates the transaction)
        // TODO: Enable support for linked lists (?)
        let descaddr = if circular_xfer {
            // SAFETY: This is safe as we are only reading the descriptor's address,
            // and not actually writing any data to it. We also assume the descriptor
            // will never be moved.
            unsafe {
                DESCRIPTOR_SECTION.as_ptr() as u32
                    + mem::size_of::<DmacDescriptor>() as u32 * (ID as u32)
            }
        } else {
            0
        };

        let src = &mut config.source;
        let dst = &mut config.dest;

        let xfer_descriptor = DmacDescriptor {
            // Next descriptor address:  0x0 terminates the transaction (no linked list),
            // any other address points to the next block descriptor
            descaddr,
            // Source address: address of the last beat transfer source in block
            srcaddr: src.to_dma_ptr() as u32,
            // Destination address: address of the last beat transfer destination in block
            dstaddr: dst.to_dma_ptr() as u32,
            // Block transfer count: number of beats in block transfer
            btcnt: config.length,
            // Block transfer control: Datasheet  section 19.8.2.1 p.329
            btctrl: ((src.incrementing() as u16) << 10) // Increment source address?
                | ((dst.incrementing() as u16) << 11)   // Tncrement dest address?
                | ((B::to_beatsize() as u16) << 8)      // Beatsize
                | (1 << 0), // Validate descriptor
        };

        // SAFETY this is safe as long as we ONLY write to the descriptor
        // belonging to OUR channel. We assume this is the only place
        // in the entire library that this section or the array
        // will be written to.
        unsafe {
            DESCRIPTOR_SECTION[ID as usize] = xfer_descriptor;
        }

        DmaTransfer {
            chan,
            config,
            payload,
        }
    }

    /// Begin DMA transfer. If [TriggerSource::DISABLE](TriggerSource::DISABLE) is used,
    /// a sowftware trigger will be issued to the DMA channel to launch the transfer.
    pub fn begin(
        self,
        dmac: &mut DmaController,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
    ) -> DmaTransfer<B, P, Src, Dst, Busy, ID> {
        // Compiler fence to prevent the compiler from re-ordering read/write operations
        // beyond this fence.
        // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
        compiler_fence(Ordering::Release); //  ▲

        let chan = self.chan.start(trig_src, trig_act, dmac.as_mut());

        DmaTransfer {
            config: self.config,
            chan,
            payload: self.payload,
        }
    }
}

/// These methods are available to a `DmaTransfer` holding a `Busy` channel
impl<B, P, Src, Dst, const ID: u8> DmaTransfer<B, P, Src, Dst, Busy, ID>
where
    B: 'static + DmaBeat,
    Src: DmaBuffer<B>,
    Dst: DmaBuffer<B>,
{
    /// Blocking; Wait for the DMA transfer to complete and release all owned resources
    pub fn wait(self, dmac: &mut DmaController) -> (Src, Dst, Channel<Ready, ID>, P) {
        let chan = self.chan.release(dmac.as_mut());

        // Compiler fence to prevent the compiler from re-ordering read/write operations
        // in front this fence.
        // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
        compiler_fence(Ordering::Acquire); // ▼

        (self.config.source, self.config.dest, chan, self.payload)
    }

    /// Non-blocking; Immediately stop the DMA transfer and release all owned resources
    pub fn stop(self, dmac: &mut DmaController) -> (Src, Dst, Channel<Ready, ID>, P) {
        let chan = self.chan.stop(dmac.as_mut());

        // Compiler fence to prevent the compiler from re-ordering read/write operations
        // in front this fence.
        // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
        compiler_fence(Ordering::Acquire); // ▼

        (self.config.source, self.config.dest, chan, self.payload)
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

// Trait enabling taking `*const` references to references to a single `T`,
/// references to arrays of `T` or slices of `T`, where `T: DmaBeat`,
/// as well as required transfer length
pub trait DmaBuffer<T: DmaBeat>: DerefMut {
    /// Take a `*const` reference to the last object
    /// in the buffer needed by the DMAC. The pointer
    /// changes depending on whether the address should
    fn to_dma_ptr(&self) -> *const T;
    /// Check if the DMA address should be incrementing or not
    fn incrementing(&self) -> bool;
}

impl<T: DmaBeat, const N: usize> DmaBuffer<T> for &mut [T; N] {
    #[inline(always)]
    fn to_dma_ptr(&self) -> *const T {
        let ptr = self.as_ptr();
        if self.incrementing() {
            unsafe { ptr.offset(N as isize) }
        } else {
            ptr
        }
    }

    #[inline(always)]
    fn incrementing(&self) -> bool {
        N > 1
    }
}

impl<T: DmaBeat> DmaBuffer<T> for &mut [T] {
    #[inline(always)]
    fn to_dma_ptr(&self) -> *const T {
        let ptr = self.as_ptr();
        if self.incrementing() {
            unsafe { ptr.offset(self.len() as isize) }
        } else {
            ptr
        }
    }

    #[inline(always)]
    fn incrementing(&self) -> bool {
        self.len() > 1
    }
}

impl<T: DmaBeat> DmaBuffer<T> for &mut T {
    #[inline(always)]
    fn to_dma_ptr(&self) -> *const T {
        *self as *const T
    }

    #[inline(always)]
    fn incrementing(&self) -> bool {
        false
    }
}

/// Convert u8, u16 and u32 integers
/// into BeatSize enums
pub trait DmaBeat {
    /// Convert to BeatSize
    fn to_beatsize() -> BeatSize;
    /// Number of bytes in type
    fn num_bytes() -> u8;
}

impl DmaBeat for u8 {
    #[inline(always)]
    fn to_beatsize() -> BeatSize {
        BeatSize::Byte
    }

    #[inline(always)]
    fn num_bytes() -> u8 {
        1
    }
}

impl DmaBeat for i8 {
    #[inline(always)]
    fn to_beatsize() -> BeatSize {
        BeatSize::Byte
    }

    #[inline(always)]
    fn num_bytes() -> u8 {
        1
    }
}

impl DmaBeat for u16 {
    #[inline(always)]
    fn to_beatsize() -> BeatSize {
        BeatSize::HalfWord
    }

    #[inline(always)]
    fn num_bytes() -> u8 {
        2
    }
}

impl DmaBeat for i16 {
    #[inline(always)]
    fn to_beatsize() -> BeatSize {
        BeatSize::HalfWord
    }

    #[inline(always)]
    fn num_bytes() -> u8 {
        2
    }
}

impl DmaBeat for u32 {
    #[inline(always)]
    fn to_beatsize() -> BeatSize {
        BeatSize::Word
    }

    #[inline(always)]
    fn num_bytes() -> u8 {
        4
    }
}

impl DmaBeat for i32 {
    #[inline(always)]
    fn to_beatsize() -> BeatSize {
        BeatSize::Word
    }

    #[inline(always)]
    fn num_bytes() -> u8 {
        4
    }
}

impl DmaBeat for f32 {
    fn to_beatsize() -> BeatSize {
        BeatSize::Word
    }

    #[inline(always)]
    fn num_bytes() -> u8 {
        4
    }
}
