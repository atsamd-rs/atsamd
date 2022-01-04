#[doc = "Register `RHR` reader"]
pub struct R(crate::R<RHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA` reader - Reception Data"]
pub struct RDATA_R(crate::FieldReader<u32, u32>);
impl RDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Reception Data"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(self.bits as u32)
    }
}
#[doc = "Reception Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rhr](index.html) module"]
pub struct RHR_SPEC;
impl crate::RegisterSpec for RHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rhr::R](R) reader structure"]
impl crate::Readable for RHR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RHR to value 0"]
impl crate::Resettable for RHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
