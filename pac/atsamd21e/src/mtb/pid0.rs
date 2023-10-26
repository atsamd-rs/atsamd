#[doc = "Register `PID0` reader"]
pub type R = crate::R<PID0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PID0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PID0_SPEC;
impl crate::RegisterSpec for PID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid0::R`](R) reader structure"]
impl crate::Readable for PID0_SPEC {}
#[doc = "`reset()` method sets PID0 to value 0"]
impl crate::Resettable for PID0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
