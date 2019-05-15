//! DMA controller (DMAC)

#[cfg(not(feature = "samd51"))]
mod samd21;
#[cfg(feature = "samd51")]
mod samd51;

#[cfg(not(feature = "samd51"))]
pub(super) use self::samd21::*;
#[cfg(feature = "samd51")]
pub(super) use self::samd51::*;
use super::descriptor::DescriptorList;
use super::error::Error;
use crate::target_device::{DMAC, NVIC};
use core::ops::Deref;
use cortex_m::interrupt::{CriticalSection, Mutex};

/// (DMAC_CHINTENCLR) Mask Register
const CHINTENCLR_MASK: u8 = 0x07;

/// (DMAC_CHINTENSET) Mask Register
const CHINTENSET_MASK: u8 = 0x07;

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

/// Direct Memory Access controller
pub struct Controller {
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
    pub(super) fn lock(&self) -> Guard {
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
