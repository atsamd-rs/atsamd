#[doc = "Register `PID3` reader"]
pub type R = crate::R<PID3_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PID3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PID3_SPEC;
impl crate::RegisterSpec for PID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid3::R`](R) reader structure"]
impl crate::Readable for PID3_SPEC {}
#[doc = "`reset()` method sets PID3 to value 0"]
impl crate::Resettable for PID3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
