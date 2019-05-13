//! DMA controller (DMAC)

#[cfg(not(feature = "samd51"))]
mod samd21;
#[cfg(feature = "samd51")]
mod samd51;

#[cfg(not(feature = "samd51"))]
pub(super) use self::samd21::*;
#[cfg(feature = "samd51")]
pub(super) use self::samd51::*;
use super::error::Error;
use crate::target_device::{DMAC, NVIC};
use core::ops::Deref;
use cortex_m::interrupt::{CriticalSection, Mutex};
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

/// Channel IDs
pub(super) type ChannelId = u8;

/// Channel mask values
pub(super) type ChannelMask = u32;

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

impl BeatSize {
    /// Get the number of bytes per beat
    pub fn bytes_per_beat(self) -> usize {
        match self {
            BeatSize::Byte => 1,
            BeatSize::HWord => 2,
            BeatSize::Word => 4,
        }
    }
}

impl From<u16> for BeatSize {
    fn from(size: u16) -> BeatSize {
        match size {
            0 => BeatSize::Byte,
            1 => BeatSize::HWord,
            2 => BeatSize::Word,
            n => panic!("invalid beat size: {}", n),
        }
    }
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

/// DMA controller
pub(super) struct Controller {
    /// DMA controller state that should only be accessed from within a
    /// critical section
    critical: Mutex<ControllerCritical>,
}

impl Controller {
    /// Create a new DMA controller and initialize the underlying subsystem
    pub fn new(
        dmac: DMAC,
        clock_source: &mut ClockSource,
        nvic: &mut NVIC,
        descriptor_list: &DescriptorList,
    ) -> Result<Self, Error> {
        init_clock(clock_source);

        // Disable DMA controller
        dmac.ctrl.write(|reg| reg.dmaenable().clear_bit());

        // Perform software reset
        dmac.ctrl.write(|reg| reg.swrst().set_bit());

        // Initialize descriptor list addresses
        dmac.baseaddr.write(|reg| unsafe {
            reg.baseaddr()
                .bits(descriptor_list.descriptor.as_ptr() as u32)
        });

        dmac.wrbaddr.write(|reg| unsafe {
            reg.wrbaddr()
                .bits(descriptor_list.writeback.as_ptr() as u32)
        });

        // TODO(tarcieri): Wipe decriptor and writeback tables
        // self.descriptor_list.descriptor[0] = Descriptor::default();
        // self.descriptor_list.descriptor[0] = Descriptor::default();

        // Re-enable DMA controller with all priority levels
        dmac.ctrl.write(|reg| {
            reg.dmaenable()
                .set_bit()
                .lvlen0()
                .set_bit()
                .lvlen1()
                .set_bit()
                .lvlen2()
                .set_bit()
                .lvlen3()
                .set_bit()
        });

        // Enable DMAC IRQ(s)
        init_interrupts(nvic);

        Ok(Self {
            critical: Mutex::new(ControllerCritical::new(dmac)),
        })
    }

    /// Acquire a lock around the critical inner fields
    pub fn lock(&self) -> Guard {
        Guard {
            controller: self,
            section: unsafe { CriticalSection::new() },
        }
    }
}

/// MutexGuard-like wrapper around the critical DMAC fields
pub(super) struct Guard<'a> {
    /// Reference to an underlying controller
    controller: &'a Controller,

    /// Critical section
    section: CriticalSection,
}

impl<'a> Deref for Guard<'a> {
    type Target = ControllerCritical;

    fn deref(&self) -> &ControllerCritical {
        self.controller.critical.borrow(&self.section)
    }
}
