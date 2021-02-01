//! # DMA transfer abstractions
//!
//! # Transfer types
//!
//! Four transfer types are supported:
//!
//! * Incrementing-source to incrementing-destination
//! (normally used for memory-to-memory transfers) - see
//! [`inc_src_inc_dest`](DmaTransfer::inc_src_inc_dest)
//!
//! * Incrementing-source to fixed-destination (normally used
//! for memory-to-peripheral transfers) - see
//! [`inc_src_fixed_dest`](DmaTransfer::inc_src_fixed_dest)
//!
//! * Fixed-source to incrementing-destination (normally used for
//! peripheral-to-memory transfers) - see
//! [`fixed_src_inc_dest`](DmaTransfer::fixed_src_inc_dest)
//!
//! * Fixed-source to fixed-destination (normally used for
//! peripheral-to-peripheral transfers) - see
//! [`fixed_src_fixed_dest`](DmaTransfer::fixed_src_fixed_dest)
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
//! A transfer is started by calling [`begin`](DmaTransfer::begin). You will be
//! required to supply a trigger source and a trigger action.
//!
//! # Waiting for a transfer to complete
//!
//! A transfer can waited upon by calling [`wait`](DmaTransfer::wait). This is a
//! _blocking_ method, meaning it will busy-wait until the transfer is
//! completed. When it returns, it will release the source and destination
//! buffers, as well as the DMA channel and the payload.
//!
//! # Interrupting (stopping) a transfer
//!
//! A transfer can be stopped (regardless of whether it has completed or not) by
//! calling [`stop`](DmaTransfer::stop). This is _not_ a blocking method,
//! meaning it will stop the transfer and immediately return. When it returns,
//! it will release the source and destination buffers, as well as the DMA
//! channel and the payload.
//!
//! # Trigger sources
//!
//! Most peripherals can issue triggers to a DMA channel. A software trigger is
//! also available (see [`trigger`](DmaTransfer::software_trigger)). See
//! ATSAMD21 datasheet, table 19-8 for all available trigger sources.
//!
//! # Trigger actions
//!
//! Three trigger actions are available:
//!
//! * BLOCK: One trigger required for each block transfer. In the context of
//!   this driver,
//! one DmaTransfer is equivalent to one Block transfer.
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
use core::mem;

// TODO change source and dest types to Pin (see https://docs.rust-embedded.org/embedonomicon/dma.html#immovable-buffers)
/// DMA transfer, owning the resources until the transfer is done and
/// [`wait`](DmaTransfer::wait) is called.
pub struct DmaTransfer<B, Pld, Src, Dst, ChanStatus, const ID: u8>
where
    B: 'static + DmaBeat,
    Src: DmaBuffer<B>,
    Dst: DmaBuffer<B>,
    ChanStatus: Status,
{
    xfer: UnsafeTransfer<B, Pld, Src, Dst, ChanStatus, ID>,
}

/// Unsafe DMA transfer, owning the resources until the transfer is done and
/// [`wait`](DmaTransfer::wait) is called. The user *must* ensure the
/// transfer won't be dropped until it `wait`ed upon or `stop`ped
/// before the transfer is dropped.
pub struct UnsafeTransfer<B, Pld, Src, Dst, ChanStatus, const ID: u8>
where
    B: DmaBeat,
    Src: DmaBuffer<B>,
    Dst: DmaBuffer<B>,
    ChanStatus: Status,
{
    config: TransferConfiguration<B, Src, Dst>,
    chan: Channel<ChanStatus, ID>,
    payload: Pld,
}

/// Transfer configuration abstraction. This is also used
/// for `UnsafeTransfer`s so no `'static` bound.
struct TransferConfiguration<B, Src, Dst>
where
    B: DmaBeat,
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
    ChanStatus: Status,
{
}

// Seal the UnsafeTransfer struct. Therefore users of the crate won't be able to
// add their own `impl`s.
impl<B, Pld, Src, Dst, ChanStatus, const ID: u8> Sealed
    for UnsafeTransfer<B, Pld, Src, Dst, ChanStatus, ID>
where
    B: DmaBeat,
    Src: DmaBuffer<B>,
    Dst: DmaBuffer<B>,
    ChanStatus: Status,
{
}

/// These methods offer a transfer from an incrementing source to a fixed
/// destination
impl<B, Pld, const ID: u8> DmaTransfer<B, Pld, &'static mut [B], &'static mut B, Ready, ID>
where
    B: 'static + DmaBeat,
{
    /// Setup a transfer with an incrementing source to fixed destination.
    ///
    /// If `circular_xfer == false`, the transfer will run once
    /// and stop. If `circular_xfer == true`, then the transfer will be set as
    /// circular, meaning it will restart a new, identical block transfer
    /// when the current block transfer is complete.
    ///
    /// `payload` is just a way for the transfer to own resources
    /// while the transfer is ongoing. For instance, a SERCOM instance could be
    /// moved into the transfer to prevent data races until the transfer is
    /// complete. Calling [`wait`](DmaTransfer::wait) will release the
    /// payload for reuse.
    #[inline]
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
        DmaTransfer {
            xfer: UnsafeTransfer::setup(chan, config, circular_xfer, payload),
        }
    }
}

/// These methods offer a transfer from an incrementing source to a fixed
/// destination
impl<'a, B, Pld, const ID: u8> UnsafeTransfer<B, Pld, &'a mut [B], *mut B, Ready, ID>
where
    B: DmaBeat,
{
    /// Setup a transfer with an incrementing source to fixed destination.
    ///
    /// If `circular_xfer == false`, the transfer will run once
    /// and stop. If `circular_xfer == true`, then the transfer will be set as
    /// circular, meaning it will restart a new, identical block transfer
    /// when the current block transfer is complete.
    ///
    /// `payload` is just a way for the transfer to own resources
    /// while the transfer is ongoing. For instance, a SERCOM instance could be
    /// moved into the transfer to prevent data races until the transfer is
    /// complete. Calling [`wait`](DmaTransfer::wait) will release the
    /// payload for reuse.
    ///
    /// # Safety
    ///
    /// This function is `unsafe`. You must either manually guarantee that both
    /// `source` and `dest` are `'static`, or that the returned
    /// `UnsafeTransfer` WILL be stopped (by calling `stop`) or waited upon
    /// (by calling `wait`) before being dropped in every possible case.
    #[inline]
    pub unsafe fn inc_src_fixed_dest_unchecked(
        chan: Channel<Ready, ID>,
        source: &'a mut [B],
        dest: *mut B,
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

/// These methods offer a transfer from an fixed source to a incrementing
/// destination
impl<B, Pld, const ID: u8> DmaTransfer<B, Pld, &'static mut B, &'static mut [B], Ready, ID>
where
    B: DmaBeat,
{
    /// Setup a transfer from a fixed source to an incrementing destination.
    ///
    /// If `circular_xfer == false`, the transfer will run once
    /// and stop. If `circular_xfer == true`, then the transfer will be set as
    /// circular, meaning it will restart a new, identical block transfer
    /// when the current block transfer is complete.
    ///
    /// `payload` is just a way for the transfer to own resources
    /// while the transfer is ongoing. For instance, a SERCOM instance could be
    /// moved into the transfer to prevent data races until the transfer is
    /// complete. Calling [`wait`](DmaTransfer::wait) will release the
    /// payload for reuse.
    #[inline]
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
        DmaTransfer {
            xfer: UnsafeTransfer::setup(chan, config, circular_xfer, payload),
        }
    }
}

impl<'a, B, Pld, const ID: u8> UnsafeTransfer<B, Pld, *mut B, &'a mut [B], Ready, ID>
where
    B: DmaBeat,
{
    /// Setup a transfer with an incrementing source to fixed destination.
    ///
    /// If `circular_xfer == false`, the transfer will run once
    /// and stop. If `circular_xfer == true`, then the transfer will be set as
    /// circular, meaning it will restart a new, identical block transfer
    /// when the current block transfer is complete.
    ///
    /// `payload` is just a way for the transfer to own resources
    /// while the transfer is ongoing. For instance, a SERCOM instance could be
    /// moved into the transfer to prevent data races until the transfer is
    /// complete. Calling [`wait`](DmaTransfer::wait) will release the
    /// payload for reuse.
    ///
    /// # Safety
    ///
    /// This function is `unsafe`. You must either manually guarantee that both
    /// `source` and `dest` are `'static`, or that the returned
    /// `UnsafeTransfer` WILL be stopped (by calling `stop`) or waited upon
    /// (by calling `wait`) before being dropped in every possible case.
    #[inline]
    pub unsafe fn fixed_src_inc_dest_unchecked(
        chan: Channel<Ready, ID>,
        source: *mut B,
        dest: &'a mut [B],
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

/// These methods offer a transfer from an incrementing source to a incrementing
/// destination
impl<B: 'static + DmaBeat, P, const ID: u8, const N: usize>
    DmaTransfer<B, P, &'static mut [B; N], &'static mut [B; N], Ready, ID>
{
    /// Setup a transfer from an incrementing source to an incrementing
    /// destination.
    ///
    /// If `circular_xfer == false`, the transfer will run once
    /// and stop. If `circular_xfer == true`, then the transfer will be set as
    /// circular, meaning it will restart a new, identical block transfer
    /// when the current block transfer is complete.
    ///
    /// `payload` is just a way for the transfer to own resources
    /// while the transfer is ongoing. For instance, a SERCOM instance could be
    /// moved into the transfer to prevent data races until the transfer is
    /// complete. Calling [`wait`](DmaTransfer::wait) will release the
    /// payload for reuse.
    ///
    /// Here, we use array references instead of slices to *statically* ensure
    /// they are the same length. If the two buffers weren't the same length,
    /// we could run into undefined behaviour, especially if the destination
    /// buffer is shorter than the source buffer. In that specific case, the
    /// DMAC would end up overrunning the destination buffer and would write
    /// into a memory section we don't own.
    #[inline]
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
        DmaTransfer {
            xfer: UnsafeTransfer::setup(chan, config, circular_xfer, payload),
        }
    }
}

/// These methods offer a transfer from an incrementing source to a incrementing
/// destination
impl<'a, B, P, const ID: u8, const N: usize>
    UnsafeTransfer<B, P, &'a mut [B; N], &'a mut [B; N], Ready, ID>
where
    B: DmaBeat,
{
    /// Setup a transfer with an incrementing source to fixed destination.
    ///
    /// If `circular_xfer == false`, the transfer will run once
    /// and stop. If `circular_xfer == true`, then the transfer will be set as
    /// circular, meaning it will restart a new, identical block transfer
    /// when the current block transfer is complete.
    ///
    /// `payload` is just a way for the transfer to own resources
    /// while the transfer is ongoing. For instance, a SERCOM instance could be
    /// moved into the transfer to prevent data races until the transfer is
    /// complete. Calling [`wait`](DmaTransfer::wait) will release the
    /// payload for reuse.
    ///
    /// Here, we use array references instead of slices to *statically* ensure
    /// they are the same length. If the two buffers weren't the same length,
    /// we could run into undefined behaviour, especially if the destination
    /// buffer is shorter than the source buffer. In that specific case, the
    /// DMAC would end up overrunning the destination buffer and would write
    /// into a memory section we don't own.
    ///
    /// # Safety
    ///
    /// This function is `unsafe`. You must either manually guarantee that both
    /// `source` and `dest` are `'static`, or that the returned
    /// `UnsafeTransfer` WILL be stopped (by calling `stop`) or waited upon
    /// (by calling `wait`) before being dropped in every possible case.
    #[inline]
    pub unsafe fn inc_src_inc_dest_unchecked(
        chan: Channel<Ready, ID>,
        source: &'a mut [B; N],
        dest: &'a mut [B; N],
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

/// These methods offer a transfer from an fixed source to a fixed destination
impl<B: 'static + DmaBeat, P, const ID: u8>
    DmaTransfer<B, P, &'static mut B, &'static mut B, Ready, ID>
{
    /// Fixed source to fixed destination.
    ///
    /// If `circular_xfer == false`, the transfer will run once
    /// and stop. If `circular_xfer == true`, then the transfer will be set as
    /// circular, meaning it will restart a new, identical block transfer
    /// when the current block transfer is complete.
    ///
    /// `payload` is just a way for the transfer to own resources
    /// while the transfer is ongoing. For instance, a SERCOM instance could be
    /// moved into the transfer to prevent data races until the transfer is
    /// complete. Calling [`wait`](DmaTransfer::wait) will release the
    /// payload for reuse.
    #[inline]
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
        DmaTransfer {
            xfer: UnsafeTransfer::setup(chan, config, circular_xfer, payload),
        }
    }
}

/// These methods offer a transfer from an fixed source to a fixed destination
impl<'a, B, P, const ID: u8> UnsafeTransfer<B, P, *mut B, *mut B, Ready, ID>
where
    B: DmaBeat,
{
    /// Setup a transfer with an incrementing source to fixed destination.
    ///
    /// If `circular_xfer == false`, the transfer will run once
    /// and stop. If `circular_xfer == true`, then the transfer will be set as
    /// circular, meaning it will restart a new, identical block transfer
    /// when the current block transfer is complete.
    ///
    /// `payload` is just a way for the transfer to own resources
    /// while the transfer is ongoing. For instance, a SERCOM instance could be
    /// moved into the transfer to prevent data races until the transfer is
    /// complete. Calling [`wait`](DmaTransfer::wait) will release the
    /// payload for reuse.
    ///
    /// # Safety
    ///
    /// This function is `unsafe`. You must either manually guarantee that both
    /// `source` and `dest` are `'static`, or that the returned
    /// `UnsafeTransfer` WILL be stopped (by calling `stop`) or waited upon
    /// (by calling `wait`) before being dropped in every possible case.
    #[inline]
    pub unsafe fn fixed_src_fixed_dest_unchecked(
        chan: Channel<Ready, ID>,
        source: *mut B,
        dest: *mut B,
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

/// These methods are available to an `UnsafeTransfer` holding a `Ready` channel
impl<B, P, Src, Dst, const ID: u8> UnsafeTransfer<B, P, Src, Dst, Ready, ID>
where
    B: DmaBeat,
    Src: DmaBuffer<B>,
    Dst: DmaBuffer<B>,
{
    /// Setup a DMA transfer.
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
                | ((B::BEATSIZE as u16) << 8)           // Beatsize
                | (1 << 0), // Validate descriptor
        };

        // SAFETY this is safe as long as we ONLY write to the descriptor
        // belonging to OUR channel. We assume this is the only place
        // in the entire library that this section or the array
        // will be written to.
        unsafe {
            DESCRIPTOR_SECTION[ID as usize] = xfer_descriptor;
        }

        UnsafeTransfer {
            chan,
            config,
            payload,
        }
    }

    /// Begin DMA transfer. If [TriggerSource::DISABLE](TriggerSource::DISABLE)
    /// is used, a sowftware trigger will be issued to the DMA channel to
    /// launch the transfer. Is is therefore not necessary, in most cases,
    /// to manually issue a software trigger to the channel.
    pub unsafe fn begin(
        self,
        dmac: &mut DmaController,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
    ) -> UnsafeTransfer<B, P, Src, Dst, Busy, ID> {
        // Memory barrier to prevent the compiler/CPU from re-ordering read/write
        // operations beyond this fence.
        // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
        atomic::fence(atomic::Ordering::Release); //  ▲

        // SAFETY: This is safe because we only borrow dmac once
        let dmac = dmac.dmac_mut();
        let chan = self.chan.start(dmac, trig_src, trig_act);

        UnsafeTransfer {
            config: self.config,
            chan,
            payload: self.payload,
        }
    }
}

/// These methods are available to a `DmaTransfer` holding a `Ready` channel
impl<B, P, Src, Dst, const ID: u8> DmaTransfer<B, P, Src, Dst, Ready, ID>
where
    B: 'static + DmaBeat,
    Src: DmaBuffer<B>,
    Dst: DmaBuffer<B>,
{
    /// Begin DMA transfer. If [TriggerSource::DISABLE](TriggerSource::DISABLE)
    /// is used, a sowftware trigger will be issued to the DMA channel to
    /// launch the transfer. Is is therefore not necessary, in most cases,
    /// to manually issue a software trigger to the channel.
    #[inline]
    pub fn begin(
        self,
        dmac: &mut DmaController,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
    ) -> DmaTransfer<B, P, Src, Dst, Busy, ID> {
        let xfer = unsafe { self.xfer.begin(dmac, trig_src, trig_act) };
        DmaTransfer { xfer }
    }
}

/// These methods are available to an `UnsafeTransfer` holding a `Busy` channel
impl<B, P, Src, Dst, const ID: u8> UnsafeTransfer<B, P, Src, Dst, Busy, ID>
where
    B: DmaBeat,
    Src: DmaBuffer<B>,
    Dst: DmaBuffer<B>,
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
    pub fn wait(self, dmac: &mut DmaController) -> (Src, Dst, Channel<Ready, ID>, P) {
        // SAFETY: This is safe because we only borrow dmac once.
        let dmac = unsafe { dmac.dmac_mut() };
        let chan = self.chan.free(dmac);

        // Memory barrier to prevent the compiler/CPU from re-ordering read/write
        // operations beyond this fence.
        // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
        atomic::fence(atomic::Ordering::Acquire); // ▼

        (self.config.source, self.config.dest, chan, self.payload)
    }

    /// Non-blocking; Immediately stop the DMA transfer and release all owned
    /// resources
    pub fn stop(self, dmac: &mut DmaController) -> (Src, Dst, Channel<Ready, ID>, P) {
        // SAFETY: This is safe because we only borrow dmac once.
        let dmac = unsafe { dmac.dmac_mut() };
        let chan = self.chan.stop(dmac);

        // Memory barrier to prevent the compiler/CPU from re-ordering read/write
        // operations beyond this fence.
        // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
        atomic::fence(atomic::Ordering::Acquire); // ▼

        (self.config.source, self.config.dest, chan, self.payload)
    }
}

/// These methods are available to a `DmaTransfer` holding a `Busy` channel
impl<B, P, Src, Dst, const ID: u8> DmaTransfer<B, P, Src, Dst, Busy, ID>
where
    B: 'static + DmaBeat,
    Src: DmaBuffer<B>,
    Dst: DmaBuffer<B>,
{
    /// Issue a software trigger request to the corresponding channel.
    /// Note that is not guaranteed that the trigger request will register,
    /// if a trigger request is already pending for the channel.
    #[inline]
    pub fn software_trigger(&mut self, dmac: &mut DmaController) {
        self.xfer.software_trigger(dmac);
    }
    /// Blocking; Wait for the DMA transfer to complete and release all owned
    /// resources
    #[inline]
    pub fn wait(self, dmac: &mut DmaController) -> (Src, Dst, Channel<Ready, ID>, P) {
        self.xfer.wait(dmac)
    }

    /// Non-blocking; Immediately stop the DMA transfer and release all owned
    /// resources
    #[inline]
    pub fn stop(self, dmac: &mut DmaController) -> (Src, Dst, Channel<Ready, ID>, P) {
        self.xfer.stop(dmac)
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
/// references to arrays of `T` or slices of `T`, where `T: DmaBeat`,
/// as well as required transfer length
pub trait DmaBuffer<T: DmaBeat>: Sealed {
    /// Take a `*mut` reference to the last object
    /// in the buffer needed by the DMAC. The pointer
    /// changes depending on whether the address should
    fn to_dma_ptr(&mut self) -> *mut T;
    /// Check if the DMA address should be incrementing or not
    fn incrementing(&self) -> bool;
}

impl<T: DmaBeat, const N: usize> Sealed for &mut [T; N] {}

impl<T: DmaBeat, const N: usize> DmaBuffer<T> for &mut [T; N] {
    #[inline]
    fn to_dma_ptr(&mut self) -> *mut T {
        let ptr = self.as_mut_ptr();
        if self.incrementing() {
            unsafe { ptr.add(N) }
        } else {
            ptr
        }
    }

    #[inline]
    fn incrementing(&self) -> bool {
        N > 1
    }
}

impl<T: DmaBeat> Sealed for &mut [T] {}

impl<T: DmaBeat> DmaBuffer<T> for &mut [T] {
    #[inline]
    fn to_dma_ptr(&mut self) -> *mut T {
        let ptr = self.as_mut_ptr();
        if self.incrementing() {
            unsafe { ptr.add(self.len()) }
        } else {
            ptr
        }
    }

    #[inline]
    fn incrementing(&self) -> bool {
        self.len() > 1
    }
}

impl<T: DmaBeat> Sealed for &mut T {}

impl<T: DmaBeat> DmaBuffer<T> for &mut T {
    #[inline]
    fn to_dma_ptr(&mut self) -> *mut T {
        *self as *mut T
    }

    #[inline]
    fn incrementing(&self) -> bool {
        false
    }
}

impl<T: DmaBeat> Sealed for *mut T {}

impl<T: DmaBeat> DmaBuffer<T> for *mut T {
    #[inline]
    fn to_dma_ptr(&mut self) -> *mut T {
        *self
    }

    fn incrementing(&self) -> bool {
        false
    }
}

/// Convert 8, 16 and 32 bit types
/// into [`BeatSize`](BeatSize)
pub trait DmaBeat: Sealed {
    /// Convert to BeatSize enum
    const BEATSIZE: BeatSize;
}

impl Sealed for u8 {}
impl DmaBeat for u8 {
    const BEATSIZE: BeatSize = BeatSize::Byte;
}

impl Sealed for i8 {}
impl DmaBeat for i8 {
    const BEATSIZE: BeatSize = BeatSize::Byte;
}

impl Sealed for u16 {}
impl DmaBeat for u16 {
    const BEATSIZE: BeatSize = BeatSize::HalfWord;
}

impl Sealed for i16 {}
impl DmaBeat for i16 {
    const BEATSIZE: BeatSize = BeatSize::HalfWord;
}

impl Sealed for u32 {}
impl DmaBeat for u32 {
    const BEATSIZE: BeatSize = BeatSize::Word;
}

impl Sealed for i32 {}
impl DmaBeat for i32 {
    const BEATSIZE: BeatSize = BeatSize::Word;
}

impl Sealed for f32 {}
impl DmaBeat for f32 {
    const BEATSIZE: BeatSize = BeatSize::Word;
}
