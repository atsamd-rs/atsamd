#[doc = "Register `PID3` reader"]
pub struct R(crate::R<PID3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid3](index.html) module"]
pub struct PID3_SPEC;
impl crate::RegisterSpec for PID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid3::R](R) reader structure"]
impl crate::Readable for PID3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PID3 to value 0"]
impl crate::Resettable for PID3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
