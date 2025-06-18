#[doc = "Register `PFT` reader"]
pub type R = crate::R<PftSpec>;
#[doc = "Field `PFTX` reader - Pause Frames Transmitted Register"]
pub type PftxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Pause Frames Transmitted Register"]
    #[inline(always)]
    pub fn pftx(&self) -> PftxR {
        PftxR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Pause Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pft::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PftSpec;
impl crate::RegisterSpec for PftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pft::R`](R) reader structure"]
impl crate::Readable for PftSpec {}
#[doc = "`reset()` method sets PFT to value 0"]
impl crate::Resettable for PftSpec {}
