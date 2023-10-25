#[doc = "Register `ITATBCTR2` reader"]
pub type R = crate::R<ITATBCTR2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<ITATBCTR2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Integration Test ATB Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itatbctr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITATBCTR2_SPEC;
impl crate::RegisterSpec for ITATBCTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itatbctr2::R`](R) reader structure"]
impl crate::Readable for ITATBCTR2_SPEC {}
#[doc = "`reset()` method sets ITATBCTR2 to value 0"]
impl crate::Resettable for ITATBCTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
