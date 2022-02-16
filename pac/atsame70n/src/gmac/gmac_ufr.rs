#[doc = "Register `GMAC_UFR` reader"]
pub struct R(crate::R<GMAC_UFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_UFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_UFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_UFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UFRX` reader - Undersize Frames Received"]
pub struct UFRX_R(crate::FieldReader<u16, u16>);
impl UFRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        UFRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UFRX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Undersize Frames Received"]
    #[inline(always)]
    pub fn ufrx(&self) -> UFRX_R {
        UFRX_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Undersize Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ufr](index.html) module"]
pub struct GMAC_UFR_SPEC;
impl crate::RegisterSpec for GMAC_UFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_ufr::R](R) reader structure"]
impl crate::Readable for GMAC_UFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_UFR to value 0"]
impl crate::Resettable for GMAC_UFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
