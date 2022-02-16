#[doc = "Register `GMAC_OFR` reader"]
pub struct R(crate::R<GMAC_OFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_OFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_OFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_OFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OFRX` reader - Oversized Frames Received"]
pub struct OFRX_R(crate::FieldReader<u16, u16>);
impl OFRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        OFRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFRX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Oversized Frames Received"]
    #[inline(always)]
    pub fn ofrx(&self) -> OFRX_R {
        OFRX_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Oversize Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ofr](index.html) module"]
pub struct GMAC_OFR_SPEC;
impl crate::RegisterSpec for GMAC_OFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_ofr::R](R) reader structure"]
impl crate::Readable for GMAC_OFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_OFR to value 0"]
impl crate::Resettable for GMAC_OFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
