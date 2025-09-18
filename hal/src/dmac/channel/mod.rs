//! # Abstractions over individual DMA channels
//!
//! # Initializing
//!
//! Individual channels should be initialized through the
//! [`Channel::init`] method. This will return a `Channel<Id, Ready>` ready for
//! use by a [`Transfer`]. Initializing a channel requires setting a priority
//! level, as well as enabling or disabling interrupt requests (only for the
//! specific channel being initialized).
//!
//! # Burst Length and FIFO Threshold (SAMD51/SAME5x only)
//!
//! The transfer burst length can be configured through the
//! [`Channel::burst_length`] method. A burst is an atomic,
//! uninterruptible transfer which length corresponds to a number of beats. See
//! SAMD5x/E5x datasheet section 22.6.1.1 for more information. The FIFO
//! threshold can be configured through the
//! [`Channel::fifo_threshold`] method. This enables the channel
//! to wait for multiple Beats before sending a Burst. See SAMD5x/E5x datasheet
//! section 22.6.2.8 for more information.
//!
//! # Channel status
//!
//! Channels can be in any of three statuses: [`Uninitialized`], [`Ready`], and
//! [`Busy`]. These statuses are checked at compile time to ensure they are
//! properly initialized before launching DMA transfers.
//!
//! # Resetting
//!
//! Calling the [`Channel::reset`] method will reset the channel to its
//! `Uninitialized` state. You will be required to call [`Channel::init`]
//! again before being able to use it with a `Transfer`.

#![allow(unused_braces)]

use core::marker::PhantomData;
use core::sync::atomic;

use atsamd_hal_macros::{hal_cfg, hal_macro_helper};

use super::{
    Beat, Buffer, Error,
    dma_controller::{ChId, PriorityLevel, TriggerAction, TriggerSource},
    sram::{self, DmacDescriptor},
    transfer::{BufferPair, Transfer},
};
use crate::typelevel::{Is, Sealed};
use modular_bitfield::prelude::*;

mod reg;
use reg::RegisterBlock;

#[hal_cfg("dmac-d5x")]
use super::dma_controller::{BurstLength, FifoThreshold};

//==============================================================================
// Channel Status
//==============================================================================
pub trait Status: Sealed {
    type Uninitialized: Status;
    type Ready: Status;
}

/// Uninitialized channel
pub enum Uninitialized {}
impl Sealed for Uninitialized {}
impl Status for Uninitialized {
    type Uninitialized = Uninitialized;
    type Ready = Ready;
}

/// Initialized and ready to transfer channel
pub enum Ready {}
impl Sealed for Ready {}
impl Status for Ready {
    type Uninitialized = Uninitialized;
    type Ready = Ready;
}

/// Busy channel
pub enum Busy {}
impl Sealed for Busy {}
impl Status for Busy {
    type Uninitialized = Uninitialized;
    type Ready = Ready;
}

/// Uninitialized [`Channel`] configured for `async` operation
#[cfg(feature = "async")]
pub enum UninitializedFuture {}
#[cfg(feature = "async")]
impl Sealed for UninitializedFuture {}
#[cfg(feature = "async")]
impl Status for UninitializedFuture {
    type Uninitialized = UninitializedFuture;
    type Ready = ReadyFuture;
}

/// Initialized and ready to transfer in `async` operation
#[cfg(feature = "async")]
pub enum ReadyFuture {}
#[cfg(feature = "async")]
impl Sealed for ReadyFuture {}
#[cfg(feature = "async")]
impl Status for ReadyFuture {
    type Uninitialized = UninitializedFuture;
    type Ready = ReadyFuture;
}

pub trait ReadyChannel: Status {}
impl ReadyChannel for Ready {}
#[cfg(feature = "async")]
impl ReadyChannel for ReadyFuture {}

//==============================================================================
// AnyChannel
//==============================================================================
pub trait AnyChannel: Sealed + Is<Type = SpecificChannel<Self>> {
    type Status: Status;
    type Id: ChId;
}

pub type SpecificChannel<C> = Channel<<C as AnyChannel>::Id, <C as AnyChannel>::Status>;

pub type ChannelStatus<C> = <C as AnyChannel>::Status;
pub type ChannelId<C> = <C as AnyChannel>::Id;

impl<Id, S> Sealed for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
}

impl<Id, S> AnyChannel for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
    type Id = Id;
    type Status = S;
}

impl<Id, S> AsRef<Self> for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<Id, S> AsMut<Self> for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

//==============================================================================
// Channel
//==============================================================================

/// DMA channel, capable of executing
/// [`Transfer`]s. Ongoing DMA transfers are automatically stopped when a
/// [`Channel`] is dropped.
pub struct Channel<Id: ChId, S: Status> {
    regs: RegisterBlock<Id>,
    _status: PhantomData<S>,
}

#[inline]
pub(super) fn new_chan<Id: ChId>(_id: PhantomData<Id>) -> Channel<Id, Uninitialized> {
    Channel {
        regs: RegisterBlock::new(_id),
        _status: PhantomData,
    }
}

#[cfg(feature = "async")]
#[inline]
pub(super) fn new_chan_future<Id: ChId>(_id: PhantomData<Id>) -> Channel<Id, UninitializedFuture> {
    Channel {
        regs: RegisterBlock::new(_id),
        _status: PhantomData,
    }
}

/// These methods may be used on any DMA channel in any configuration
impl<Id: ChId, S: Status> Channel<Id, S> {
    /// Configure the DMA channel so that it is ready to be used by a
    /// [`Transfer`](super::transfer::Transfer).
    ///
    /// # Return
    ///
    /// A `Channel` with a `Ready` status
    #[inline]
    #[hal_macro_helper]
    pub fn init(mut self, lvl: PriorityLevel) -> Channel<Id, S::Ready> {
        // Software reset the channel for good measure
        self._reset_private();

        #[hal_cfg(any("dmac-d11", "dmac-d21"))]
        // Setup priority level
        self.regs.chctrlb.modify(|_, w| w.lvl().variant(lvl));

        #[hal_cfg("dmac-d5x")]
        self.regs.chprilvl.modify(|_, w| w.prilvl().variant(lvl));

        self.change_status()
    }

    /// Selectively enable interrupts
    #[inline]
    pub fn enable_interrupts(&mut self, flags: InterruptFlags) {
        // SAFETY: This is safe as InterruptFlags is only capable of writing in
        // non-reserved bits
        self.regs
            .chintenset
            .write(|w| unsafe { w.bits(flags.into()) });
    }

    /// Selectively disable interrupts
    #[inline]
    pub fn disable_interrupts(&mut self, flags: InterruptFlags) {
        // SAFETY: This is safe as InterruptFlags is only capable of writing in
        // non-reserved bits
        self.regs
            .chintenclr
            .write(|w| unsafe { w.bits(flags.into()) });
    }

    /// Check the specified `flags`, clear then return any that were set
    #[inline]
    pub fn check_and_clear_interrupts(&mut self, flags: InterruptFlags) -> InterruptFlags {
        let mut cleared = 0;
        self.regs.chintflag.modify(|r, w| {
            cleared = r.bits() & flags.into_bytes()[0];
            unsafe { w.bits(cleared) }
        });

        InterruptFlags::from_bytes([cleared])
    }

    #[inline]
    pub(super) fn change_status<N: Status>(self) -> Channel<Id, N> {
        Channel {
            regs: self.regs,
            _status: PhantomData,
        }
    }

    #[inline]
    fn _reset_private(&mut self) {
        // Reset the channel to its startup state and wait for reset to complete
        self.regs.chctrla.modify(|_, w| w.swrst().set_bit());
        while self.regs.chctrla.read().swrst().bit_is_set() {}
    }

    #[inline]
    fn _trigger_private(&mut self) {
        self.regs.swtrigctrl.set_bit();
    }

    /// Enable the transfer, and emit a compiler fence.
    #[inline]
    fn _enable_private(&mut self) {
        // Prevent the compiler from re-ordering read/write
        // operations beyond this fence.
        // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
        atomic::fence(atomic::Ordering::Release); // ▲
        self.regs.chctrla.modify(|_, w| w.enable().set_bit());
    }

    /// Stop transfer on channel whether or not the transfer has completed
    #[inline]
    pub(crate) fn stop(&mut self) {
        self.regs.chctrla.modify(|_, w| w.enable().clear_bit());

        // Wait for the burst to finish
        while !self.xfer_complete() {
            core::hint::spin_loop();
        }

        // Prevent the compiler from re-ordering read/write
        // operations beyond this fence.
        // (see https://docs.rust-embedded.org/embedonomicon/dma.html#compiler-misoptimizations)
        atomic::fence(atomic::Ordering::Acquire); // ▼
    }

    /// Returns whether or not the transfer is complete.
    #[inline]
    pub(crate) fn xfer_complete(&mut self) -> bool {
        self.regs.chctrla.read().enable().bit_is_clear()
    }

    /// Returns the transfer's success status.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn xfer_success(&mut self) -> super::Result<()> {
        let success = self.regs.chintflag.read().terr().bit_is_clear();
        success.then_some(()).ok_or(Error::TransferError)
    }

    /// Return a mutable reference to the DMAC descriptor that belongs to this
    /// channel. In the case of linked transfers, this will be the first
    /// descriptor in the chain.
    #[inline]
    fn descriptor_mut(&mut self) -> &mut DmacDescriptor {
        // SAFETY this is only safe as long as we read/write to the descriptor
        // belonging to OUR channel. We assume this is the case, as there can only ever
        // exist one (safely created) instance of Self, and we're taking an exclusive
        // reference to Self.
        unsafe {
            let id = ChannelId::<Self>::USIZE;
            &mut *sram::get_descriptor(id)
        }
    }

    /// Fill the first descriptor of a channel into the SRAM descriptor section.
    ///
    /// # Safety
    ///
    /// This method may only be called on a channel which is not actively being
    /// used for transferring data.
    #[inline]
    pub(super) unsafe fn fill_descriptor<Src: Buffer, Dst: Buffer<Beat = Src::Beat>>(
        &mut self,
        source: &mut Src,
        destination: &mut Dst,
        circular: bool,
    ) {
        let descriptor = self.descriptor_mut();

        // Enable support for circular transfers. If circular_xfer is true,
        // we set the address of the "next" block descriptor to actually
        // be the same address as the current block descriptor.
        // Otherwise we set it to NULL, which terminates the transaction.
        let descaddr = if circular {
            // SAFETY This is safe as we are only reading the descriptor's address,
            // and not actually writing any data to it. We also assume the descriptor
            // will never be moved.
            descriptor as *mut _
        } else {
            core::ptr::null_mut()
        };

        unsafe {
            write_descriptor(descriptor, source, destination, descaddr);
        }
    }

    /// Add a linked descriptor after the first descriptor in the transfer.
    ///
    /// # Safety
    ///
    /// * This method may only be called on a channel which is not actively
    ///   being used for transferring data.
    ///
    /// * `next` must point to a valid [`DmacDescriptor`], with all the safety
    ///   considerations that entails: the source and destination buffers must
    ///   be valid, have compatible lengths, remain in scope for the entirety of
    ///   the transfer, etc.
    ///
    /// * The pointer in the `descaddr` field of `next`, along with the
    ///   descriptor it points to, etc, must point to a valid [`DmacDescriptor`]
    ///   memory location, or be null. They must not be circular (ie, points to
    ///   itself). Any linked transfer must strictly be a read transaction
    ///   (destination pointer is a byte buffer, source pointer is the SERCOM
    ///   DATA register).
    pub(super) unsafe fn link_next(&mut self, next: *mut DmacDescriptor) {
        self.descriptor_mut().descaddr = next;
    }
}

impl<Id, R> Channel<Id, R>
where
    Id: ChId,
    R: ReadyChannel,
{
    /// Issue a software reset to the channel. This will return the channel to
    /// its startup state
    #[inline]
    pub fn reset(mut self) -> Channel<Id, R::Uninitialized> {
        self._reset_private();
        self.change_status()
    }

    /// Set the FIFO threshold length. The channel will wait until it has
    /// received the selected number of Beats before triggering the Burst
    /// transfer, reducing the DMA transfer latency.
    #[hal_cfg("dmac-d5x")]
    #[inline]
    pub fn fifo_threshold(&mut self, threshold: FifoThreshold) {
        self.regs
            .chctrla
            .modify(|_, w| w.threshold().variant(threshold));
    }

    #[cfg(doc)]
    #[hal_cfg(not("dmac-d5x"))]
    /// This method is not present with the selected feature set, defined for
    /// documentation only
    pub fn fifo_threshold(&mut self) {
        unimplemented!()
    }

    /// Set burst length for the channel, in number of beats. A burst transfer
    /// is an atomic, uninterruptible operation.
    #[hal_cfg("dmac-d5x")]
    #[inline]
    pub fn burst_length(&mut self, burst_length: BurstLength) {
        self.regs
            .chctrla
            .modify(|_, w| w.burstlen().variant(burst_length));
    }

    #[cfg(doc)]
    #[hal_cfg(not("dmac-d5x"))]
    /// This method is not present with the selected feature set, defined for
    /// documentation only
    pub fn burst_length(&mut self) {
        unimplemented!()
    }

    /// Start the transfer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it starts the transfer without changing
    /// the channel status to [`Busy`]. A [`Ready`] channel which is actively
    /// transferring should NEVER be leaked.
    #[inline]
    #[hal_macro_helper]
    pub(super) unsafe fn _start_private(
        &mut self,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
    ) {
        // Configure the trigger source and trigger action
        self.configure_trigger(trig_src, trig_act);
        self._enable_private();

        // If trigger source is DISABLE, manually trigger transfer
        if trig_src == TriggerSource::Disable {
            self._trigger_private();
        }
    }

    #[inline]
    #[hal_macro_helper]
    pub(super) fn configure_trigger(&mut self, trig_src: TriggerSource, trig_act: TriggerAction) {
        // Configure the trigger source and trigger action
        #[hal_cfg(any("dmac-d11", "dmac-d21"))]
        self.regs.chctrlb.modify(|_, w| {
            w.trigsrc().variant(trig_src);
            w.trigact().variant(trig_act)
        });

        #[hal_cfg("dmac-d5x")]
        self.regs.chctrla.modify(|_, w| {
            w.trigsrc().variant(trig_src);
            w.trigact().variant(trig_act)
        });
    }
}

impl<Id: ChId> Channel<Id, Ready> {
    /// Start transfer on channel using the specified trigger source.
    ///
    /// # Return
    ///
    /// A `Channel` with a `Busy` status.
    #[inline]
    pub(crate) fn start(
        mut self,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
    ) -> Channel<Id, Busy> {
        unsafe {
            self._start_private(trig_src, trig_act);
        }
        self.change_status()
    }

    /// Begin a [`Transfer`], without changing the channel's type to [`Busy`].
    ///
    /// This method provides an additional safety guarantee over
    /// [`Self::transfer_unchecked`]; it checks that the buffer lengths are
    /// valid before attempting to start the transfer.
    ///
    /// Also provides support for linked transfers via an optional `&mut
    /// DmacDescriptor`.
    ///
    /// This function guarantees that it will never return [`Err`] if the
    /// transfer has been started.
    ///
    /// # Safety
    ///
    /// * You must ensure that the transfer is completed or stopped before
    ///   returning the [`Channel`]. Doing otherwise breaks type safety, because
    ///   a [`Ready`] channel would still be in the middle of a transfer.
    /// * If the provided `linked_descriptor` is `Some` it must not be dropped
    ///   until the transfer is completed or stopped.
    /// * Additionnally, this function doesn't take `'static` buffers. Again,
    ///   you must guarantee that the returned transfer has completed or has
    ///   been stopped before giving up control of the underlying [`Channel`].
    #[inline]
    #[allow(dead_code)]
    pub(crate) unsafe fn transfer<S, D>(
        &mut self,
        source: &mut S,
        dest: &mut D,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
        linked_descriptor: Option<&mut DmacDescriptor>,
    ) -> Result<(), Error>
    where
        S: Buffer,
        D: Buffer<Beat = S::Beat>,
    {
        Transfer::<Self, BufferPair<S, D>>::check_buffer_pair(source, dest)?;
        unsafe {
            self.transfer_unchecked(source, dest, trig_src, trig_act, linked_descriptor);
        }
        Ok(())
    }

    /// Begin a transfer, without changing the channel's type to [`Busy`].
    ///
    /// Also provides support for linked transfers via an optional `&mut
    /// DmacDescriptor`.
    ///
    /// # Safety
    ///
    /// * This method does not check that the two provided buffers have
    ///   compatible lengths. You must guarantee that:
    ///   - Either `source` or `dest` has a buffer length of 1, or
    ///   - Both buffers have the same length.
    /// * You must ensure that the transfer is completed or stopped before
    ///   returning the [`Channel`]. Doing otherwise breaks type safety, because
    ///   a [`Ready`] channel would still be in the middle of a transfer.
    /// * If the provided `linked_descriptor` is `Some` it must not be dropped
    ///   until the transfer is completed or stopped.
    /// * Additionnally, this function doesn't take `'static` buffers. Again,
    ///   you must guarantee that the returned transfer has completed or has
    ///   been stopped before giving up control of the underlying [`Channel`].
    #[inline]
    pub(crate) unsafe fn transfer_unchecked<S, D>(
        &mut self,
        source: &mut S,
        dest: &mut D,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
        linked_descriptor: Option<&mut DmacDescriptor>,
    ) where
        S: Buffer,
        D: Buffer<Beat = S::Beat>,
    {
        unsafe {
            self.fill_descriptor(source, dest, false);

            if let Some(next) = linked_descriptor {
                self.link_next(next as *mut _);
            }
        }

        self.configure_trigger(trig_src, trig_act);
        self._enable_private();

        if trig_src == TriggerSource::Disable {
            self._trigger_private();
        }
    }
}

/// These methods may only be used on a `Busy` DMA channel
impl<Id: ChId> Channel<Id, Busy> {
    /// Issue a software trigger to the channel
    #[inline]
    pub(crate) fn software_trigger(&mut self) {
        self._trigger_private();
    }

    /// Stop transfer on channel whether or not the transfer has completed, and
    /// return the resources it holds.
    ///
    /// # Return
    ///
    /// A `Channel` with a `Ready` status, ready to be reused by a new
    /// [`Transfer`](super::transfer::Transfer)
    #[inline]
    pub(crate) fn free(mut self) -> Channel<Id, Ready> {
        self.stop();
        self.change_status()
    }

    /// Restart transfer using previously-configured trigger source and action
    #[inline]
    pub(crate) fn restart(&mut self) {
        self._enable_private();
    }
}

impl<Id: ChId> From<Channel<Id, Ready>> for Channel<Id, Uninitialized> {
    fn from(mut item: Channel<Id, Ready>) -> Self {
        item._reset_private();
        item.change_status()
    }
}

#[cfg(feature = "async")]
impl<Id: ChId> Channel<Id, ReadyFuture> {
    /// Begin DMA transfer using `async` operation.
    ///
    /// If [`TriggerSource::Disable`] is used, a software
    /// trigger will be issued to the DMA channel to launch the transfer. It
    /// is therefore not necessary, in most cases, to manually issue a
    /// software trigger to the channel.
    ///
    /// # Safety
    ///
    /// In `async` mode, a transfer does NOT require `'static` source and
    /// destination buffers. This, in theory, makes
    /// [`transfer_future`](Channel::transfer_future) an `unsafe` function,
    /// although it is marked as safe for better ergonomics.
    ///
    /// This means that, as an user, you **must** ensure that the [`Future`]
    /// returned by this function may never be forgotten through [`forget`] or
    /// by wrapping it with a [`ManuallyDrop`].
    ///
    /// The returned future implements [`Drop`] and will automatically stop any
    /// ongoing transfers to guarantee that the memory occupied by the
    /// now-dropped buffers may not be corrupted by running transfers. This
    /// also means that should you [`forget`] this [`Future`] after its
    /// first [`poll`] call, the transfer will keep running, ruining the
    /// now-reclaimed memory, as well as the rest of your day.
    ///
    /// * `await`ing is fine: the [`Future`] will run to completion.
    /// * Dropping an incomplete transfer is also fine. Dropping can happen, for
    ///   example, if the transfer doesn't complete before a timeout expires.
    ///
    /// [`forget`]: core::mem::forget
    /// [`ManuallyDrop`]: core::mem::ManuallyDrop
    /// [`Future`]: core::future::Future
    /// [`poll`]: core::future::Future::poll
    #[inline]
    pub async fn transfer_future<S, D>(
        &mut self,
        mut source: S,
        mut dest: D,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
    ) -> Result<(), super::Error>
    where
        S: super::Buffer,
        D: super::Buffer<Beat = S::Beat>,
    {
        unsafe {
            self.transfer_future_linked(&mut source, &mut dest, trig_src, trig_act, None)
                .await
        }
    }

    /// Begin an async transfer, without changing the channel's type to
    /// [`Busy`].
    ///
    /// Also provides support for linked transfers via an optional `&mut
    /// DmacDescriptor`.
    ///
    /// # Safety
    ///
    /// * This method does not check that the two provided buffers have
    ///   compatible lengths. You must guarantee that:
    ///   - Either `source` or `dest` has a buffer length of 1, or
    ///   - Both buffers have the same length.
    /// * You must ensure that the transfer is completed or stopped before
    ///   returning the [`Channel`]. Doing otherwise breaks type safety, because
    ///   a [`ReadyFuture`] channel would still be in the middle of a transfer.
    /// * If the provided `linked_descriptor` is `Some` it must not be dropped
    ///   until the transfer is completed or stopped.
    /// * Additionnally, this function doesn't take `'static` buffers. Again,
    ///   you must guarantee that the returned transfer has completed or has
    ///   been stopped before giving up control of the underlying [`Channel`].
    pub(crate) async unsafe fn transfer_future_linked<S, D>(
        &mut self,
        source: &mut S,
        dest: &mut D,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
        linked_descriptor: Option<&mut DmacDescriptor>,
    ) -> Result<(), super::Error>
    where
        S: super::Buffer,
        D: super::Buffer<Beat = S::Beat>,
    {
        super::Transfer::<Self, super::transfer::BufferPair<S, D>>::check_buffer_pair(
            source, dest,
        )?;
        unsafe {
            self.fill_descriptor(source, dest, false);
            if let Some(next) = linked_descriptor {
                self.link_next(next as *mut _);
            }
        }

        self.disable_interrupts(
            InterruptFlags::new()
                .with_susp(true)
                .with_tcmpl(true)
                .with_terr(true),
        );

        self.configure_trigger(trig_src, trig_act);

        transfer_future::TransferFuture::new(self, trig_src).await

        // No need to defensively disable channel here; it's automatically
        // stopped when TransferFuture is dropped. Even though `stop()`
        // is implicitly called through TransferFuture::drop, it
        // *absolutely* must be called before this function is returned,
        // because it emits the compiler fence which ensures memory operations
        // aren't reordered beyond the DMA transfer's bounds.
    }
}

#[cfg(feature = "async")]
mod transfer_future {
    use super::*;

    /// [`Future`](core::future::Future) which starts, then waits on a DMA
    /// transfer.
    ///
    /// This implementation is a standalone struct instead of using
    /// [`poll_fn`](core::future::poll_fn), because we want to implement
    /// [`Drop`] for the future returned by the
    /// [`transfer_future`](super::Channel::transfer_future) method. This way we
    /// can stop transfers when they are dropped, thus avoiding undefined
    /// behaviour.
    pub(super) struct TransferFuture<'a, Id: ChId> {
        triggered: bool,
        trig_src: TriggerSource,
        chan: &'a mut Channel<Id, ReadyFuture>,
    }

    impl<'a, Id: ChId> TransferFuture<'a, Id> {
        pub(super) fn new(chan: &'a mut Channel<Id, ReadyFuture>, trig_src: TriggerSource) -> Self {
            Self {
                triggered: false,
                trig_src,
                chan,
            }
        }
    }

    impl<Id: ChId> Drop for TransferFuture<'_, Id> {
        fn drop(&mut self) {
            self.chan.stop();
        }
    }

    impl<Id: ChId> core::future::Future for TransferFuture<'_, Id> {
        type Output = Result<(), super::Error>;

        fn poll(
            mut self: core::pin::Pin<&mut Self>,
            cx: &mut core::task::Context<'_>,
        ) -> core::task::Poll<Self::Output> {
            use crate::dmac::waker::WAKERS;
            use core::task::Poll;

            let flags_to_check = InterruptFlags::new().with_tcmpl(true).with_terr(true);
            let triggered_flags = self.chan.check_and_clear_interrupts(flags_to_check);

            if triggered_flags.tcmpl() {
                Poll::Ready(Ok(()))
            } else if triggered_flags.terr() {
                Poll::Ready(Err(super::Error::TransferError))
            } else {
                WAKERS[Id::USIZE].register(cx.waker());
                self.chan.enable_interrupts(flags_to_check);
                self.chan._enable_private();

                if !self.triggered && self.trig_src == TriggerSource::Disable {
                    self.triggered = true;
                    self.chan._trigger_private();
                }

                Poll::Pending
            }
        }
    }
}

/// Interrupt sources available to a DMA channel
#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
pub struct InterruptFlags {
    /// Transfer error
    pub terr: bool,
    /// Transfer complete
    pub tcmpl: bool,
    /// Transfer suspended
    pub susp: bool,
    #[skip]
    _reserved: B5,
}

impl Default for InterruptFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate a [`DmacDescriptor`], and write it to the provided descriptor
/// reference.
///
/// `next` is the address of the next descriptor (for linked transfers). If
/// it is set to `0`, the transfer will terminate after this descriptor. For
/// circular transfers, set `next` to the descriptor's own address.
///
/// # Safety
///
/// * This method may only be called on a channel which is not actively being
///   used for transferring data.
///
/// * `next` must point to a valid [`DmacDescriptor`], with all the safety
///   considerations that entails: the source and destination buffers must be
///   valid, have compatible lengths, remain in scope for the entirety of the
///   transfer, etc.
///
/// * The pointer in the `descaddr` field of `next`, along with the descriptor
///   it points to, etc, must point to a valid [`DmacDescriptor`] memory
///   location, or be null. They must not be circular (ie, points to itself).
///   Any linked transfer must strictly be a read transaction (destination
///   pointer is a byte buffer, source pointer is the SERCOM DATA register).
#[inline]
pub(crate) unsafe fn write_descriptor<Src: Buffer, Dst: Buffer<Beat = Src::Beat>>(
    descriptor: &mut DmacDescriptor,
    source: &mut Src,
    destination: &mut Dst,
    next: *mut DmacDescriptor,
) {
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
    let btctrl = sram::BlockTransferControl::new()
        .with_srcinc(src_inc)
        .with_dstinc(dst_inc)
        .with_beatsize(Src::Beat::BEATSIZE)
        .with_valid(true);

    *descriptor = DmacDescriptor {
        // Next descriptor address:  0x0 terminates the transaction (no linked list),
        // any other address points to the next block descriptor
        descaddr: next,
        // Source address: address of the last beat transfer source in block
        srcaddr: src_ptr as *mut _,
        // Destination address: address of the last beat transfer destination in block
        dstaddr: dst_ptr as *mut _,
        // Block transfer count: number of beats in block transfer
        btcnt: length as u16,
        // Block transfer control: Datasheet  section 19.8.2.1 p.329
        btctrl,
    };
}
