#[doc = "Register `BFR64` reader"]
pub type R = crate::R<Bfr64Spec>;
#[doc = "Field `NFRX` reader - 64 Byte Frames Received without Error"]
pub type NfrxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 64 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NfrxR {
        NfrxR::new(self.bits)
    }
}
#[doc = "64 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bfr64::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bfr64Spec;
impl crate::RegisterSpec for Bfr64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bfr64::R`](R) reader structure"]
impl crate::Readable for Bfr64Spec {}
#[doc = "`reset()` method sets BFR64 to value 0"]
impl crate::Resettable for Bfr64Spec {}
