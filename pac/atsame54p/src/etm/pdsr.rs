#[doc = "Register `PDSR` reader"]
pub type R = crate::R<PDSR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PDSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Device Power-Down Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDSR_SPEC;
impl crate::RegisterSpec for PDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdsr::R`](R) reader structure"]
impl crate::Readable for PDSR_SPEC {}
#[doc = "`reset()` method sets PDSR to value 0x01"]
impl crate::Resettable for PDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
