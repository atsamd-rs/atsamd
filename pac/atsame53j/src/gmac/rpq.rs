#[doc = "Register `RPQ` reader"]
pub struct R(crate::R<RPQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPQ` reader - Received Pause Quantum"]
pub type RPQ_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Received Pause Quantum"]
    #[inline(always)]
    pub fn rpq(&self) -> RPQ_R {
        RPQ_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Received Pause Quantum Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpq](index.html) module"]
pub struct RPQ_SPEC;
impl crate::RegisterSpec for RPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpq::R](R) reader structure"]
impl crate::Readable for RPQ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RPQ to value 0"]
impl crate::Resettable for RPQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
