#[doc = "Register `GMAC_TUR` reader"]
pub struct R(crate::R<GMAC_TUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_TUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_TUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_TUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXUNR` reader - Transmit Underruns"]
pub struct TXUNR_R(crate::FieldReader<u16, u16>);
impl TXUNR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TXUNR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Transmit Underruns"]
    #[inline(always)]
    pub fn txunr(&self) -> TXUNR_R {
        TXUNR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Transmit Underruns Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_tur](index.html) module"]
pub struct GMAC_TUR_SPEC;
impl crate::RegisterSpec for GMAC_TUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_tur::R](R) reader structure"]
impl crate::Readable for GMAC_TUR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_TUR to value 0"]
impl crate::Resettable for GMAC_TUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
