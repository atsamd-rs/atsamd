#[doc = "Reader of register DID"]
pub type R = crate::R<u32, super::DID>;
#[doc = "Reader of field `DEVSEL`"]
pub type DEVSEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `DIE`"]
pub type DIE_R = crate::R<u8, u8>;
#[doc = "Series\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SERIES_A {
    #[doc = "0: Cortex-M0+ processor, basic feature set"]
    _0 = 0,
    #[doc = "1: Cortex-M0+ processor, USB"]
    _1 = 1,
}
impl From<SERIES_A> for u8 {
    #[inline(always)]
    fn from(variant: SERIES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SERIES`"]
pub type SERIES_R = crate::R<u8, SERIES_A>;
impl SERIES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SERIES_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SERIES_A::_0),
            1 => Val(SERIES_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SERIES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SERIES_A::_1
    }
}
#[doc = "Family\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FAMILY_A {
    #[doc = "0: General purpose microcontroller"]
    _0 = 0,
    #[doc = "1: PicoPower"]
    _1 = 1,
}
impl From<FAMILY_A> for u8 {
    #[inline(always)]
    fn from(variant: FAMILY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FAMILY`"]
pub type FAMILY_R = crate::R<u8, FAMILY_A>;
impl FAMILY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FAMILY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FAMILY_A::_0),
            1 => Val(FAMILY_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAMILY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAMILY_A::_1
    }
}
#[doc = "Processor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PROCESSOR_A {
    #[doc = "0: Cortex-M0"]
    _0 = 0,
    #[doc = "1: Cortex-M0+"]
    _1 = 1,
    #[doc = "2: Cortex-M3"]
    _2 = 2,
    #[doc = "3: Cortex-M4"]
    _3 = 3,
}
impl From<PROCESSOR_A> for u8 {
    #[inline(always)]
    fn from(variant: PROCESSOR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PROCESSOR`"]
pub type PROCESSOR_R = crate::R<u8, PROCESSOR_A>;
impl PROCESSOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PROCESSOR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PROCESSOR_A::_0),
            1 => Val(PROCESSOR_A::_1),
            2 => Val(PROCESSOR_A::_2),
            3 => Val(PROCESSOR_A::_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PROCESSOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PROCESSOR_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == PROCESSOR_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == PROCESSOR_A::_3
    }
}
impl R {
    #[doc = "Bits 0:7 - Device Select"]
    #[inline(always)]
    pub fn devsel(&self) -> DEVSEL_R {
        DEVSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Revision Number"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Die Number"]
    #[inline(always)]
    pub fn die(&self) -> DIE_R {
        DIE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Series"]
    #[inline(always)]
    pub fn series(&self) -> SERIES_R {
        SERIES_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 23:27 - Family"]
    #[inline(always)]
    pub fn family(&self) -> FAMILY_R {
        FAMILY_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31 - Processor"]
    #[inline(always)]
    pub fn processor(&self) -> PROCESSOR_R {
        PROCESSOR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
