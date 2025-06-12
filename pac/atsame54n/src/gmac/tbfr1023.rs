#[doc = "Register `TBFR1023` reader"]
pub type R = crate::R<Tbfr1023Spec>;
#[doc = "Field `NFRX` reader - 512 to 1023 Byte Frames Received without Error"]
pub type NfrxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 512 to 1023 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NfrxR {
        NfrxR::new(self.bits)
    }
}
#[doc = "512 to 1023 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbfr1023::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbfr1023Spec;
impl crate::RegisterSpec for Tbfr1023Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbfr1023::R`](R) reader structure"]
impl crate::Readable for Tbfr1023Spec {}
#[doc = "`reset()` method sets TBFR1023 to value 0"]
impl crate::Resettable for Tbfr1023Spec {}
