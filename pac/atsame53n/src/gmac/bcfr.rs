#[doc = "Register `BCFR` reader"]
pub struct R(crate::R<BCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BFRX` reader - Broadcast Frames Received without Error"]
pub type BFRX_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Broadcast Frames Received without Error"]
    #[inline(always)]
    pub fn bfrx(&self) -> BFRX_R {
        BFRX_R::new(self.bits)
    }
}
#[doc = "Broadcast Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcfr](index.html) module"]
pub struct BCFR_SPEC;
impl crate::RegisterSpec for BCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcfr::R](R) reader structure"]
impl crate::Readable for BCFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BCFR to value 0"]
impl crate::Resettable for BCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
