#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `READY` reader - Ready to accept a command"]
pub type ReadyR = crate::BitReader;
#[doc = "Field `PRM` reader - Power Reduction Mode"]
pub type PrmR = crate::BitReader;
#[doc = "Field `LOAD` reader - NVM Page Buffer Active Loading"]
pub type LoadR = crate::BitReader;
#[doc = "Field `SUSP` reader - NVM Write Or Erase Operation Is Suspended"]
pub type SuspR = crate::BitReader;
#[doc = "Field `AFIRST` reader - BANKA First"]
pub type AfirstR = crate::BitReader;
#[doc = "Field `BPDIS` reader - Boot Loader Protection Disable"]
pub type BpdisR = crate::BitReader;
#[doc = "Boot Loader Protection Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bootprotselect {
    #[doc = "15: 0 kbytes"]
    _0 = 15,
    #[doc = "14: 8 kbytes"]
    _8 = 14,
    #[doc = "13: 16 kbytes"]
    _16 = 13,
    #[doc = "12: 24 kbytes"]
    _24 = 12,
    #[doc = "11: 32 kbytes"]
    _32 = 11,
    #[doc = "10: 40 kbytes"]
    _40 = 10,
    #[doc = "9: 48 kbytes"]
    _48 = 9,
    #[doc = "8: 56 kbytes"]
    _56 = 8,
    #[doc = "7: 64 kbytes"]
    _64 = 7,
    #[doc = "6: 72 kbytes"]
    _72 = 6,
    #[doc = "5: 80 kbytes"]
    _80 = 5,
    #[doc = "4: 88 kbytes"]
    _88 = 4,
    #[doc = "3: 96 kbytes"]
    _96 = 3,
    #[doc = "2: 104 kbytes"]
    _104 = 2,
    #[doc = "1: 112 kbytes"]
    _112 = 1,
    #[doc = "0: 120 kbytes"]
    _120 = 0,
}
impl From<Bootprotselect> for u8 {
    #[inline(always)]
    fn from(variant: Bootprotselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bootprotselect {
    type Ux = u8;
}
impl crate::IsEnum for Bootprotselect {}
#[doc = "Field `BOOTPROT` reader - Boot Loader Protection Size"]
pub type BootprotR = crate::FieldReader<Bootprotselect>;
impl BootprotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootprotselect {
        match self.bits {
            15 => Bootprotselect::_0,
            14 => Bootprotselect::_8,
            13 => Bootprotselect::_16,
            12 => Bootprotselect::_24,
            11 => Bootprotselect::_32,
            10 => Bootprotselect::_40,
            9 => Bootprotselect::_48,
            8 => Bootprotselect::_56,
            7 => Bootprotselect::_64,
            6 => Bootprotselect::_72,
            5 => Bootprotselect::_80,
            4 => Bootprotselect::_88,
            3 => Bootprotselect::_96,
            2 => Bootprotselect::_104,
            1 => Bootprotselect::_112,
            0 => Bootprotselect::_120,
            _ => unreachable!(),
        }
    }
    #[doc = "0 kbytes"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bootprotselect::_0
    }
    #[doc = "8 kbytes"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Bootprotselect::_8
    }
    #[doc = "16 kbytes"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Bootprotselect::_16
    }
    #[doc = "24 kbytes"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == Bootprotselect::_24
    }
    #[doc = "32 kbytes"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Bootprotselect::_32
    }
    #[doc = "40 kbytes"]
    #[inline(always)]
    pub fn is_40(&self) -> bool {
        *self == Bootprotselect::_40
    }
    #[doc = "48 kbytes"]
    #[inline(always)]
    pub fn is_48(&self) -> bool {
        *self == Bootprotselect::_48
    }
    #[doc = "56 kbytes"]
    #[inline(always)]
    pub fn is_56(&self) -> bool {
        *self == Bootprotselect::_56
    }
    #[doc = "64 kbytes"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Bootprotselect::_64
    }
    #[doc = "72 kbytes"]
    #[inline(always)]
    pub fn is_72(&self) -> bool {
        *self == Bootprotselect::_72
    }
    #[doc = "80 kbytes"]
    #[inline(always)]
    pub fn is_80(&self) -> bool {
        *self == Bootprotselect::_80
    }
    #[doc = "88 kbytes"]
    #[inline(always)]
    pub fn is_88(&self) -> bool {
        *self == Bootprotselect::_88
    }
    #[doc = "96 kbytes"]
    #[inline(always)]
    pub fn is_96(&self) -> bool {
        *self == Bootprotselect::_96
    }
    #[doc = "104 kbytes"]
    #[inline(always)]
    pub fn is_104(&self) -> bool {
        *self == Bootprotselect::_104
    }
    #[doc = "112 kbytes"]
    #[inline(always)]
    pub fn is_112(&self) -> bool {
        *self == Bootprotselect::_112
    }
    #[doc = "120 kbytes"]
    #[inline(always)]
    pub fn is_120(&self) -> bool {
        *self == Bootprotselect::_120
    }
}
impl R {
    #[doc = "Bit 0 - Ready to accept a command"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Reduction Mode"]
    #[inline(always)]
    pub fn prm(&self) -> PrmR {
        PrmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVM Page Buffer Active Loading"]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NVM Write Or Erase Operation Is Suspended"]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BANKA First"]
    #[inline(always)]
    pub fn afirst(&self) -> AfirstR {
        AfirstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Boot Loader Protection Disable"]
    #[inline(always)]
    pub fn bpdis(&self) -> BpdisR {
        BpdisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Boot Loader Protection Size"]
    #[inline(always)]
    pub fn bootprot(&self) -> BootprotR {
        BootprotR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u16 = 0;
}
