#[doc = "Register `PID7` reader"]
pub struct R(crate::R<PID7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid7](index.html) module"]
pub struct PID7_SPEC;
impl crate::RegisterSpec for PID7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid7::R](R) reader structure"]
impl crate::Readable for PID7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PID7 to value 0"]
impl crate::Resettable for PID7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
