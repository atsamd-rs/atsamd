#[doc = "Register `PEFRSL` reader"]
pub type R = crate::R<PEFRSL_SPEC>;
#[doc = "Field `RUD` reader - Register Update"]
pub type RUD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new(self.bits)
    }
}
#[doc = "PTP Peer Event Frame Received Seconds Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pefrsl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PEFRSL_SPEC;
impl crate::RegisterSpec for PEFRSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pefrsl::R`](R) reader structure"]
impl crate::Readable for PEFRSL_SPEC {}
#[doc = "`reset()` method sets PEFRSL to value 0"]
impl crate::Resettable for PEFRSL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
