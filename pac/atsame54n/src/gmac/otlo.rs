#[doc = "Register `OTLO` reader"]
pub type R = crate::R<OTLO_SPEC>;
#[doc = "Field `TXO` reader - Transmitted Octets"]
pub type TXO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted Octets"]
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new(self.bits)
    }
}
#[doc = "Octets Transmitted \\[31:0\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otlo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTLO_SPEC;
impl crate::RegisterSpec for OTLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otlo::R`](R) reader structure"]
impl crate::Readable for OTLO_SPEC {}
#[doc = "`reset()` method sets OTLO to value 0"]
impl crate::Resettable for OTLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
