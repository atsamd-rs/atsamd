#[doc = "Register `PID5` reader"]
pub struct R(crate::R<PID5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid5](index.html) module"]
pub struct PID5_SPEC;
impl crate::RegisterSpec for PID5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid5::R](R) reader structure"]
impl crate::Readable for PID5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PID5 to value 0"]
impl crate::Resettable for PID5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
