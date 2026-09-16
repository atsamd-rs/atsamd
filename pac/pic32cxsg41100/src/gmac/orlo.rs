#[doc = "Register `ORLO` reader"]
pub type R = crate::R<OrloSpec>;
#[doc = "Field `RXO` reader - Received Octets"]
pub type RxoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received Octets"]
    #[inline(always)]
    pub fn rxo(&self) -> RxoR {
        RxoR::new(self.bits)
    }
}
#[doc = "Octets Received \\[31:0\\]
Received\n\nYou can [`read`](crate::Reg::read) this register and get [`orlo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OrloSpec;
impl crate::RegisterSpec for OrloSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`orlo::R`](R) reader structure"]
impl crate::Readable for OrloSpec {}
#[doc = "`reset()` method sets ORLO to value 0"]
impl crate::Resettable for OrloSpec {
    const RESET_VALUE: u32 = 0;
}
