#[doc = "Register `PID5` reader"]
pub type R = crate::R<PID5_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PID5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PID5_SPEC;
impl crate::RegisterSpec for PID5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid5::R`](R) reader structure"]
impl crate::Readable for PID5_SPEC {}
#[doc = "`reset()` method sets PID5 to value 0"]
impl crate::Resettable for PID5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
