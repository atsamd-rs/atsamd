//! DMA controller (DMAC)

use vcell::VolatileCell;

/// (DMAC_BTCTRL) Source Address Increment Enable
pub(super) const BTCTRL_SRCINC: u16 = 1 << 10;

/// (DMAC_BTCTRL) Destination Address Increment Enable
pub(super) const BTCTRL_DSTINC: u16 = 1 << 11;

/// (DMAC_BTCTRL) Step Selection
pub(super) const BTCTRL_STEPSEL: u16 = 1 << 12;

/// (DMAC_BTCTRL) Beat Size Mask
pub(super) const BTCTRL_BEATSIZE_MASK: u16 = 3 << BTCTRL_BEATSIZE_POS;

/// (DMAC_BTCTRL) Beat Size Mask
pub(super) const BTCTRL_BEATSIZE_POS: u16 = 8;

/// (DMAC_BTCTRL) Step Size Mask
pub(super) const BTCTRL_STEPSIZE_MASK: u16 = 7 << BTCTRL_STEPSIZE_POS;

/// (DMAC_BTCTRL) Step Size Mask
pub(super) const BTCTRL_STEPSIZE_POS: u16 = 13;

/// (DMAC_BTCTRL) Descriptor Valid
pub(super) const BTCTRL_VALID: u16 = 1;

/// (DMAC_CHINTENCLR) Mask Register
pub(super) const CHINTENCLR_MASK: u8 = 0x07;

/// Channel Transfer Error Interrupt Enable
pub(super) const CHINTENCLR_TERR: u8 = 1;

/// Channel Transfer Complete Interrupt Enable
pub(super) const CHINTENCLR_TCMPL: u8 = 1 << 1;

/// Channel Suspend Interrupt Enable
pub(super) const CHINTENCLR_SUSP: u8 = 1 << 2;

/// (DMAC_CHINTENSET) Mask Register
pub(super) const CHINTENSET_MASK: u8 = 0x07;

/// Disable event output
pub(super) const EVENT_OUTPUT_DISABLE: u16 = 0;

/// Number of DMAC channels (SAMD21)
#[cfg(not(feature = "samd51"))]
pub(super) const NUM_CHANNELS: usize = 1;

/// Number of DMAC channels (SAMD51)
#[cfg(feature = "samd51")]
pub(super) const NUM_CHANNELS: usize = 32;

#[cfg(not(feature = "samd51"))]
pub(super) const NVIC_PRIO_BITS: u16 = 2;

#[cfg(feature = "samd51")]
pub(super) const NVIC_PRIO_BITS: u16 = 3;

/// Channel IDs
pub(super) type ChannelId = u8;

/// Compute BTCTRL value for beat size
pub(super) fn btctrl_beatsize(size: u16) -> u16 {
    (0x3 << 8) & (size << 8)
}

/// Compute BTCTRL value for step size
pub(super) fn btctrl_stepsize(size: u16) -> u16 {
    (0x7 << 13) & (size << 13)
}

/// Beat sizes
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BeatSize {
    /// 8-bit
    Byte = 0,

    /// 16-bit
    HWord = 1,

    /// 32-bit
    Word = 2,
}

/// DMA descriptor
#[repr(align(16))]
pub struct Descriptor {
    /// 0x00 - Block Transfer Control
    pub btctrl: VolatileCell<u16>,

    /// 0x02 - Block Transfer Count
    pub btcnt: VolatileCell<u16>,

    /// 0x04 - Block Transfer Source Address
    pub srcaddr: VolatileCell<u32>,

    /// 0x08 - Block Transfer Destination Address
    pub dstaddr: VolatileCell<u32>,

    /// 0x0C - Next Descriptor Address
    pub descaddr: VolatileCell<u32>,
}

impl Default for Descriptor {
    fn default() -> Descriptor {
        Descriptor {
            btctrl: VolatileCell::new(0),
            btcnt: VolatileCell::new(0),
            srcaddr: VolatileCell::new(0),
            dstaddr: VolatileCell::new(0),
            descaddr: VolatileCell::new(0),
        }
    }
}

/// Descriptor and writeback sections for all DMAC channels
#[derive(Default)]
pub struct DescriptorList {
    /// Descriptor section for DMAC channels
    pub descriptor: [Descriptor; NUM_CHANNELS],

    /// Writeback section for DMAC channels
    pub writeback: [Descriptor; NUM_CHANNELS],
}

impl DescriptorList {
    /// Get DMAC channel `Descriptor`
    pub fn descriptor(&self, channel_id: ChannelId) -> &Descriptor {
        &self.descriptor[channel_id as usize]
    }

    /// Get DMAC channel writeback `Descriptor`
    pub fn writeback(&self, channel_id: ChannelId) -> &Descriptor {
        &self.descriptor[channel_id as usize]
    }
}

/// DMA priority
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Priority {
    /// Lowest priority (default)
    Priority0,

    /// Second lowest priority
    Priority1,

    /// Second highest priority
    Priority2,

    /// Highest priority
    Priority3,
}

impl Default for Priority {
    fn default() -> Priority {
        Priority::Priority0
    }
}

#[cfg(not(feature = "samd51"))]
pub(super) mod samd21 {
    /// IRQ type
    pub type IRQn = u8;

    /// IRQ value
    pub const IRQN: IRQn = 6;
}

#[cfg(feature = "samd51")]
pub(super) mod samd51 {
    use super::{ChannelId, NUM_CHANNELS};
    use crate::target_device::dmac;
    use core::mem;

    /// Size of the DMAC registers for a single channel:
    ///
    /// `u32 + (u8 * 8)` = 12 bytes
    const CHANNEL_REGISTERS_SIZE: u32 = 12;

    /// DMAC IRQn
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum IRQn {
        /// DMAC IRQn 0
        ///
        /// - DMAC_SUSP_0, DMAC_TCMPL_0, DMAC_TERR_0
        N0 = 31,

        /// DMAC IRQn 1
        ///
        /// - DMAC_SUSP_1, DMAC_TCMPL_1, DMAC_TERR_1
        N1 = 32,

        /// DMAC IRQn 2
        ///
        /// - DMAC_SUSP_2, DMAC_TCMPL_2, DMAC_TERR_2
        N2 = 33,

        /// DMAC IRQn 3
        ///
        /// - DMAC_SUSP_3, DMAC_TCMPL_3, DMAC_TERR_3
        N3 = 34,

        /// DMAC IRQn 4
        ///
        /// - DMAC_SUSP_4, DMAC_SUSP_5, DMAC_SUSP_6, DMAC_SUSP_7, DMAC_SUSP_8, DMAC_SUSP_9,
        /// - DMAC_SUSP_10, DMAC_SUSP_11, DMAC_SUSP_12, DMAC_SUSP_13, DMAC_SUSP_14, DMAC_SUSP_15,
        /// - DMAC_SUSP_16, DMAC_SUSP_17, DMAC_SUSP_18, DMAC_SUSP_19, DMAC_SUSP_20, DMAC_SUSP_21,
        /// - DMAC_SUSP_22, DMAC_SUSP_23, DMAC_SUSP_24, DMAC_SUSP_25, DMAC_SUSP_26, DMAC_SUSP_27,
        /// - DMAC_SUSP_28, DMAC_SUSP_29, DMAC_SUSP_30, DMAC_SUSP_31
        /// - DMAC_TCMPL_4, DMAC_TCMPL_5, DMAC_TCMPL_6, DMAC_TCMPL_7, DMAC_TCMPL_8, DMAC_TCMPL_9
        /// - DMAC_TCMPL_10, DMAC_TCMPL_11, DMAC_TCMPL_12, DMAC_TCMPL_13, DMAC_TCMPL_14, DMAC_TCMPL_15,
        /// - DMAC_TCMPL_16, DMAC_TCMPL_17, DMAC_TCMPL_18, DMAC_TCMPL_19, DMAC_TCMPL_20, DMAC_TCMPL_21,
        /// - DMAC_TCMPL_22, DMAC_TCMPL_23, DMAC_TCMPL_24, DMAC_TCMPL_25, DMAC_TCMPL_26, DMAC_TCMPL_27,
        /// - DMAC_TCMPL_28, DMAC_TCMPL_29, DMAC_TCMPL_30, DMAC_TCMPL_31
        /// - DMAC_TERR_4, DMAC_TERR_5, DMAC_TERR_6, DMAC_TERR_7, DMAC_TERR_8, DMAC_TERR_9
        /// - DMAC_TERR_10, DMAC_TERR_11, DMAC_TERR_12, DMAC_TERR_13, DMAC_TERR_14, DMAC_TERR_15,
        /// - DMAC_TERR_16, DMAC_TERR_17, DMAC_TERR_18, DMAC_TERR_19, DMAC_TERR_20, DMAC_TERR_21,
        /// - DMAC_TERR_22, DMAC_TERR_23, DMAC_TERR_24, DMAC_TERR_25, DMAC_TERR_26, DMAC_TERR_27,
        /// - DMAC_TERR_28, DMAC_TERR_29, DMAC_TERR_30, DMAC_TERR_31
        N4 = 35,
    }

    macro_rules! decl_channel_register {
        ($name:ident, $type:ty) => {
            fn $name(&self, channel_id: ChannelId) -> &$type;
        }
    }

    macro_rules! impl_channel_register {
        ($name:ident, $type:ty, $base:ident) => {
            fn $name(&self, channel_id: ChannelId) -> &$type {
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
}
