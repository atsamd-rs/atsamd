#[doc = "Register `BCFR` reader"]
pub type R = crate::R<BcfrSpec>;
#[doc = "Field `BFRX` reader - Broadcast Frames Received without Error"]
pub type BfrxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Broadcast Frames Received without Error"]
    #[inline(always)]
    pub fn bfrx(&self) -> BfrxR {
        BfrxR::new(self.bits)
    }
}
#[doc = "Broadcast Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcfrSpec;
impl crate::RegisterSpec for BcfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcfr::R`](R) reader structure"]
impl crate::Readable for BcfrSpec {}
#[doc = "`reset()` method sets BCFR to value 0"]
impl crate::Resettable for BcfrSpec {}
