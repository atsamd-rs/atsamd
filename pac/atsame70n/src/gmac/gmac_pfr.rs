#[doc = "Register `GMAC_PFR` reader"]
pub struct R(crate::R<GMAC_PFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_PFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_PFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_PFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PFRX` reader - Pause Frames Received Register"]
pub struct PFRX_R(crate::FieldReader<u16, u16>);
impl PFRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PFRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PFRX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Pause Frames Received Register"]
    #[inline(always)]
    pub fn pfrx(&self) -> PFRX_R {
        PFRX_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Pause Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_pfr](index.html) module"]
pub struct GMAC_PFR_SPEC;
impl crate::RegisterSpec for GMAC_PFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_pfr::R](R) reader structure"]
impl crate::Readable for GMAC_PFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_PFR to value 0"]
impl crate::Resettable for GMAC_PFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
