#[doc = "Register `SISR` reader"]
pub type R = crate::R<SISR_SPEC>;
#[doc = "Field `INTSSL` reader - Interrupt Signal for Each Slot"]
pub type INTSSL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Signal for Each Slot"]
    #[inline(always)]
    pub fn intssl(&self) -> INTSSL_R {
        INTSSL_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Slot Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SISR_SPEC;
impl crate::RegisterSpec for SISR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sisr::R`](R) reader structure"]
impl crate::Readable for SISR_SPEC {}
#[doc = "`reset()` method sets SISR to value 0"]
impl crate::Resettable for SISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
