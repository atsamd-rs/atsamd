#[doc = "Register `PFT` reader"]
pub type R = crate::R<PFT_SPEC>;
#[doc = "Field `PFTX` reader - Pause Frames Transmitted Register"]
pub type PFTX_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Pause Frames Transmitted Register"]
    #[inline(always)]
    pub fn pftx(&self) -> PFTX_R {
        PFTX_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Pause Frames Transmitted Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pft::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFT_SPEC;
impl crate::RegisterSpec for PFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pft::R`](R) reader structure"]
impl crate::Readable for PFT_SPEC {}
#[doc = "`reset()` method sets PFT to value 0"]
impl crate::Resettable for PFT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
