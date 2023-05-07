#[doc = "Register `DID` reader"]
pub struct R(crate::R<DID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEVSEL` reader - Device Select"]
pub type DEVSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REVISION` reader - Revision Number"]
pub type REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIE` reader - Die Number"]
pub type DIE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SERIES` reader - Series"]
pub type SERIES_R = crate::FieldReader<u8, SERIESSELECT_A>;
#[doc = "Series\n\nValue on reset: 1"]
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
impl SERIES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SERIESSELECT_A> {
        match self.bits {
            0 => Some(SERIESSELECT_A::_0),
            1 => Some(SERIESSELECT_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SERIESSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SERIESSELECT_A::_1
    }
}
#[doc = "Field `FAMILY` reader - Family"]
pub type FAMILY_R = crate::FieldReader<u8, FAMILYSELECT_A>;
#[doc = "Family\n\nValue on reset: 3"]
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
impl FAMILY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FAMILYSELECT_A> {
        match self.bits {
            0 => Some(FAMILYSELECT_A::_0),
            1 => Some(FAMILYSELECT_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAMILYSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAMILYSELECT_A::_1
    }
}
#[doc = "Field `PROCESSOR` reader - Processor"]
pub type PROCESSOR_R = crate::FieldReader<u8, PROCESSORSELECT_A>;
#[doc = "Processor\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PROCESSORSELECT_A {
    #[doc = "1: Cortex-M0+"]
    CM0P = 1,
    #[doc = "2: Cortex-M23"]
    CM23 = 2,
    #[doc = "3: Cortex-M3"]
    CM3 = 3,
    #[doc = "5: Cortex-M4"]
    CM4 = 5,
    #[doc = "6: Cortex-M4 with FPU"]
    CM4F = 6,
    #[doc = "7: Cortex-M33"]
    CM33 = 7,
}
impl From<PROCESSORSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PROCESSORSELECT_A) -> Self {
        variant as _
    }
}
impl PROCESSOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PROCESSORSELECT_A> {
        match self.bits {
            1 => Some(PROCESSORSELECT_A::CM0P),
            2 => Some(PROCESSORSELECT_A::CM23),
            3 => Some(PROCESSORSELECT_A::CM3),
            5 => Some(PROCESSORSELECT_A::CM4),
            6 => Some(PROCESSORSELECT_A::CM4F),
            7 => Some(PROCESSORSELECT_A::CM33),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CM0P`"]
    #[inline(always)]
    pub fn is_cm0p(&self) -> bool {
        *self == PROCESSORSELECT_A::CM0P
    }
    #[doc = "Checks if the value of the field is `CM23`"]
    #[inline(always)]
    pub fn is_cm23(&self) -> bool {
        *self == PROCESSORSELECT_A::CM23
    }
    #[doc = "Checks if the value of the field is `CM3`"]
    #[inline(always)]
    pub fn is_cm3(&self) -> bool {
        *self == PROCESSORSELECT_A::CM3
    }
    #[doc = "Checks if the value of the field is `CM4`"]
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        *self == PROCESSORSELECT_A::CM4
    }
    #[doc = "Checks if the value of the field is `CM4F`"]
    #[inline(always)]
    pub fn is_cm4f(&self) -> bool {
        *self == PROCESSORSELECT_A::CM4F
    }
    #[doc = "Checks if the value of the field is `CM33`"]
    #[inline(always)]
    pub fn is_cm33(&self) -> bool {
        *self == PROCESSORSELECT_A::CM33
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
#[doc = "Device Identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [did](index.html) module"]
pub struct DID_SPEC;
impl crate::RegisterSpec for DID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [did::R](R) reader structure"]
impl crate::Readable for DID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DID to value 0x6181_0301"]
impl crate::Resettable for DID_SPEC {
    const RESET_VALUE: Self::Ux = 0x6181_0301;
}
