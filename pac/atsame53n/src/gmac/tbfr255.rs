#[doc = "Register `TBFR255` reader"]
pub type R = crate::R<Tbfr255Spec>;
#[doc = "Field `NFRX` reader - 128 to 255 Byte Frames Received without Error"]
pub type NfrxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 128 to 255 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NfrxR {
        NfrxR::new(self.bits)
    }
}
#[doc = "128 to 255 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbfr255::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbfr255Spec;
impl crate::RegisterSpec for Tbfr255Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbfr255::R`](R) reader structure"]
impl crate::Readable for Tbfr255Spec {}
#[doc = "`reset()` method sets TBFR255 to value 0"]
impl crate::Resettable for Tbfr255Spec {}
