#[doc = "Register `TBFR511` reader"]
pub type R = crate::R<Tbfr511Spec>;
#[doc = "Field `NFRX` reader - 256 to 511 Byte Frames Received without Error"]
pub type NfrxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 256 to 511 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NfrxR {
        NfrxR::new(self.bits)
    }
}
#[doc = "256 to 511Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbfr511::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbfr511Spec;
impl crate::RegisterSpec for Tbfr511Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbfr511::R`](R) reader structure"]
impl crate::Readable for Tbfr511Spec {}
#[doc = "`reset()` method sets TBFR511 to value 0"]
impl crate::Resettable for Tbfr511Spec {
    const RESET_VALUE: u32 = 0;
}
