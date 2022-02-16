#[doc = "Register `GMAC_OTLO` reader"]
pub struct R(crate::R<GMAC_OTLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_OTLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_OTLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_OTLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXO` reader - Transmitted Octets"]
pub struct TXO_R(crate::FieldReader<u32, u32>);
impl TXO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TXO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmitted Octets"]
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new(self.bits as u32)
    }
}
#[doc = "Octets Transmitted Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_otlo](index.html) module"]
pub struct GMAC_OTLO_SPEC;
impl crate::RegisterSpec for GMAC_OTLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_otlo::R](R) reader structure"]
impl crate::Readable for GMAC_OTLO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_OTLO to value 0"]
impl crate::Resettable for GMAC_OTLO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
