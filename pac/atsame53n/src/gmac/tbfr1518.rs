#[doc = "Register `TBFR1518` reader"]
pub type R = crate::R<Tbfr1518Spec>;
#[doc = "Field `NFRX` reader - 1024 to 1518 Byte Frames Received without Error"]
pub type NfrxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 1024 to 1518 Byte Frames Received without Error"]
    #[inline(always)]
    pub fn nfrx(&self) -> NfrxR {
        NfrxR::new(self.bits)
    }
}
#[doc = "1024 to 1518 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbfr1518::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tbfr1518Spec;
impl crate::RegisterSpec for Tbfr1518Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbfr1518::R`](R) reader structure"]
impl crate::Readable for Tbfr1518Spec {}
#[doc = "`reset()` method sets TBFR1518 to value 0"]
impl crate::Resettable for Tbfr1518Spec {}
