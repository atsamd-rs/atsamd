#[doc = "Register `MFR` reader"]
pub type R = crate::R<MFR_SPEC>;
#[doc = "Field `MFRX` reader - Multicast Frames Received without Error"]
pub type MFRX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Multicast Frames Received without Error"]
    #[inline(always)]
    pub fn mfrx(&self) -> MFRX_R {
        MFRX_R::new(self.bits)
    }
}
#[doc = "Multicast Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MFR_SPEC;
impl crate::RegisterSpec for MFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfr::R`](R) reader structure"]
impl crate::Readable for MFR_SPEC {}
#[doc = "`reset()` method sets MFR to value 0"]
impl crate::Resettable for MFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
