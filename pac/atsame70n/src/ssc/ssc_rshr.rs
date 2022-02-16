#[doc = "Register `SSC_RSHR` reader"]
pub struct R(crate::R<SSC_RSHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSC_RSHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSC_RSHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSC_RSHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSDAT` reader - Receive Synchronization Data"]
pub struct RSDAT_R(crate::FieldReader<u16, u16>);
impl RSDAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RSDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSDAT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Receive Synchronization Data"]
    #[inline(always)]
    pub fn rsdat(&self) -> RSDAT_R {
        RSDAT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive Sync. Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_rshr](index.html) module"]
pub struct SSC_RSHR_SPEC;
impl crate::RegisterSpec for SSC_RSHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssc_rshr::R](R) reader structure"]
impl crate::Readable for SSC_RSHR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSC_RSHR to value 0"]
impl crate::Resettable for SSC_RSHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
