#[doc = "Register `ITMISCIN` reader"]
pub type R = crate::R<ITMISCIN_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<ITMISCIN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Integration Test Miscellaneous Inputs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itmiscin::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITMISCIN_SPEC;
impl crate::RegisterSpec for ITMISCIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itmiscin::R`](R) reader structure"]
impl crate::Readable for ITMISCIN_SPEC {}
#[doc = "`reset()` method sets ITMISCIN to value 0"]
impl crate::Resettable for ITMISCIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
