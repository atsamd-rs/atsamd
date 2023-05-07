#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READY` reader - Ready to accept a command"]
pub type READY_R = crate::BitReader<bool>;
#[doc = "Field `PRM` reader - Power Reduction Mode"]
pub type PRM_R = crate::BitReader<bool>;
#[doc = "Field `LOAD` reader - NVM Page Buffer Active Loading"]
pub type LOAD_R = crate::BitReader<bool>;
#[doc = "Field `SUSP` reader - NVM Write Or Erase Operation Is Suspended"]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `AFIRST` reader - BANKA First"]
pub type AFIRST_R = crate::BitReader<bool>;
#[doc = "Field `BPDIS` reader - Boot Loader Protection Disable"]
pub type BPDIS_R = crate::BitReader<bool>;
#[doc = "Field `BOOTPROT` reader - Boot Loader Protection Size"]
pub type BOOTPROT_R = crate::FieldReader<u8, BOOTPROTSELECT_A>;
#[doc = "Boot Loader Protection Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOTPROTSELECT_A {
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
impl From<BOOTPROTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOTPROTSELECT_A) -> Self {
        variant as _
    }
}
impl BOOTPROT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOTPROTSELECT_A {
        match self.bits {
            15 => BOOTPROTSELECT_A::_0,
            14 => BOOTPROTSELECT_A::_8,
            13 => BOOTPROTSELECT_A::_16,
            12 => BOOTPROTSELECT_A::_24,
            11 => BOOTPROTSELECT_A::_32,
            10 => BOOTPROTSELECT_A::_40,
            9 => BOOTPROTSELECT_A::_48,
            8 => BOOTPROTSELECT_A::_56,
            7 => BOOTPROTSELECT_A::_64,
            6 => BOOTPROTSELECT_A::_72,
            5 => BOOTPROTSELECT_A::_80,
            4 => BOOTPROTSELECT_A::_88,
            3 => BOOTPROTSELECT_A::_96,
            2 => BOOTPROTSELECT_A::_104,
            1 => BOOTPROTSELECT_A::_112,
            0 => BOOTPROTSELECT_A::_120,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOOTPROTSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == BOOTPROTSELECT_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == BOOTPROTSELECT_A::_16
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == BOOTPROTSELECT_A::_24
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == BOOTPROTSELECT_A::_32
    }
    #[doc = "Checks if the value of the field is `_40`"]
    #[inline(always)]
    pub fn is_40(&self) -> bool {
        *self == BOOTPROTSELECT_A::_40
    }
    #[doc = "Checks if the value of the field is `_48`"]
    #[inline(always)]
    pub fn is_48(&self) -> bool {
        *self == BOOTPROTSELECT_A::_48
    }
    #[doc = "Checks if the value of the field is `_56`"]
    #[inline(always)]
    pub fn is_56(&self) -> bool {
        *self == BOOTPROTSELECT_A::_56
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == BOOTPROTSELECT_A::_64
    }
    #[doc = "Checks if the value of the field is `_72`"]
    #[inline(always)]
    pub fn is_72(&self) -> bool {
        *self == BOOTPROTSELECT_A::_72
    }
    #[doc = "Checks if the value of the field is `_80`"]
    #[inline(always)]
    pub fn is_80(&self) -> bool {
        *self == BOOTPROTSELECT_A::_80
    }
    #[doc = "Checks if the value of the field is `_88`"]
    #[inline(always)]
    pub fn is_88(&self) -> bool {
        *self == BOOTPROTSELECT_A::_88
    }
    #[doc = "Checks if the value of the field is `_96`"]
    #[inline(always)]
    pub fn is_96(&self) -> bool {
        *self == BOOTPROTSELECT_A::_96
    }
    #[doc = "Checks if the value of the field is `_104`"]
    #[inline(always)]
    pub fn is_104(&self) -> bool {
        *self == BOOTPROTSELECT_A::_104
    }
    #[doc = "Checks if the value of the field is `_112`"]
    #[inline(always)]
    pub fn is_112(&self) -> bool {
        *self == BOOTPROTSELECT_A::_112
    }
    #[doc = "Checks if the value of the field is `_120`"]
    #[inline(always)]
    pub fn is_120(&self) -> bool {
        *self == BOOTPROTSELECT_A::_120
    }
}
impl R {
    #[doc = "Bit 0 - Ready to accept a command"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Reduction Mode"]
    #[inline(always)]
    pub fn prm(&self) -> PRM_R {
        PRM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVM Page Buffer Active Loading"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NVM Write Or Erase Operation Is Suspended"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BANKA First"]
    #[inline(always)]
    pub fn afirst(&self) -> AFIRST_R {
        AFIRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Boot Loader Protection Disable"]
    #[inline(always)]
    pub fn bpdis(&self) -> BPDIS_R {
        BPDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Boot Loader Protection Size"]
    #[inline(always)]
    pub fn bootprot(&self) -> BOOTPROT_R {
        BOOTPROT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
