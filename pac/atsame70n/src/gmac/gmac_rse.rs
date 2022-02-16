#[doc = "Register `GMAC_RSE` reader"]
pub struct R(crate::R<GMAC_RSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_RSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_RSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_RSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXSE` reader - Receive Symbol Errors"]
pub struct RXSE_R(crate::FieldReader<u16, u16>);
impl RXSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RXSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Receive Symbol Errors"]
    #[inline(always)]
    pub fn rxse(&self) -> RXSE_R {
        RXSE_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Receive Symbol Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rse](index.html) module"]
pub struct GMAC_RSE_SPEC;
impl crate::RegisterSpec for GMAC_RSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_rse::R](R) reader structure"]
impl crate::Readable for GMAC_RSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_RSE to value 0"]
impl crate::Resettable for GMAC_RSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
