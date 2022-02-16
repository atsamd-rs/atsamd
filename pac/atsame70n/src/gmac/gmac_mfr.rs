#[doc = "Register `GMAC_MFR` reader"]
pub struct R(crate::R<GMAC_MFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_MFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_MFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_MFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MFRX` reader - Multicast Frames Received without Error"]
pub struct MFRX_R(crate::FieldReader<u32, u32>);
impl MFRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MFRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFRX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Multicast Frames Received without Error"]
    #[inline(always)]
    pub fn mfrx(&self) -> MFRX_R {
        MFRX_R::new(self.bits as u32)
    }
}
#[doc = "Multicast Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_mfr](index.html) module"]
pub struct GMAC_MFR_SPEC;
impl crate::RegisterSpec for GMAC_MFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_mfr::R](R) reader structure"]
impl crate::Readable for GMAC_MFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_MFR to value 0"]
impl crate::Resettable for GMAC_MFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
