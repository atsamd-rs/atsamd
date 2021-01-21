//! # Abstractions over individual DMA channels

use super::dma_controller::{DmaController, PriorityLevel, TriggerAction, TriggerSource};
use crate::target_device::DMAC;
use crate::typelevel::Sealed;
use cortex_m::interrupt;

/// Represents an uninitialized DMA channel
pub struct Uninitialized;

/// Represents an initialized and ready to begin DMA channel
pub struct Ready;

/// Represents a busy DMA channel
pub struct Busy;

/// Represents a DMA channel
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
    pub fn init(
        mut self,
        lvl: PriorityLevel,
        enable_interrupts: bool,
        controller: &mut DmaController,
    ) -> Channel<Ready, ID> {
        let dmac = controller.as_mut();

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
}

/// These methods may only be used on a configured DMA channel
impl<const ID: u8> Channel<Ready, ID> {
    /// Start transfer on channel using the specified trigger source
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
            // SAFETY: This is safe because we are writing the correct channel
            // number into the register
            unsafe {
                dmac.swtrigctrl.modify(|_, w| w.bits(1 << ID));
            }
        }

        Channel { _status: Busy }
    }
}

impl<const ID: u8> Channel<Busy, ID> {
    /// Stop transfer on channel whether or not the transfer has completed
    ///
    /// # Return
    ///
    /// A channel with a Ready status, ready to be reused by a new
    /// (`DmaTransfer`)[super::transfer::DmaTransfer]
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
    /// A channel with a Ready status, ready to be reused by a new
    /// (`DmaTransfer`)[super::transfer::DmaTransfer]
    #[inline(always)]
    pub(crate) fn release(self, dmac: &mut DMAC) -> Channel<Ready, ID> {
        while !self.xfer_complete(dmac) {}
        Channel { _status: Ready }
    }
}
