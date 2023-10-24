#[doc = "Register `SSPSR` reader"]
pub type R = crate::R<SSPSR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SSPSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Supported Parallel Port Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sspsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPSR_SPEC;
impl crate::RegisterSpec for SSPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspsr::R`](R) reader structure"]
impl crate::Readable for SSPSR_SPEC {}
#[doc = "`reset()` method sets SSPSR to value 0"]
impl crate::Resettable for SSPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
