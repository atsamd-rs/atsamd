//! # Abstractions over individual DMA channels
//!
//! # Initializing
//!
//! Individual channels should be initialized through the
//! [`Channel::init`](Channel::init) method. This will return a `Channel<Ready,
//! ID>` ready for use by a [`Transfer`](super::transfer::Transfer).
//! Initializing a channel requires setting a priority level, as well as
//! enabling or disabling interrupt requests (only for the specific channel
//! being initialized).
//!
//! # Burst Length and FIFO Threshold (SAMD51/SAME5x only)
//!
//! The transfer burst length can be configured through the
//! [`burst_length`](Channel::burst_length) method. A burst is an atomic,
//! uninterruptible transfer which length corresponds to a number of beats. See
//! SAMD5x/E5x datasheet section 22.6.1.1 for more information. The FIFO
//! threshold can be configured through the
//! [`fifo_threshold`](Channel::fifo_threshold) method. This enables the channel
//! to wait for multiple Beats before sending a Burst. See SAMD5x/E5x datasheet
//! section 22.6.2.8 for more information.
//!
//! # Channel status
//!
//! Channels can be in any of three statuses: `Uninitialized`, `Ready`, and
//! `Busy`. These statuses are checked at compile time to ensure they are
//! properly initialized before launching DMA transfers.
//!
//! # Resetting
//!
//! Calling the [`reset`](Channel::reset) method will reset the channel to its
//! `Uninitialized` state. You will be required to call [`init`](Channel::init)
//! again before being able to use it with a `Transfer`.

use super::dma_controller::{DmaController, PriorityLevel, TriggerAction, TriggerSource};

#[cfg(feature = "min-samd51g")]
use super::dma_controller::{BurstLength, FifoThreshold};

#[cfg(feature = "min-samd51g")]
use crate::target_device::dmac::CHANNEL;

use crate::{target_device::DMAC, typelevel::Sealed};

pub trait Status: Sealed {}

/// Uninitialized channel
pub struct Uninitialized;
impl Sealed for Uninitialized {}
impl Status for Uninitialized {}
/// Initialized and ready to transfer channel
pub struct Ready;
impl Sealed for Ready {}
impl Status for Ready {}
/// Busy channel
pub struct Busy;
impl Sealed for Busy {}
impl Status for Busy {}

/// DMA channel, capable of executing
/// [`Transfer`](super::transfer::Transfer)s.
pub struct Channel<S: Status, const ID: u8> {
    _status: S,
}

#[inline]
pub(crate) fn new_chan<const ID: u8>() -> Channel<Uninitialized, ID> {
    Channel {
        _status: Uninitialized,
    }
}

/// These methods may be used on any DMA channel in any configuration
impl<S: Status, const ID: u8> Channel<S, ID> {
    /// Set channel ID and run the closure. A closure is needed to ensure
    /// the registers are accessed in an interrupt-safe way, as the SAMD21
    /// DMAC is a little funky - It requires setting the channel number in
    /// the CHID register, then access the channel control registers.
    /// If an interrupt were to change the CHID register, we would be faced
    /// with undefined behaviour.
    #[cfg(any(feature = "samd11", feature = "samd21"))]
    fn with_chid<F: Fn(&mut DMAC)>(&mut self, dmac: &mut DMAC, fun: F) {
        cortex_m::interrupt::free(|_| {
            // SAFETY: this is actually safe as long as we write a correct channel number to
            // the CHID register
            unsafe {
                dmac.chid.modify(|_, w| w.id().bits(ID));
            }

            fun(dmac);
        });
    }

    /// Set channel ID and run the closure. A closure is needed to ensure
    /// the registers are accessed in an interrupt-safe way, as the SAMD21
    /// DMAC is a little funky. For the SAMD51/SAMEx, we simply take a reference
    /// to the correct channel number and run the closure on that.
    #[cfg(feature = "min-samd51g")]
    #[inline]
    fn with_chid<F: Fn(&CHANNEL)>(&mut self, dmac: &mut DMAC, fun: F) {
        let mut ch = &dmac.channel[ID as usize];
        fun(&mut ch);
    }

    /// Configure the DMA channel so that it is ready to be used by a
    /// [`Transfer`](super::transfer::Transfer).
    ///
    /// # Return
    ///
    /// A `Channel` with a `Ready` status
    pub fn init(
        mut self,
        controller: &mut DmaController,
        lvl: PriorityLevel,
        enable_interrupts: bool,
    ) -> Channel<Ready, ID> {
        // SAFETY: this is safe because we only mutably borrow dmac once.
        let dmac = unsafe { controller.dmac_mut() };

        // Software reset the channel for good measure
        self._reset_private(dmac);

        self.with_chid(dmac, |d| {
            #[cfg(any(feature = "samd11", feature = "samd21"))]
            // Setup priority level
            d.chctrlb.modify(|_, w| w.lvl().bits(lvl as u8));

            #[cfg(feature = "min-samd51g")]
            d.chprilvl.modify(|_, w| w.prilvl().bits(lvl as u8));

            if enable_interrupts {
                // Enable all interrupt sources
                d.chintenset.modify(|_, w| {
                    w.susp().set_bit();
                    w.tcmpl().set_bit();
                    w.terr().set_bit()
                });
            }
        });

        Channel { _status: Ready }
    }

    #[inline]
    fn _reset_private(&mut self, dmac: &mut DMAC) {
        self.with_chid(dmac, |d| {
            // Reset the channel to its startup state and wait for reset to complete
            d.chctrla.modify(|_, w| w.swrst().set_bit());
            while d.chctrla.read().swrst().bit_is_set() {}
        })
    }

    #[inline]
    fn _trigger_private(&mut self, dmac: &mut DMAC) {
        // SAFETY: This is safe because we are writing the correct channel
        // number into the register
        unsafe {
            dmac.swtrigctrl.modify(|_, w| w.bits(1 << ID));
        }
    }
}

/// These methods may only be used on a `Ready` DMA channel
impl<const ID: u8> Channel<Ready, ID> {
    /// Issue a software reset to the channel. This will return the channel to
    /// its startup state
    #[inline]
    pub fn reset(mut self, dmac: &mut DMAC) -> Channel<Uninitialized, ID> {
        self._reset_private(dmac);

        Channel {
            _status: Uninitialized,
        }
    }

    /// Set the FIFO threshold length. The channel will wait until it has
    /// received the selected number of Beats before triggering the Burst
    /// transfer, reducing the DMA transfer latency.
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn fifo_threshold(&mut self, dmac: &mut DmaController, threshold: FifoThreshold) {
        // SAFETY: This is safe because we only borrow dmac once.
        let dmac = unsafe { dmac.dmac_mut() };
        self.with_chid(dmac, |d| {
            d.chctrla.modify(|_, w| w.threshold().bits(threshold as u8));
        })
    }

    /// Set burst length for the channel, in number of beats. A burst transfer
    /// is an atomic, uninterruptible operation.
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn burst_length(&mut self, dmac: &mut DmaController, burst_length: BurstLength) {
        // SAFETY: This is safe because we only borrow dmac once.
        let dmac = unsafe { dmac.dmac_mut() };
        self.with_chid(dmac, |d| {
            d.chctrla
                .modify(|_, w| w.burstlen().bits(burst_length as u8));
        })
    }

    /// Start transfer on channel using the specified trigger source.
    ///
    /// # Return
    ///
    /// A `Channel` with a `Busy` status.
    pub(crate) fn start(
        mut self,
        dmac: &mut DMAC,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
    ) -> Channel<Busy, ID> {
        // Set the channel ID. We assume the CHID register doesn't change
        // for the duration of this function.
        self.with_chid(dmac, |d| {
            // Configure the trigger source and trigger action
            // SAFETY: This is actually safe because we are writing the correct enum value
            // (imported from the PAC) into the register

            #[cfg(any(feature = "samd11", feature = "samd21"))]
            let trigger_channel = &d.chctrlb;

            #[cfg(feature = "min-samd51g")]
            let trigger_channel = &d.chctrla;

            unsafe {
                trigger_channel.modify(|_, w| {
                    w.trigsrc().bits(trig_src as u8);
                    w.trigact().bits(trig_act as u8)
                });
            }

            // Start channel
            d.chctrla.modify(|_, w| w.enable().set_bit());
        });

        // If trigger source is DISABLE, manually trigger transfer
        if trig_src == TriggerSource::DISABLE {
            self._trigger_private(dmac);
        }

        Channel { _status: Busy }
    }
}

/// These methods may only be used on a `Busy` DMA channel
impl<const ID: u8> Channel<Busy, ID> {
    /// Issue a software trigger to the channel
    #[inline]
    pub(crate) fn software_trigger(&mut self, dmac: &mut DMAC) {
        self._trigger_private(dmac);
    }

    /// Stop transfer on channel whether or not the transfer has completed
    ///
    /// # Return
    ///
    /// A `Channel` with a `Ready` status, ready to be reused by a new
    /// [`Transfer`](super::transfer::Transfer)
    #[inline]
    pub(crate) fn stop(mut self, dmac: &mut DMAC) -> Channel<Ready, ID> {
        self.with_chid(dmac, |d| d.chctrla.modify(|_, w| w.enable().clear_bit()));
        self.free(dmac)
    }

    /// Returns whether or not the transfer is complete.
    ///
    /// BUSYCH is set when the channel is ACTIVELY transferring;
    /// PENDCH is set when a trigger request has been received
    /// but the transfer hasn't been started yet.
    /// Therefore, when a trigger request is issued, PENDCH will be set first,
    /// then when the arbiter begins to service the channel, PENDCH is cleared
    /// and BUSYCH is set. To make sure the transfer is actually complete, the
    /// channel needs to be both NOT PENDING and NOT BUSY.
    #[inline]
    pub(crate) fn xfer_complete(&self, dmac: &mut DMAC) -> bool {
        dmac.busych.read().bits() & (1 << ID) == 0 && dmac.pendch.read().bits() & (1 << ID) == 0
    }

    /// Wait for the channel to clear its busy status, then release the channel.
    ///
    /// # Return
    ///
    /// A `Channel` with a `Ready` status, ready to be reused by a new
    /// [`Transfer`](super::transfer::Transfer)
    #[inline]
    pub(crate) fn free(self, dmac: &mut DMAC) -> Channel<Ready, ID> {
        while !self.xfer_complete(dmac) {}
        Channel { _status: Ready }
    }
}
