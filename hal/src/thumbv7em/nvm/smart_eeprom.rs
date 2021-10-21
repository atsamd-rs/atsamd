use atsame51n::nvmctrl::ctrlb::CMD_AW;

use core::marker::PhantomData;

use super::Nvm;
use crate::typelevel::Sealed;

pub struct SmartEeprom<'a, T: SmartEepromState> {
    nvm: &'a mut Nvm,
    virtual_size: usize,
    __: PhantomData<T>,
}

pub trait SmartEepromState: Sealed {}

pub enum Locked {}
impl SmartEepromState for Locked {}
impl Sealed for Locked {}
pub enum Unlocked {}
impl SmartEepromState for Unlocked {}
impl Sealed for Unlocked {}

pub enum SmartEepromRetrievalFailure {
    InvalidSmartEepromBlockCount { incorrect_sblk: u32 },
}

pub enum SmartEepromMode<'a> {
    Locked(SmartEeprom<'a, Locked>),
    Unlocked(SmartEeprom<'a, Unlocked>),
}

pub type Result<'a> = core::result::Result<SmartEepromMode<'a>, SmartEepromRetrievalFailure>;

impl<'a> SmartEepromMode<'a> {
    /// TODO:
    /// Note: Does HW call
    pub(super) fn retrieve(nvm: &'a mut Nvm) -> Result<'a> {
        use SmartEepromMode as Mode;
        use SmartEepromRetrievalFailure::*;
        // TODO: Check NVMCTRL.SEECFG and call unimplemented maybe?
        let sblk = nvm.nvm.seestat.read().sblk().bits() as u32;
        let psz = nvm.nvm.seestat.read().psz().bits() as u32;
        // TODO: Unit test for this?
        let virtual_size = match (sblk, psz) {
            (incorrect_sblk @ (0 | 11..), _) => {
                return Err(InvalidSmartEepromBlockCount { incorrect_sblk })
            }
            (_, 8..) => unreachable!("`psz` value is represented with 3 bits"),
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
        };
        if Self::is_locked(nvm) {
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

    fn is_locked(nvm: &Nvm) -> bool {
        nvm.nvm.seestat.read().lock().bit_is_set()
    }
}

impl<'a, T: SmartEepromState> SmartEeprom<'a, T> {
    const SEEPROM_ADDR: *mut usize = 0x44000000 as _;

    pub unsafe fn get_slice<TP: SmartEepromPointableSize>(&self) -> &[TP] {
        core::slice::from_raw_parts_mut(
            Self::SEEPROM_ADDR as _,
            self.virtual_size / core::mem::size_of::<TP>(),
        )
    }

    pub fn get<TP: SmartEepromPointableSize>(&self, offset: usize, buffer: &mut [TP]) {
        self.wait_if_busy();
        let slice = unsafe { self.get_slice() };
        buffer
            .iter_mut()
            .zip(slice.iter().skip(offset))
            .for_each(|(target, source)| *target = *source);
    }

    fn wait_if_busy(&self) {
        while self.nvm.nvm.seestat.read().busy().bit_is_set() {}
    }
}

pub trait SmartEepromPointableSize: Sealed + Copy {}

impl SmartEepromPointableSize for u8 {}
impl Sealed for u8 {}
impl SmartEepromPointableSize for u16 {}
impl Sealed for u16 {}
impl SmartEepromPointableSize for u32 {}
impl Sealed for u32 {}

impl<'a> SmartEeprom<'a, Unlocked> {
    pub unsafe fn get_mut_slice<TP: SmartEepromPointableSize>(&mut self) -> &mut [TP] {
        core::slice::from_raw_parts_mut(
            Self::SEEPROM_ADDR as _,
            self.virtual_size / core::mem::size_of::<TP>(),
        )
    }

    pub fn set<TP: SmartEepromPointableSize>(&mut self, offset: usize, buffer: &[TP]) {
        self.wait_if_busy();
        let slice = unsafe { self.get_mut_slice() };
        buffer
            .iter()
            .zip(slice.iter_mut().skip(offset))
            .for_each(|(source, target)| *target = *source);
    }

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
