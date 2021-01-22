//! # Abstractions over individual DMA channels
//!
//! # Initializing
//!
//! Individual channels should be initialized through the [`Channel::init`](Channel::init) method.
//! This will return a `Channel<Ready, ID>` ready for use by a [`DmaTransfer`](super::transfer::DmaTransfer).
//! Initializing a channel requires setting a priority level, as well as enabling or disabling interrupt
//! requests (only for the specific channel being initialized).
//!
//! # Channel status
//!
//! Channels can be in any of three statuses: `Uninitialized`, `Ready`, and `Busy`. These statuses are checked
//! at compile time to ensure they are properly initialized before launching DMA transfers.
//!
//! # Resetting
//!
//! Calling the [`reset`](Channel::reset) method will reset the channel to its `Uninitialized` state. You
//! will be required to call [`init`](Channel::init) again before being able to use it with a `DmaTransfer`.

use super::dma_controller::{DmaController, PriorityLevel, TriggerAction, TriggerSource};
use crate::target_device::DMAC;
use crate::typelevel::Sealed;
use cortex_m::interrupt;

/// Uninitialized channel
pub struct Uninitialized;

/// Initialized and ready to transfer channel
pub struct Ready;

/// Busy channel
pub struct Busy;

/// DMA channel, capable of executing [`DmaTransfer`](super::transfer::DmaTransfer)s.
pub struct Channel<Status, const ID: u8> {
    _status: Status,
}

pub(crate) fn new_chan<const ID: u8>() -> Channel<Uninitialized, ID> {
    Channel {
        _status: Uninitialized,
    }
}

impl<S, const ID: u8> Sealed for Channel<S, ID> {}

/// These methods may be used on any DMA channel in any configuration
impl<S, const ID: u8> Channel<S, ID> {
    /// Configure the DMA channel so that it is ready to be used by a [`DmaTransfer`](super::transfer::DmaTransfer).
    ///
    /// # Return
    ///
    /// A `Channel` with a `Ready` status
    pub fn init(
        mut self,
        lvl: PriorityLevel,
        enable_interrupts: bool,
        controller: &mut DmaController,
    ) -> Channel<Ready, ID> {
        let dmac = controller.as_mut();

        // Software reset the channel for good measure
        self._reset_private(dmac);

        self.with_chid(dmac, |d| {
            // Reset the channel to its startup state and wait for reset to complete
            d.chctrla.modify(|_, w| w.swrst().set_bit());
            while d.chctrla.read().swrst().bit_is_set() {}

            // Setup priority level
            d.chctrlb.modify(|_, w| w.lvl().bits(lvl as u8));

            if enable_interrupts {
                // Enable all interrupt sources
                d.chintenset
                    .modify(|_, w| w.susp().set_bit().tcmpl().set_bit().terr().set_bit());
            }
        });

        Channel { _status: Ready }
    }

    /// Set channel ID and run the closure. A closure is needed to ensure
    /// the registers are accessed in an interrupt-safe way, as the SAMD21/51
    /// DMAC is a little funky - It requires setting the channel number in
    /// the CHID register, then access the channel control registers.
    /// If an interrupt were to change the CHID register, we would be faced
    /// with undefined behaviour.
    fn with_chid<F: Fn(&mut DMAC)>(&mut self, dmac: &mut DMAC, fun: F) {
        interrupt::free(|_| {
            // SAFETY: this is actually safe as long as we write a correct channel number to
            // the CHID register
            unsafe {
                dmac.chid.modify(|_, w| w.id().bits(ID));
            }

            fun(dmac);
        });
    }

    #[inline(always)]
    fn _reset_private(&mut self, dmac: &mut DMAC) {
        self.with_chid(dmac, |d| {
            // Reset the channel to its startup state and wait for reset to complete
            d.chctrla.modify(|_, w| w.swrst().set_bit());
            while d.chctrla.read().swrst().bit_is_set() {}
        })
    }

    #[inline(always)]
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
    /// Issue a software reset to the channel. This will return the channel to its startup state
    pub fn reset(mut self, dmac: &mut DMAC) -> Channel<Uninitialized, ID> {
        self._reset_private(dmac);

        Channel {
            _status: Uninitialized,
        }
    }

    /// Start transfer on channel using the specified trigger source.
    ///
    /// # Return
    ///
    /// A `Channel` with a `Busy` status.
    pub(crate) fn start(
        mut self,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
        dmac: &mut DMAC,
    ) -> Channel<Busy, ID> {
        // Set the channel ID. We assume the CHID register doesn't change
        // for the duration of this function.
        self.with_chid(dmac, |d| {
            // Configure the trigger source and trigger action
            // SAFETY: This is actually safe because we are writing the correct enum value
            // (imported from the PAC) into the register
            unsafe {
                d.chctrlb.modify(|_, w| {
                    w.trigsrc()
                        .bits(trig_src as u8)
                        .trigact()
                        .bits(trig_act as u8)
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
    pub(crate) fn software_trigger(&mut self, dmac: &mut DMAC) {
        self._trigger_private(dmac);
    }

    /// Stop transfer on channel whether or not the transfer has completed
    ///
    /// # Return
    ///
    /// A `Channel` with a `Ready` status, ready to be reused by a new
    /// [`DmaTransfer`](super::transfer::DmaTransfer)
    #[inline(always)]
    pub(crate) fn stop(mut self, dmac: &mut DMAC) -> Channel<Ready, ID> {
        self.with_chid(dmac, |d| d.chctrla.modify(|_, w| w.enable().clear_bit()));
        self.release(dmac)
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
    #[inline(always)]
    pub(crate) fn xfer_complete(&self, dmac: &mut DMAC) -> bool {
        dmac.busych.read().bits() & (1 << ID) == 0 && dmac.pendch.read().bits() & (1 << ID) == 0
    }

    /// Wait for the channel to clear its busy status, then release the channel.
    ///
    /// # Return
    ///
    /// A `Channel` with a `Ready` status, ready to be reused by a new
    /// [`DmaTransfer`](super::transfer::DmaTransfer)
    #[inline(always)]
    pub(crate) fn release(self, dmac: &mut DMAC) -> Channel<Ready, ID> {
        while !self.xfer_complete(dmac) {}
        Channel { _status: Ready }
    }
}
