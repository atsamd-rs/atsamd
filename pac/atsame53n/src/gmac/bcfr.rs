#[doc = "Register `BCFR` reader"]
pub type R = crate::R<BCFR_SPEC>;
#[doc = "Field `BFRX` reader - Broadcast Frames Received without Error"]
pub type BFRX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Broadcast Frames Received without Error"]
    #[inline(always)]
    pub fn bfrx(&self) -> BFRX_R {
        BFRX_R::new(self.bits)
    }
}
#[doc = "Broadcast Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCFR_SPEC;
impl crate::RegisterSpec for BCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcfr::R`](R) reader structure"]
impl crate::Readable for BCFR_SPEC {}
#[doc = "`reset()` method sets BCFR to value 0"]
impl crate::Resettable for BCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
