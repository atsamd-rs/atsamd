#[doc = "Register `RPQ` reader"]
pub type R = crate::R<RpqSpec>;
#[doc = "Field `RPQ` reader - Received Pause Quantum"]
pub type RpqR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Received Pause Quantum"]
    #[inline(always)]
    pub fn rpq(&self) -> RpqR {
        RpqR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Received Pause Quantum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpq::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RpqSpec;
impl crate::RegisterSpec for RpqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpq::R`](R) reader structure"]
impl crate::Readable for RpqSpec {}
#[doc = "`reset()` method sets RPQ to value 0"]
impl crate::Resettable for RpqSpec {}
