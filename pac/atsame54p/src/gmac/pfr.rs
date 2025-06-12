#[doc = "Register `PFR` reader"]
pub type R = crate::R<PfrSpec>;
#[doc = "Field `PFRX` reader - Pause Frames Received Register"]
pub type PfrxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Pause Frames Received Register"]
    #[inline(always)]
    pub fn pfrx(&self) -> PfrxR {
        PfrxR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Pause Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfrSpec;
impl crate::RegisterSpec for PfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr::R`](R) reader structure"]
impl crate::Readable for PfrSpec {}
#[doc = "`reset()` method sets PFR to value 0"]
impl crate::Resettable for PfrSpec {}
