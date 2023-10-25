#[doc = "Register `EFRSL` reader"]
pub type R = crate::R<EFRSL_SPEC>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RUD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new(self.bits)
    }
}
#[doc = "PTP Event Frame Received Seconds Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efrsl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EFRSL_SPEC;
impl crate::RegisterSpec for EFRSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efrsl::R`](R) reader structure"]
impl crate::Readable for EFRSL_SPEC {}
#[doc = "`reset()` method sets EFRSL to value 0"]
impl crate::Resettable for EFRSL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
