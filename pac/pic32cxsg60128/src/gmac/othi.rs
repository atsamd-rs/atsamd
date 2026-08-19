#[doc = "Register `OTHI` reader"]
pub type R = crate::R<OthiSpec>;
#[doc = "Field `TXO` reader - Transmitted Octets"]
pub type TxoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Transmitted Octets"]
    #[inline(always)]
    pub fn txo(&self) -> TxoR {
        TxoR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Octets Transmitted \\[47:32\\]
Register\n\nYou can [`read`](crate::Reg::read) this register and get [`othi::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OthiSpec;
impl crate::RegisterSpec for OthiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`othi::R`](R) reader structure"]
impl crate::Readable for OthiSpec {}
#[doc = "`reset()` method sets OTHI to value 0"]
impl crate::Resettable for OthiSpec {
    const RESET_VALUE: u32 = 0;
}
