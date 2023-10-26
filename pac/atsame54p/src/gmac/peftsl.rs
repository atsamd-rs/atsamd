#[doc = "Register `PEFTSL` reader"]
pub type R = crate::R<PEFTSL_SPEC>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RUD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new(self.bits)
    }
}
#[doc = "PTP Peer Event Frame Transmitted Seconds Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peftsl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PEFTSL_SPEC;
impl crate::RegisterSpec for PEFTSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peftsl::R`](R) reader structure"]
impl crate::Readable for PEFTSL_SPEC {}
#[doc = "`reset()` method sets PEFTSL to value 0"]
impl crate::Resettable for PEFTSL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
