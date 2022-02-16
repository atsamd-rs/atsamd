#[doc = "Register `ICM_SR` reader"]
pub struct R(crate::R<ICM_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICM_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICM_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICM_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENABLE` reader - ICM Controller Enable Register"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWRMDIS` reader - Region Monitoring Disabled Raw Status"]
pub struct RAWRMDIS_R(crate::FieldReader<u8, u8>);
impl RAWRMDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAWRMDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWRMDIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMDIS` reader - Region Monitoring Disabled Status"]
pub struct RMDIS_R(crate::FieldReader<u8, u8>);
impl RMDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RMDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMDIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - ICM Controller Enable Register"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Region Monitoring Disabled Raw Status"]
    #[inline(always)]
    pub fn rawrmdis(&self) -> RAWRMDIS_R {
        RAWRMDIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region Monitoring Disabled Status"]
    #[inline(always)]
    pub fn rmdis(&self) -> RMDIS_R {
        RMDIS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_sr](index.html) module"]
pub struct ICM_SR_SPEC;
impl crate::RegisterSpec for ICM_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icm_sr::R](R) reader structure"]
impl crate::Readable for ICM_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ICM_SR to value 0"]
impl crate::Resettable for ICM_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
