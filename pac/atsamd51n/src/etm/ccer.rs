#[doc = "Register `CCER` reader"]
pub type R = crate::R<CCER_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CCER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Configuration Code Extension Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccer::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCER_SPEC;
impl crate::RegisterSpec for CCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccer::R`](R) reader structure"]
impl crate::Readable for CCER_SPEC {}
#[doc = "`reset()` method sets CCER to value 0x1854_1800"]
impl crate::Resettable for CCER_SPEC {
    const RESET_VALUE: Self::Ux = 0x1854_1800;
}
