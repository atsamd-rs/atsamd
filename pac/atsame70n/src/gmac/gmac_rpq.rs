#[doc = "Register `GMAC_RPQ` reader"]
pub struct R(crate::R<GMAC_RPQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_RPQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_RPQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_RPQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPQ` reader - Received Pause Quantum"]
pub struct RPQ_R(crate::FieldReader<u16, u16>);
impl RPQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RPQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPQ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Received Pause Quantum"]
    #[inline(always)]
    pub fn rpq(&self) -> RPQ_R {
        RPQ_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Received Pause Quantum Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_rpq](index.html) module"]
pub struct GMAC_RPQ_SPEC;
impl crate::RegisterSpec for GMAC_RPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_rpq::R](R) reader structure"]
impl crate::Readable for GMAC_RPQ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_RPQ to value 0"]
impl crate::Resettable for GMAC_RPQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
