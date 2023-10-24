#[doc = "Register `RPQ` reader"]
pub type R = crate::R<RPQ_SPEC>;
#[doc = "Field `RPQ` reader - Received Pause Quantum"]
pub type RPQ_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Received Pause Quantum"]
    #[inline(always)]
    pub fn rpq(&self) -> RPQ_R {
        RPQ_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Received Pause Quantum Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpq::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPQ_SPEC;
impl crate::RegisterSpec for RPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpq::R`](R) reader structure"]
impl crate::Readable for RPQ_SPEC {}
#[doc = "`reset()` method sets RPQ to value 0"]
impl crate::Resettable for RPQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
