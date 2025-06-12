#[doc = "Register `UFR` reader"]
pub type R = crate::R<UfrSpec>;
#[doc = "Field `UFRX` reader - Undersize Frames Received"]
pub type UfrxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Undersize Frames Received"]
    #[inline(always)]
    pub fn ufrx(&self) -> UfrxR {
        UfrxR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Undersize Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ufr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UfrSpec;
impl crate::RegisterSpec for UfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ufr::R`](R) reader structure"]
impl crate::Readable for UfrSpec {}
#[doc = "`reset()` method sets UFR to value 0"]
impl crate::Resettable for UfrSpec {}
