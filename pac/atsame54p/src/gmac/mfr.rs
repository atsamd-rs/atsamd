#[doc = "Register `MFR` reader"]
pub struct R(crate::R<MFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MFRX` reader - Multicast Frames Received without Error"]
pub type MFRX_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Multicast Frames Received without Error"]
    #[inline(always)]
    pub fn mfrx(&self) -> MFRX_R {
        MFRX_R::new(self.bits)
    }
}
#[doc = "Multicast Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfr](index.html) module"]
pub struct MFR_SPEC;
impl crate::RegisterSpec for MFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mfr::R](R) reader structure"]
impl crate::Readable for MFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MFR to value 0"]
impl crate::Resettable for MFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
