#[doc = "Register `SSC_RHR` reader"]
pub struct R(crate::R<SSC_RHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSC_RHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSC_RHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSC_RHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDAT` reader - Receive Data"]
pub struct RDAT_R(crate::FieldReader<u32, u32>);
impl RDAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDAT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive Data"]
    #[inline(always)]
    pub fn rdat(&self) -> RDAT_R {
        RDAT_R::new(self.bits as u32)
    }
}
#[doc = "Receive Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_rhr](index.html) module"]
pub struct SSC_RHR_SPEC;
impl crate::RegisterSpec for SSC_RHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssc_rhr::R](R) reader structure"]
impl crate::Readable for SSC_RHR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSC_RHR to value 0"]
impl crate::Resettable for SSC_RHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
