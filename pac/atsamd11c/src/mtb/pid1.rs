#[doc = "Register `PID1` reader"]
pub type R = crate::R<PID1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PID1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PID1_SPEC;
impl crate::RegisterSpec for PID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid1::R`](R) reader structure"]
impl crate::Readable for PID1_SPEC {}
#[doc = "`reset()` method sets PID1 to value 0"]
impl crate::Resettable for PID1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
