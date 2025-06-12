#[doc = "Register `TBFR127` reader"]
pub type R = crate::R<Tbfr127Spec>;
#[doc = "Field `NFRX` reader - 65 to 127 Byte Frames Received without Error"]
pub type NfrxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 65 to 127 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NfrxR {
        NfrxR::new(self.bits)
    }
}
#[doc = "65 to 127 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbfr127::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbfr127Spec;
impl crate::RegisterSpec for Tbfr127Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbfr127::R`](R) reader structure"]
impl crate::Readable for Tbfr127Spec {}
#[doc = "`reset()` method sets TBFR127 to value 0"]
impl crate::Resettable for Tbfr127Spec {}
