#[doc = "Register `DEVARCH` reader"]
pub type R = crate::R<DEVARCH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DEVARCH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "MTB Device Architecture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devarch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVARCH_SPEC;
impl crate::RegisterSpec for DEVARCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devarch::R`](R) reader structure"]
impl crate::Readable for DEVARCH_SPEC {}
#[doc = "`reset()` method sets DEVARCH to value 0"]
impl crate::Resettable for DEVARCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
