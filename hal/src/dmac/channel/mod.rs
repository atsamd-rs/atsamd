//! # Abstractions over individual DMA channels
//!
//! # Initializing
//!
//! Individual channels should be initialized through the
//! [`Channel::init`] method. This will return a `Channel<Id, Ready>` ready for
//! use by a [`Transfer`](super::transfer::Transfer). Initializing a channel
//! requires setting a priority level, as well as enabling or disabling
//! interrupt requests (only for the specific channel being initialized).
#![cfg_attr(
    feature = "thumbv7",
    doc = "# Burst Length and FIFO Threshold (SAMD51/SAME5x only)

The transfer burst length can be configured through the
[`Channel::burst_length`] method. A burst is an atomic,
uninterruptible transfer which length corresponds to a number of beats. See
SAMD5x/E5x datasheet section 22.6.1.1 for more information. The FIFO
threshold can be configured through the
[`Channel::fifo_threshold`] method. This enables the channel
to wait for multiple Beats before sending a Burst. See SAMD5x/E5x datasheet
section 22.6.2.8 for more information."
)]
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

use super::dma_controller::{ChId, PriorityLevel, TriggerAction, TriggerSource};
use crate::typelevel::{Is, Sealed};
use core::marker::PhantomData;
use modular_bitfield::prelude::*;

mod reg;
use reg::RegisterBlock;

#[cfg(feature = "thumbv7")]
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
/// [`Transfer`](crate::dmac::transfer::Transfer)s. Ongoing DMA transfers are
/// automatically stopped when a [`Channel`] is dropped.
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
    pub fn init(mut self, lvl: PriorityLevel) -> Channel<Id, S::Ready> {
        // Software reset the channel for good measure
        self._reset_private();

        #[cfg(feature = "thumbv6")]
        // Setup priority level
        self.regs.chctrlb.modify(|_, w| w.lvl().bits(lvl as u8));

        #[cfg(feature = "thumbv7")]
        self.regs.chprilvl.modify(|_, w| w.prilvl().bits(lvl as u8));

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

    #[inline]
    fn _enable_private(&mut self) {
        self.regs.chctrla.modify(|_, w| w.enable().set_bit());
    }

    /// Stop transfer on channel whether or not the transfer has completed
    #[inline]
    pub(crate) fn stop(&mut self) {
        self.regs.chctrla.modify(|_, w| w.enable().clear_bit());
    }

    /// Returns whether or not the transfer is complete.
    #[inline]
    pub(crate) fn xfer_complete(&mut self) -> bool {
        !self.regs.chctrla.read().enable().bit_is_set()
    }

    /// Returns the transfer's success status.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn xfer_success(&mut self) -> super::Result<()> {
        let success = self.regs.chintflag.read().terr().bit_is_clear();
        success.then_some(()).ok_or(super::Error::TransferError)
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
    #[cfg(feature = "thumbv7")]
    #[inline]
    pub fn fifo_threshold(&mut self, threshold: FifoThreshold) {
        self.regs
            .chctrla
            .modify(|_, w| w.threshold().bits(threshold as u8));
    }

    /// Set burst length for the channel, in number of beats. A burst transfer
    /// is an atomic, uninterruptible operation.
    #[cfg(feature = "thumbv7")]
    #[inline]
    pub fn burst_length(&mut self, burst_length: BurstLength) {
        self.regs
            .chctrla
            .modify(|_, w| w.burstlen().bits(burst_length as u8));
    }

    /// Start the transfer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it starts the transfer without changing
    /// the channel status to [`Busy`]. A [`Ready`] channel which is actively
    /// transferring should NEVER be leaked.
    #[inline]
    pub(super) unsafe fn _start_private(
        &mut self,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
    ) {
        // Configure the trigger source and trigger action
        self.configure_trigger(trig_src, trig_act);
        self._enable_private();

        // If trigger source is DISABLE, manually trigger transfer
        if trig_src == TriggerSource::DISABLE {
            self._trigger_private();
        }
    }

    #[inline]
    pub(super) fn configure_trigger(&mut self, trig_src: TriggerSource, trig_act: TriggerAction) {
        // Configure the trigger source and trigger action
        #[cfg(feature = "thumbv6")]
        self.regs.chctrlb.modify(|_, w| {
            w.trigsrc().variant(trig_src);
            w.trigact().variant(trig_act)
        });

        #[cfg(feature = "thumbv7")]
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
        while !self.xfer_complete() {}
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
    /// If [TriggerSource::DISABLE](TriggerSource::DISABLE) is used, a software
    /// trigger will be issued to the DMA channel to launch the transfer. Is
    /// is therefore not necessary, in most cases, to manually issue a
    /// software trigger to the channel.
    ///
    /// # Safety
    ///
    /// In `async` mode, a transfer does NOT require `'static` source and
    /// destination buffers. This, in theory, makes
    /// [`transfer_future`](Channel::transfer_future) an `unsafe` function,
    /// although it is marked as safe (for ergonomics).
    ///
    /// This means that, as an user, you **must** ensure that the [`Future`]
    /// returned by this function may never be forgotten through [`forget`].
    /// [`Channel`]s implement [`Drop`] and will automatically s
    ///top any ongoing transfers to guarantee that the memor
    ///y occupied by the now-dropped buffers may
    /// not be corrupted by running transfers. This also means
    /// memory, memory, memory, memory, that should you [`forget`] this
    /// [`Future`] after it is first [`poll`] call, the transfer will keep
    /// running, ruining the now-reclaimed memory, as well as the rest of
    /// your day.
    ///
    /// * `await`ing is fine: the [`Future`] will run to completion.
    /// * Dropping an incomplete transfer is also fine. Dropping can happen,
    /// for example, if the transfer doesn't complete before a timeout
    /// expires.
    ///
    /// [`forget`]: core::mem::forget
    /// [`Future`]: core::future::Future
    /// [`poll`]: core::future::Future::poll
    #[cfg(feature = "async")]
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
        use crate::dmac::waker::WAKERS;
        use core::sync::atomic;
        use core::task::Poll;

        super::Transfer::<Self, super::transfer::BufferPair<S, D>>::check_buffer_pair(
            &source, &dest,
        )?;
        unsafe {
            super::Transfer::<Self, super::transfer::BufferPair<S, D>>::fill_descriptor(
                &mut source,
                &mut dest,
                false,
            )
        };

        self.disable_interrupts(
            InterruptFlags::new()
                .with_susp(true)
                .with_tcmpl(true)
                .with_terr(true),
        );

        self.configure_trigger(trig_src, trig_act);
        let mut triggered = false;

        core::future::poll_fn(|cx| {
            atomic::fence(atomic::Ordering::Release);
            self._enable_private();

            if !triggered && trig_src == TriggerSource::DISABLE {
                triggered = true;
                self._trigger_private();
            }

            let flags_to_check = InterruptFlags::new().with_tcmpl(true).with_terr(true);

            if self.check_and_clear_interrupts(flags_to_check).tcmpl() {
                return Poll::Ready(());
            }

            WAKERS[Id::USIZE].register(cx.waker());
            self.enable_interrupts(flags_to_check);

            if self.check_and_clear_interrupts(flags_to_check).tcmpl() {
                self.disable_interrupts(flags_to_check);

                return Poll::Ready(());
            }

            Poll::Pending
        })
        .await;

        // Defensively disable channel
        self.stop();
        // TODO currently this will always return Ok(()) since we unconditionally clear
        // ERROR
        self.xfer_success()
    }
}

/// Status of a transfer callback
#[derive(Clone, Copy)]
pub enum CallbackStatus {
    /// Transfer Complete
    TransferComplete,
    /// Transfer Error
    TransferError,
    /// Transfer Suspended
    TransferSuspended,
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
