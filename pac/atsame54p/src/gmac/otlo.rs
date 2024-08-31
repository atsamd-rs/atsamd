#[doc = "Register `OTLO` reader"]
pub type R = crate::R<OtloSpec>;
#[doc = "Field `TXO` reader - Transmitted Octets"]
pub type TxoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted Octets"]
    #[inline(always)]
    pub fn txo(&self) -> TxoR {
        TxoR::new(self.bits)
    }
}
#[doc = "Octets Transmitted \\[31:0\\]
Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otlo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtloSpec;
impl crate::RegisterSpec for OtloSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otlo::R`](R) reader structure"]
impl crate::Readable for OtloSpec {}
#[doc = "`reset()` method sets OTLO to value 0"]
impl crate::Resettable for OtloSpec {
    const RESET_VALUE: u32 = 0;
}
