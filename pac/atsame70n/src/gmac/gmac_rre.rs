#[doc = "Register `GMAC_RRE` reader"]
pub struct R(crate::R<GMAC_RRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_RRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_RRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_RRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRER` reader - Receive Resource Errors"]
pub struct RXRER_R(crate::FieldReader<u32, u32>);
impl RXRER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RXRER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:17 - Receive Resource Errors"]
    #[inline(always)]
    pub fn rxrer(&self) -> RXRER_R {
        RXRER_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "Receive Resource Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rre](index.html) module"]
pub struct GMAC_RRE_SPEC;
impl crate::RegisterSpec for GMAC_RRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_rre::R](R) reader structure"]
impl crate::Readable for GMAC_RRE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_RRE to value 0"]
impl crate::Resettable for GMAC_RRE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
