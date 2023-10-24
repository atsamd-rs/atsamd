#[doc = "Register `ORHI` reader"]
pub type R = crate::R<ORHI_SPEC>;
#[doc = "Field `RXO` reader - Received Octets"]
pub type RXO_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Received Octets"]
    #[inline(always)]
    pub fn rxo(&self) -> RXO_R {
        RXO_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Octets Received \\[47:32\\]
Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`orhi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ORHI_SPEC;
impl crate::RegisterSpec for ORHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`orhi::R`](R) reader structure"]
impl crate::Readable for ORHI_SPEC {}
#[doc = "`reset()` method sets ORHI to value 0"]
impl crate::Resettable for ORHI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
