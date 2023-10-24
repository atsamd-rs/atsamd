#[doc = "Register `PFR` reader"]
pub type R = crate::R<PFR_SPEC>;
#[doc = "Field `PFRX` reader - Pause Frames Received Register"]
pub type PFRX_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Pause Frames Received Register"]
    #[inline(always)]
    pub fn pfrx(&self) -> PFRX_R {
        PFRX_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Pause Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFR_SPEC;
impl crate::RegisterSpec for PFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr::R`](R) reader structure"]
impl crate::Readable for PFR_SPEC {}
#[doc = "`reset()` method sets PFR to value 0"]
impl crate::Resettable for PFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
