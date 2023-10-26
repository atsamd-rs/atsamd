#[doc = "Register `DID` reader"]
pub type R = crate::R<DID_SPEC>;
#[doc = "Field `DEVSEL` reader - Device Select"]
pub type DEVSEL_R = crate::FieldReader;
#[doc = "Field `REVISION` reader - Revision Number"]
pub type REVISION_R = crate::FieldReader;
#[doc = "Field `DIE` reader - Die Number"]
pub type DIE_R = crate::FieldReader;
#[doc = "Field `SERIES` reader - Series"]
pub type SERIES_R = crate::FieldReader<SERIESSELECT_A>;
#[doc = "Series\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SERIESSELECT_A {
    #[doc = "0: Cortex-M0+ processor, basic feature set"]
    _0 = 0,
    #[doc = "1: Cortex-M0+ processor, USB"]
    _1 = 1,
}
impl From<SERIESSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SERIESSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SERIESSELECT_A {
    type Ux = u8;
}
impl SERIES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SERIESSELECT_A> {
        match self.bits {
            0 => Some(SERIESSELECT_A::_0),
            1 => Some(SERIESSELECT_A::_1),
            _ => None,
        }
    }
    #[doc = "Cortex-M0+ processor, basic feature set"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SERIESSELECT_A::_0
    }
    #[doc = "Cortex-M0+ processor, USB"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SERIESSELECT_A::_1
    }
}
#[doc = "Field `FAMILY` reader - Family"]
pub type FAMILY_R = crate::FieldReader<FAMILYSELECT_A>;
#[doc = "Family\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FAMILYSELECT_A {
    #[doc = "0: General purpose microcontroller"]
    _0 = 0,
    #[doc = "1: PicoPower"]
    _1 = 1,
}
impl From<FAMILYSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FAMILYSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FAMILYSELECT_A {
    type Ux = u8;
}
impl FAMILY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FAMILYSELECT_A> {
        match self.bits {
            0 => Some(FAMILYSELECT_A::_0),
            1 => Some(FAMILYSELECT_A::_1),
            _ => None,
        }
    }
    #[doc = "General purpose microcontroller"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAMILYSELECT_A::_0
    }
    #[doc = "PicoPower"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAMILYSELECT_A::_1
    }
}
#[doc = "Field `PROCESSOR` reader - Processor"]
pub type PROCESSOR_R = crate::FieldReader<PROCESSORSELECT_A>;
#[doc = "Processor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PROCESSORSELECT_A {
    #[doc = "0: Cortex-M0"]
    _0 = 0,
    #[doc = "1: Cortex-M0+"]
    _1 = 1,
    #[doc = "2: Cortex-M3"]
    _2 = 2,
    #[doc = "3: Cortex-M4"]
    _3 = 3,
}
impl From<PROCESSORSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PROCESSORSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PROCESSORSELECT_A {
    type Ux = u8;
}
impl PROCESSOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PROCESSORSELECT_A> {
        match self.bits {
            0 => Some(PROCESSORSELECT_A::_0),
            1 => Some(PROCESSORSELECT_A::_1),
            2 => Some(PROCESSORSELECT_A::_2),
            3 => Some(PROCESSORSELECT_A::_3),
            _ => None,
        }
    }
    #[doc = "Cortex-M0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PROCESSORSELECT_A::_0
    }
    #[doc = "Cortex-M0+"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PROCESSORSELECT_A::_1
    }
    #[doc = "Cortex-M3"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == PROCESSORSELECT_A::_2
    }
    #[doc = "Cortex-M4"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == PROCESSORSELECT_A::_3
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
#[doc = "Device Identification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`did::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DID_SPEC;
impl crate::RegisterSpec for DID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`did::R`](R) reader structure"]
impl crate::Readable for DID_SPEC {}
#[doc = "`reset()` method sets DID to value 0x1003_0109"]
impl crate::Resettable for DID_SPEC {
    const RESET_VALUE: Self::Ux = 0x1003_0109;
}
