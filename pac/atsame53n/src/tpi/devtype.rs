#[doc = "Register `DEVTYPE` reader"]
pub struct R(crate::R<DEVTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SubType` reader - "]
pub struct SUBTYPE_R(crate::FieldReader<u8, u8>);
impl SUBTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SUBTYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUBTYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MajorType` reader - "]
pub struct MAJORTYPE_R(crate::FieldReader<u8, u8>);
impl MAJORTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAJORTYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAJORTYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sub_type(&self) -> SUBTYPE_R {
        SUBTYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn major_type(&self) -> MAJORTYPE_R {
        MAJORTYPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "TPIU_DEVTYPE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devtype](index.html) module"]
pub struct DEVTYPE_SPEC;
impl crate::RegisterSpec for DEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devtype::R](R) reader structure"]
impl crate::Readable for DEVTYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVTYPE to value 0"]
impl crate::Resettable for DEVTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
