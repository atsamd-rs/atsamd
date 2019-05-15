//! Direct Memory Access descriptors

use super::channel;
use super::controller::NUM_CHANNELS;
use vcell::VolatileCell;

/// DMA descriptor
#[repr(align(16))]
pub struct Descriptor {
    /// 0x00 - Block Transfer Control
    pub btctrl: VolatileCell<u16>,

    /// 0x02 - Block Transfer Count
    pub btcnt: VolatileCell<u16>,

    /// 0x04 - Block Transfer Source Address
    pub srcaddr: VolatileCell<*const u8>,

    /// 0x08 - Block Transfer Destination Address
    pub dstaddr: VolatileCell<*mut u8>,

    /// 0x0C - Next Descriptor Address
    pub descaddr: VolatileCell<*const Descriptor>,
}

impl Default for Descriptor {
    fn default() -> Descriptor {
        Descriptor {
            btctrl: VolatileCell::new(0),
            btcnt: VolatileCell::new(0),
            srcaddr: VolatileCell::new(0 as *const u8),
            dstaddr: VolatileCell::new(0 as *mut u8),
            descaddr: VolatileCell::new(0 as *const Descriptor),
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
    pub fn descriptor(&self, channel_id: channel::Id) -> &Descriptor {
        &self.descriptor[channel_id as usize]
    }

    /// Get DMAC channel writeback `Descriptor`
    pub fn writeback(&self, channel_id: channel::Id) -> &Descriptor {
        &self.descriptor[channel_id as usize]
    }
}
