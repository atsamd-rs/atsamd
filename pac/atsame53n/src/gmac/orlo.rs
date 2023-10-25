#[doc = "Register `ORLO` reader"]
pub type R = crate::R<ORLO_SPEC>;
#[doc = "Field `RXO` reader - Received Octets"]
pub type RXO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received Octets"]
    #[inline(always)]
    pub fn rxo(&self) -> RXO_R {
        RXO_R::new(self.bits)
    }
}
#[doc = "Octets Received \\[31:0\\]
Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`orlo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ORLO_SPEC;
impl crate::RegisterSpec for ORLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`orlo::R`](R) reader structure"]
impl crate::Readable for ORLO_SPEC {}
#[doc = "`reset()` method sets ORLO to value 0"]
impl crate::Resettable for ORLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
