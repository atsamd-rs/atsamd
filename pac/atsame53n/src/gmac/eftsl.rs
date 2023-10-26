#[doc = "Register `EFTSL` reader"]
pub type R = crate::R<EFTSL_SPEC>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RUD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new(self.bits)
    }
}
#[doc = "PTP Event Frame Transmitted Seconds Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eftsl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EFTSL_SPEC;
impl crate::RegisterSpec for EFTSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eftsl::R`](R) reader structure"]
impl crate::Readable for EFTSL_SPEC {}
#[doc = "`reset()` method sets EFTSL to value 0"]
impl crate::Resettable for EFTSL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
