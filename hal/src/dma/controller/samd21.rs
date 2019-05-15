//! SAMD21 DMA controller (1 channel)

use crate::dma::{channel, Error};
use crate::target_device::{dmac, Interrupt, DMAC, NVIC};
use core::ops::Deref;
use vcell::VolatileCell;

/// Number of DMAC channels (SAMD21)
pub const NUM_CHANNELS: usize = 1;

/// NVIC priority bits
pub(crate) const NVIC_PRIO_BITS: u8 = 2;

/// DMA clock source
pub(crate) type ClockSource = crate::target_device::PM;

/// Initialize the underlying clock source
pub(super) fn init_clock(clock_source: &mut ClockSource) {
    clock_source.ahbmask.write(|reg| reg.dmac_().set_bit());
    clock_source.apbbmask.write(|reg| reg.dmac_().set_bit());
}

/// Enable DMAC IRQ
pub(super) fn init_interrupts(nvic: &mut NVIC) {
    nvic.enable(Interrupt::DMAC);
    unsafe {
        nvic.set_priority(Interrupt::DMAC, (1 << NVIC_PRIO_BITS) - 1);
    }
}

/// Critical DMAC fields
pub(crate) struct ControllerCritical {
    /// Underlying DMA controller
    dmac: DMAC,

    /// Bitmask of allocated channels
    channel_mask: VolatileCell<channel::Mask>,
}

impl ControllerCritical {
    /// Create a new critical DMA controller
    pub fn new(dmac: DMAC) -> Self {
        Self {
            dmac,
            channel_mask: VolatileCell::new(0),
        }
    }

    /// Borrow the DMAC registers
    pub fn registers(&self) -> &dmac::RegisterBlock {
        self.dmac.deref()
    }

    /// Read the current state of the channel mask
    pub fn channel_mask(&self) -> channel::Mask {
        self.channel_mask.get()
    }

    /// Read the current channel flags given a flag value
    pub fn read_channel_id_and_flags(&self, flags: u8) -> (u8, channel::Id) {
        (flags, 0)
    }

    /// Clear transfer error (TERR) flag
    pub fn clear_transfer_error_flag(&self, channel_id: channel::Id) {
        debug_assert_eq!(channel_id, 0);
        self.dmac.chintflag.write(|reg| reg.terr().clear_bit());
    }

    /// Clear transfer complete (TCMPL) flag
    pub fn clear_transfer_complete_flag(&self, channel_id: channel::Id) {
        debug_assert_eq!(channel_id, 0);
        self.dmac.chintflag.write(|reg| reg.tcmpl().set_bit());
    }

    /// Clear channel suspend (SUSP) flag
    pub fn clear_suspend_flag(&self, channel_id: channel::Id) {
        debug_assert_eq!(channel_id, 0);
        self.dmac.chintflag.write(|reg| reg.susp().set_bit());
    }

    /// Configure default behaviors
    pub fn configure_default_behaviors(
        &self,
        channel_id: channel::Id,
        peripheral_trigger: dmac::chctrlb::TRIGSRCW,
        trigger_action: dmac::chctrlb::TRIGACTW,
    ) {
        debug_assert_eq!(channel_id, 0);
        self.dmac.chctrlb.write(|reg| reg.lvl().lvl0());

        self.dmac
            .chctrlb
            .write(|reg| reg.trigsrc().variant(peripheral_trigger));

        self.dmac
            .chctrlb
            .write(|reg| reg.trigact().variant(trigger_action));
    }

    /// Set priority
    pub fn set_priority(&self, channel_id: channel::Id, priority: super::Priority) {
        debug_assert_eq!(channel_id, 0);
        self.dmac
            .chctrlb
            .write(|reg| reg.lvl().bits(priority as u8));
    }

    /// Set interrupt mask
    pub fn set_interrupt_mask(&self, channel_id: channel::Id, interrupt_mask: u8) {
        debug_assert_eq!(channel_id, 0);

        unsafe {
            self.dmac.chid.write(|reg| reg.id().bits(channel_id));

            self.dmac
                .chintenset
                .write(|reg| reg.bits(super::CHINTENSET_MASK & interrupt_mask));

            self.dmac
                .chintenclr
                .write(|reg| reg.bits(super::CHINTENCLR_MASK & !interrupt_mask));

            self.dmac.chctrla.write(|reg| reg.enable().set_bit());
        }
    }

    /// Set action
    pub fn set_action(&self, channel_id: channel::Id, action: dmac::chctrlb::TRIGACTW) {
        unsafe {
            self.dmac.chid.write(|reg| reg.id().bits(channel_id));
            self.dmac.chctrlb.write(|reg| reg.trigact().variant(action));
        }
    }

    /// Set trigger
    pub fn set_trigger(&self, channel_id: channel::Id, trigger: dmac::chctrlb::TRIGSRCW) {
        unsafe {
            self.dmac.chid.write(|reg| reg.id().bits(channel_id));
            self.dmac
                .chctrlb
                .write(|reg| reg.trigsrc().variant(trigger));
        }
    }

    /// Allocate the given channel
    pub fn allocate_channel(&self, channel_id: channel::Id) -> Result<(), Error> {
        let channel_mask = self.channel_mask();

        if channel_mask & (1 << channel_id) != 0 {
            return Err(Error::Busy);
        }

        // Mark channel as allocated
        self.channel_mask.set(channel_mask | 1 << channel_id);

        // Reset the allocated channel
        unsafe {
            self.dmac.chid.write(|reg| reg.id().bits(channel_id));
            self.dmac
                .chctrla
                .write(|reg| reg.enable().clear_bit().swrst().set_bit());
        }

        Ok(())
    }

    /// Release the given channel
    pub fn release_channel(
        &self,
        channel_id: channel::Id,
        clock_source: &mut ClockSource,
        nvic: &mut NVIC,
    ) -> Result<(), Error> {
        if self.channel_mask.get() & (1 << channel_id) != 0 {
            // Valid in-use channel; release it
            self.channel_mask
                .set(self.channel_mask.get() & !(1 << channel_id)); // Clear bit

            // No more channels in use?
            if self.channel_mask.get() == 0 {
                // Disable DMA interrupt
                nvic.disable(Interrupt::DMAC);

                // Disable DMA
                self.dmac.ctrl.write(|reg| reg.dmaenable().clear_bit());

                // Disable DMA clocks
                clock_source.apbbmask.write(|reg| reg.dmac_().clear_bit());
                clock_source.ahbmask.write(|reg| reg.dmac_().clear_bit());
            }

            Ok(())
        } else {
            Err(Error::NotInitialized) // Channel not in mask?
        }
    }

    /// Abort DMA operation
    pub fn abort(&self, channel_id: channel::Id) {
        unsafe {
            // Select channel
            self.dmac.chid.write(|reg| reg.bits(channel_id));

            // Disable
            self.dmac.chctrla.write(|reg| reg.bits(0));
        }
    }
}
