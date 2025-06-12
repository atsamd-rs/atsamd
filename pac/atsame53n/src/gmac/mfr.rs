#[doc = "Register `MFR` reader"]
pub type R = crate::R<MfrSpec>;
#[doc = "Field `MFRX` reader - Multicast Frames Received without Error"]
pub type MfrxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Multicast Frames Received without Error"]
    #[inline(always)]
    pub fn mfrx(&self) -> MfrxR {
        MfrxR::new(self.bits)
    }
}
#[doc = "Multicast Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MfrSpec;
impl crate::RegisterSpec for MfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfr::R`](R) reader structure"]
impl crate::Readable for MfrSpec {}
#[doc = "`reset()` method sets MFR to value 0"]
impl crate::Resettable for MfrSpec {}
