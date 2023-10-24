#[doc = "Register `OTHI` reader"]
pub type R = crate::R<OTHI_SPEC>;
#[doc = "Field `TXO` reader - Transmitted Octets"]
pub type TXO_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Transmitted Octets"]
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Octets Transmitted \\[47:32\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`othi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTHI_SPEC;
impl crate::RegisterSpec for OTHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`othi::R`](R) reader structure"]
impl crate::Readable for OTHI_SPEC {}
#[doc = "`reset()` method sets OTHI to value 0"]
impl crate::Resettable for OTHI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
