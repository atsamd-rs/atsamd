#[doc = "Register `UFR` reader"]
pub type R = crate::R<UFR_SPEC>;
#[doc = "Field `UFRX` reader - Undersize Frames Received"]
pub type UFRX_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Undersize Frames Received"]
    #[inline(always)]
    pub fn ufrx(&self) -> UFRX_R {
        UFRX_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Undersize Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ufr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UFR_SPEC;
impl crate::RegisterSpec for UFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ufr::R`](R) reader structure"]
impl crate::Readable for UFR_SPEC {}
#[doc = "`reset()` method sets UFR to value 0"]
impl crate::Resettable for UFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
