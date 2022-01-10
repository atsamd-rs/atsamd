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
pub struct DEVSEL_R(crate::FieldReader<u8, u8>);
impl DEVSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEVSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVISION` reader - Revision Number"]
pub struct REVISION_R(crate::FieldReader<u8, u8>);
impl REVISION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIE` reader - Die Number"]
pub struct DIE_R(crate::FieldReader<u8, u8>);
impl DIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Field `SERIES` reader - Series"]
pub struct SERIES_R(crate::FieldReader<u8, SERIES_A>);
impl SERIES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SERIES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SERIES_A> {
        match self.bits {
            0 => Some(SERIES_A::_0),
            1 => Some(SERIES_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SERIES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SERIES_A::_1
    }
}
impl core::ops::Deref for SERIES_R {
    type Target = crate::FieldReader<u8, SERIES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `FAMILY` reader - Family"]
pub struct FAMILY_R(crate::FieldReader<u8, FAMILY_A>);
impl FAMILY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FAMILY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FAMILY_A> {
        match self.bits {
            0 => Some(FAMILY_A::_0),
            1 => Some(FAMILY_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FAMILY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FAMILY_A::_1
    }
}
impl core::ops::Deref for FAMILY_R {
    type Target = crate::FieldReader<u8, FAMILY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `PROCESSOR` reader - Processor"]
pub struct PROCESSOR_R(crate::FieldReader<u8, PROCESSOR_A>);
impl PROCESSOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PROCESSOR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PROCESSOR_A> {
        match self.bits {
            0 => Some(PROCESSOR_A::_0),
            1 => Some(PROCESSOR_A::_1),
            2 => Some(PROCESSOR_A::_2),
            3 => Some(PROCESSOR_A::_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PROCESSOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PROCESSOR_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == PROCESSOR_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == PROCESSOR_A::_3
    }
}
impl core::ops::Deref for PROCESSOR_R {
    type Target = crate::FieldReader<u8, PROCESSOR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "`reset()` method sets DID to value 0x1003_0109"]
impl crate::Resettable for DID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1003_0109
    }
}
