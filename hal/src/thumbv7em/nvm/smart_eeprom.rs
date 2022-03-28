//! # SmartEEPROM
//!
//! SmartEEPROM is a feature of NVM controller that simulates a RAM-like memory
//! within a flash. As bits in flash cannot switch from 0 to 1 because of its
//! properties (whole page of memory has to erased and data has to be recopied),
//! SmartEEPROM introduces an indirection mechanism that handles this issue via
//! notion of virtual pages and it handles physical page reallocation and
//! erasing automatically underneath.
//!
//! From a user perspective, SmartEEPROM behaves just like a piece of memory in
//! RAM but it is non-volatile. Data does not get lost between resets/power
//! cycles.
//!
//! From technical standpoint, NVM controller sacrifices last
//! `2*8192*NVMCTRL.SEESTAT.SBLK` bytes of flash (in an inactive bank). Memory
//! access through flash address space will cause HardFault. All accesses has to
//! be conducted through SmartEEPROM specific address space.
//!
//! Prerequisites:
//! Both `NVMCTRL.SEESTAT.{SBLK,PSZ}` (block size, virtual page size) are being
//! populated from proper bits in NVM controller user page on power-on-reset. By
//! default, `SBLK` property is set to `0`, effectively disabling SmartEEPROM.
//!
//! One of possible safe ways to change user page content is to use `OpenOCD`
//! custom commands. `atsame5x`'s `OpenOCD` driver supports `atsame5 userpage`
//! command. To access it from GDB, it has to be preceded with a `monitor`
//! clause.
//!
//! To access [`SmartEeprom`] struct, call [`Nvm::smart_eeprom`] method to
//! retrieve its instance.

use core::marker::PhantomData;

use super::Nvm;
use crate::pac::{nvmctrl::ctrlb::CMD_AW, NVMCTRL};
use crate::typelevel::Sealed;

/// Struct representing a SmartEEPROM instance.
///
/// It is generic over:
/// - a lifetime of a stored [`Nvm`] reference
/// - current state ([`Locked`]/[`Unlocked`])
pub struct SmartEeprom<'a, T: SmartEepromState> {
    nvm: &'a mut Nvm,
    virtual_size: usize,
    __: PhantomData<T>,
}

/// Trait generalizing over a state of an SmartEEPROM
pub trait SmartEepromState: Sealed {}

/// Type-level enum variant representing a locked state of SmartEEPROM. In that
/// state, only read operations are permitted
pub enum Locked {}
impl SmartEepromState for Locked {}
impl Sealed for Locked {}
/// Type-level enum variant representing an unlocked state of SmartEEPROM. In
/// that state, both read and write operations are permitted
pub enum Unlocked {}
impl SmartEepromState for Unlocked {}
impl Sealed for Unlocked {}

/// Enum representing possible failure modes of SmartEEPROM while its state is
/// being retrieved from HW registers.
#[derive(Debug)]
pub enum SmartEepromRetrievalFailure {
    /// SmartEEPROM is disabled and user page is misconfigured. [`More details
    /// in module-level documentation`](self).
    Disabled,
    /// Support for disabled automatic page reallocation is not implemented.
    DisabledAutomaticPageReallocationNotSupported,
    /// Support for buffered writes to NVM is not implemented.
    BufferedWritesNotSupported,
    /// `SBLK` must be in range `1..=10`. `SBLK` is represented by 4 bits in a
    /// user page which means that it can be between `0` and `15`. Documentation
    /// does not cover cases for `11..=15`, therefore API considers them
    /// unsupported.
    InvalidBlockCount {
        /// Currently set, unsupported `SBLK` value.
        sblk: u32,
    },
}

/// Enum encapsulating different modes SmartEEPROM can be in.
pub enum SmartEepromMode<'a> {
    /// SmartEEPROM is locked
    Locked(SmartEeprom<'a, Locked>),
    /// SmartEEPROM is unlocked
    Unlocked(SmartEeprom<'a, Unlocked>),
}

/// Type alias for locally used [`Result`] type.
pub type Result<'a> = core::result::Result<SmartEepromMode<'a>, SmartEepromRetrievalFailure>;

#[inline]
fn wait_if_busy() {
    // Workaround: Cannot access `NVMCTRL` through `self.nvm.nvm` because of double
    // borrowing in iterator for [`SmartEeprom::set`]. This should be safe though.
    let nvmctrl = unsafe { &*NVMCTRL::ptr() };
    while nvmctrl.seestat.read().busy().bit_is_set() {}
}

impl<'a> SmartEepromMode<'a> {
    /// Retrieve [`SmartEeprom`] instance using information found in relevant HW
    /// registers.
    pub(super) fn retrieve(nvm: &'a mut Nvm) -> Result<'a> {
        use SmartEepromMode as Mode;
        use SmartEepromRetrievalFailure::*;
        if nvm.nvm.seecfg.read().aprdis().bit_is_set() {
            return Err(DisabledAutomaticPageReallocationNotSupported);
        }
        if nvm.nvm.seecfg.read().wmode().is_buffered() {
            return Err(BufferedWritesNotSupported);
        }
        let sblk = nvm.nvm.seestat.read().sblk().bits() as u32;
        let psz = nvm.nvm.seestat.read().psz().bits() as u32;
        let virtual_size = match (sblk, psz) {
            (0, _) => return Err(Disabled),
            (sblk @ 11..=u32::MAX, _) => return Err(InvalidBlockCount { sblk }),
            (_, 8..=u32::MAX) => {
                unreachable!("`NVMCTRL.SEESTAT.PSZ` value is represented with 3 bits in user page")
            }
            others => Self::map_sblk_psz_to_virtual_size(others),
        };
        if nvm.nvm.seestat.read().lock().bit_is_set() {
            Ok(Mode::Locked(SmartEeprom {
                nvm,
                virtual_size,
                __: PhantomData::<Locked>,
            }))
        } else {
            Ok(Mode::Unlocked(SmartEeprom {
                nvm,
                virtual_size,
                __: PhantomData::<Unlocked>,
            }))
        }
    }

    fn map_sblk_psz_to_virtual_size(sblk_psz_pair: (u32, u32)) -> usize {
        match sblk_psz_pair {
            (_, 0) => 512,
            (_, 1) => 1024,
            (_, 2) => 2048,
            (_, 3) => 4096,
            (1, _) => 4096,
            (_, 4) => 8192,
            (2, _) => 8192,
            (_, 5) => 16384,
            (3 | 4, _) => 16384,
            (_, 6) => 32768,
            (5..=8, _) => 32768,
            (_, 7) => 65536,
            _ => unreachable!(),
        }
    }
}

impl<'a, T: SmartEepromState> SmartEeprom<'a, T> {
    const SEEPROM_ADDR: *mut usize = 0x44000000 as _;

    /// Returns an immutable slice to SmartEEPROM mapped address space.
    ///
    /// [`Underlying pointed type`](SmartEepromPointableSize) can be either
    /// [`u8`], [`u16`] or [`u32`].
    ///
    /// # Safety
    ///
    /// `NVMCTRL.SEESTAT.BUSY` register must be 0 before memory access can be
    /// performed.
    pub unsafe fn get_slice<TP: SmartEepromPointableSize>(&self) -> &[TP] {
        core::slice::from_raw_parts_mut(
            Self::SEEPROM_ADDR as _,
            self.virtual_size / core::mem::size_of::<TP>(),
        )
    }

    /// Retrieves data stored in SmartEEPROM at `offset` location and copies it
    /// to `buffer`.
    ///
    /// Note:
    /// `offset_in_bytes == sizeof::<TP>() * offset`
    pub fn get<TP: SmartEepromPointableSize>(&self, offset: usize, buffer: &mut [TP]) {
        let slice = unsafe { self.get_slice() };
        buffer
            .iter_mut()
            .zip(slice.iter().skip(offset))
            .for_each(|(target, source)| {
                wait_if_busy();
                *target = *source
            });
    }

    /// Returns an  iterator over SmartEEPROM address space.
    pub fn iter<TP: SmartEepromPointableSize>(&'a self) -> SmartEepromIter<'a, TP> {
        SmartEepromIter {
            iter: unsafe { self.get_slice().iter() },
        }
    }
}

/// Trait generalizing over primitive types that are permitted to be used as
/// slice backing types
pub trait SmartEepromPointableSize: Sealed + Copy {}

impl SmartEepromPointableSize for u8 {}
impl SmartEepromPointableSize for u16 {}
impl SmartEepromPointableSize for u32 {}

impl<'a> SmartEeprom<'a, Unlocked> {
    /// Returns a mutable slice to SmartEEPROM mapped address space.
    ///
    /// [`Underlying pointed type`](SmartEepromPointableSize) can be either
    /// [`u8`], [`u16`] or [`u32`].
    ///
    /// # Safety
    ///
    /// `NVMCTRL.SEESTAT.BUSY` register must be 0 before memory access can be
    /// performed.
    pub unsafe fn get_mut_slice<TP: SmartEepromPointableSize>(&mut self) -> &mut [TP] {
        core::slice::from_raw_parts_mut(
            Self::SEEPROM_ADDR as _,
            self.virtual_size / core::mem::size_of::<TP>(),
        )
    }

    /// Copies data in a `buffer` to SmartEEPROM at `offset` location
    ///
    /// Note:
    /// `offset_in_bytes == sizeof::<TP>() * offset`
    pub fn set<TP: SmartEepromPointableSize>(&mut self, offset: usize, buffer: &[TP]) {
        let slice = unsafe { self.get_mut_slice() };
        buffer
            .iter()
            .zip(slice.iter_mut().skip(offset))
            .for_each(|(source, target)| {
                wait_if_busy();
                *target = *source
            });
    }

    /// Returns a mutable iterator over SmartEEPROM address space.
    pub fn iter_mut<TP: SmartEepromPointableSize>(&'a mut self) -> SmartEepromIterMut<'a, TP> {
        SmartEepromIterMut {
            iter: unsafe { self.get_mut_slice().iter_mut() },
        }
    }

    /// Locks SmartEEPROM, allowing only to perform read operations
    pub fn lock(self) -> SmartEeprom<'a, Locked> {
        self.nvm.command_sync(CMD_AW::LSEE);
        let Self {
            nvm, virtual_size, ..
        } = self;
        SmartEeprom {
            nvm,
            virtual_size,
            __: PhantomData,
        }
    }
}

impl<'a> SmartEeprom<'a, Locked> {
    /// Unlocks SmartEEPROM, allowing to perform both read and write operations
    pub fn unlock(self) -> SmartEeprom<'a, Unlocked> {
        self.nvm.command_sync(CMD_AW::USEE);
        let Self {
            nvm, virtual_size, ..
        } = self;
        SmartEeprom {
            nvm,
            virtual_size,
            __: PhantomData,
        }
    }
}

/// A type representing an immutable iterator over SmartEEPROM address space
pub struct SmartEepromIter<'a, TP: SmartEepromPointableSize> {
    iter: core::slice::Iter<'a, TP>,
}

impl<'a, TP: SmartEepromPointableSize> Iterator for SmartEepromIter<'a, TP> {
    type Item = &'a TP;
    fn next(&mut self) -> Option<Self::Item> {
        wait_if_busy();
        self.iter.next()
    }
}

impl<'a, TP: SmartEepromPointableSize> DoubleEndedIterator for SmartEepromIter<'a, TP> {
    fn next_back(&mut self) -> Option<Self::Item> {
        wait_if_busy();
        self.iter.next_back()
    }
}

/// A type representing a mutable iterator over SmartEEPROM address space
pub struct SmartEepromIterMut<'a, TP: SmartEepromPointableSize> {
    iter: core::slice::IterMut<'a, TP>,
}

impl<'a, TP: SmartEepromPointableSize> Iterator for SmartEepromIterMut<'a, TP> {
    type Item = &'a mut TP;
    fn next(&mut self) -> Option<Self::Item> {
        wait_if_busy();
        self.iter.next()
    }
}

impl<'a, TP: SmartEepromPointableSize> DoubleEndedIterator for SmartEepromIterMut<'a, TP> {
    fn next_back(&mut self) -> Option<Self::Item> {
        wait_if_busy();
        self.iter.next_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_if_virtual_size_is_mapped_correctly() {
        // For some reason, doing
        // `use SmartEepromMode::map_sblk_psz_to_virtual_size as f;`
        // is not permitted - 🤷
        fn f(sblk: u32, psz: u32) -> usize {
            SmartEepromMode::map_sblk_psz_to_virtual_size((sblk, psz))
        }
        assert_eq!(f(1, 0), 512);
        assert_eq!(f(1, 1), 1024);
        assert_eq!(f(1, 2), 2048);
        assert_eq!(f(1, 3), 4096);
        assert_eq!(f(1, 4), 4096);
        assert_eq!(f(1, 5), 4096);
        assert_eq!(f(1, 6), 4096);
        assert_eq!(f(1, 7), 4096);
        assert_eq!(f(2, 0), 512);
        assert_eq!(f(2, 1), 1024);
        assert_eq!(f(2, 2), 2048);
        assert_eq!(f(2, 3), 4096);
        assert_eq!(f(2, 4), 8192);
        assert_eq!(f(2, 5), 8192);
        assert_eq!(f(2, 6), 8192);
        assert_eq!(f(2, 7), 8192);
        assert_eq!(f(3, 0), 512);
        for i in 3..=4 {
            assert_eq!(f(i, 1), 1024);
            assert_eq!(f(i, 2), 2048);
            assert_eq!(f(i, 3), 4096);
            assert_eq!(f(i, 4), 8192);
            assert_eq!(f(i, 5), 16384);
            assert_eq!(f(i, 6), 16384);
            assert_eq!(f(i, 7), 16384);
        }
        for i in 5..=8 {
            assert_eq!(f(i, 0), 512);
            assert_eq!(f(i, 1), 1024);
            assert_eq!(f(i, 2), 2048);
            assert_eq!(f(i, 3), 4096);
            assert_eq!(f(i, 4), 8192);
            assert_eq!(f(i, 5), 16384);
            assert_eq!(f(i, 6), 32768);
            assert_eq!(f(i, 7), 32768);
        }
        for i in 9..=10 {
            assert_eq!(f(i, 0), 512);
            assert_eq!(f(i, 1), 1024);
            assert_eq!(f(i, 2), 2048);
            assert_eq!(f(i, 3), 4096);
            assert_eq!(f(i, 4), 8192);
            assert_eq!(f(i, 5), 16384);
            assert_eq!(f(i, 6), 32768);
            assert_eq!(f(i, 7), 65536);
        }
    }
}
