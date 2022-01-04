#[doc = "Register `PID2` reader"]
pub struct R(crate::R<PID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid2](index.html) module"]
pub struct PID2_SPEC;
impl crate::RegisterSpec for PID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid2::R](R) reader structure"]
impl crate::Readable for PID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PID2 to value 0"]
impl crate::Resettable for PID2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
