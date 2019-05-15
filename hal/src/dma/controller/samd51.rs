//! SAMD51 DMA controller (32 channels)

use crate::dma::{channel, Error};
use crate::target_device::{dmac, Interrupt, DMAC, NVIC};
use core::{mem, ops::Deref};
use vcell::VolatileCell;

/// Size of the DMAC registers for a single channel:
///
/// `u32 + (u8 * 8)` = 12 bytes
const CHANNEL_REGISTERS_SIZE: u32 = 12;

/// Number of DMAC channels (SAMD51)
pub const NUM_CHANNELS: usize = 32;

/// NVIC priority bits
pub(crate) const NVIC_PRIO_BITS: u8 = 3;

/// DMA clock source
pub(crate) type ClockSource = crate::target_device::MCLK;

/// Initialize the underlying clock source
pub(super) fn init_clock(clock_source: &mut ClockSource) {
    clock_source.ahbmask.write(|reg| reg.dmac_().set_bit());
}

macro_rules! init_interrupt {
    ($nvic:expr, $int:expr) => {
        $nvic.enable($int);
        unsafe {
            $nvic.set_priority($int, (1 << NVIC_PRIO_BITS) - 1);
        }
    };
}

/// Enable DMA interrupt at lowest priority
pub(super) fn init_interrupts(nvic: &mut NVIC) {
    // Enable DMAC interrupts
    init_interrupt!(nvic, Interrupt::DMAC_0);
    init_interrupt!(nvic, Interrupt::DMAC_1);
    init_interrupt!(nvic, Interrupt::DMAC_2);
    init_interrupt!(nvic, Interrupt::DMAC_3);
    init_interrupt!(nvic, Interrupt::DMAC_OTHER);
}

/// Critical DMAC fields
pub(crate) struct ControllerCritical {
    /// Underlying DMA controller
    dmac: DMAC,

    /// Bitmask of allocated channels
    channel_mask: VolatileCell<u32>,
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
    pub fn read_channel_id_and_flags(&self, channel_id: channel::Id) -> (channel::Id, u8) {
        let flags = self.dmac.chintflag(channel_id).read().bits();
        (channel_id, flags)
    }

    /// Clear transfer error (TERR) flag
    pub fn clear_transfer_error_flag(&self, channel_id: channel::Id) {
        self.dmac
            .chintflag(channel_id)
            .write(|reg| reg.terr().clear_bit());
    }

    /// Clear transfer complete (TCMPL) flag
    pub fn clear_transfer_complete_flag(&self, channel_id: channel::Id) {
        self.dmac
            .chintflag(channel_id)
            .write(|reg| reg.tcmpl().set_bit());
    }

    /// Clear channel suspend (SUSP) flag
    pub fn clear_suspend_flag(&self, channel_id: channel::Id) {
        self.dmac
            .chintflag(channel_id)
            .write(|reg| reg.susp().set_bit());
    }

    /// Configure default behaviors
    pub fn configure_default_behaviors(
        &self,
        channel_id: channel::Id,
        peripheral_trigger: dmac::chctrla::TRIGSRCW,
        trigger_action: dmac::chctrla::TRIGACTW,
    ) {
        self.dmac
            .chprilvl(channel_id)
            .write(|reg| reg.prilvl().lvl0());

        self.dmac.chctrla(channel_id).write(|reg| {
            reg.trigsrc()
                .variant(peripheral_trigger)
                .trigact()
                .variant(trigger_action)
                .burstlen()
                .single()
        });
    }

    /// Set priority
    pub fn set_priority(&self, channel_id: channel::Id, priority: super::Priority) {
        unsafe {
            self.dmac
                .chprilvl(channel_id)
                .write(|reg| reg.prilvl().bits(priority as u8));
        }
    }

    /// Set interrupt mask
    pub fn set_interrupt_mask(&self, channel_id: channel::Id, interrupt_mask: u8) {
        unsafe {
            self.dmac
                .chintenset(channel_id)
                .write(|reg| reg.bits(super::CHINTENSET_MASK & interrupt_mask));

            self.dmac
                .chintenclr(channel_id)
                .write(|reg| reg.bits(super::CHINTENCLR_MASK & !interrupt_mask));

            self.dmac
                .chctrla(channel_id)
                .write(|reg| reg.enable().set_bit());
        }
    }

    /// Set action
    pub fn set_action(&self, channel_id: channel::Id, action: dmac::chctrla::TRIGACTW) {
        self.dmac
            .chctrla(channel_id)
            .write(|reg| reg.trigact().variant(action));
    }

    /// Set trigger
    pub fn set_trigger(&self, channel_id: channel::Id, trigger: dmac::chctrla::TRIGSRCW) {
        self.dmac
            .chctrla(channel_id)
            .write(|reg| reg.trigsrc().variant(trigger));
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
        self.dmac
            .chctrla(channel_id)
            .write(|reg| reg.enable().clear_bit().swrst().set_bit());

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
                // TODO(tarcieri): double check this is the right interrupt
                nvic.disable(Interrupt::DMAC_0);

                // Disable DMA
                self.dmac.ctrl.write(|reg| reg.dmaenable().clear_bit());

                // Disable DMA clock
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
            self.dmac.chctrla(channel_id).write(|reg| reg.bits(0));
        }
    }
}

macro_rules! decl_channel_register {
        ($name:ident, $type:ty) => {
            fn $name(&self, channel_id: channel::Id) -> &$type;
        }
    }

macro_rules! impl_channel_register {
        ($name:ident, $type:ty, $base:ident) => {
            fn $name(&self, channel_id: channel::Id) -> &$type {
                debug_assert!(
                    channel_id < NUM_CHANNELS as u8,
                    "invalid channel ID: {} (max {})",
                    channel_id,
                    NUM_CHANNELS
                );

                let base_addr = (&self.$base as *const $type) as u32;
                unsafe { mem::transmute(base_addr + CHANNEL_REGISTERS_SIZE) }
            }
        }
    }

/// Extension trait for accessing DMAC channel registers
pub(crate) trait ChannelRegisters {
    decl_channel_register!(chctrla, dmac::CHCTRLA);
    decl_channel_register!(chctrlb, dmac::CHCTRLB);
    decl_channel_register!(chprilvl, dmac::CHPRILVL);
    decl_channel_register!(chevctrl, dmac::CHEVCTRL);
    decl_channel_register!(chintenclr, dmac::CHINTENCLR);
    decl_channel_register!(chintenset, dmac::CHINTENSET);
    decl_channel_register!(chintflag, dmac::CHINTFLAG);
    decl_channel_register!(chstatus, dmac::CHSTATUS);
}

impl ChannelRegisters for dmac::RegisterBlock {
    impl_channel_register!(chctrla, dmac::CHCTRLA, chctrla0);
    impl_channel_register!(chctrlb, dmac::CHCTRLB, chctrlb0);
    impl_channel_register!(chprilvl, dmac::CHPRILVL, chprilvl0);
    impl_channel_register!(chevctrl, dmac::CHEVCTRL, chevctrl0);
    impl_channel_register!(chintenclr, dmac::CHINTENCLR, chintenclr0);
    impl_channel_register!(chintenset, dmac::CHINTENSET, chintenset0);
    impl_channel_register!(chintflag, dmac::CHINTFLAG, chintflag0);
    impl_channel_register!(chstatus, dmac::CHSTATUS, chstatus0);
}
