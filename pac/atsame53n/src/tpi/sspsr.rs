#[doc = "Register `SSPSR` reader"]
pub struct R(crate::R<SSPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Supported Parallel Port Size Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspsr](index.html) module"]
pub struct SSPSR_SPEC;
impl crate::RegisterSpec for SSPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspsr::R](R) reader structure"]
impl crate::Readable for SSPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSPSR to value 0"]
impl crate::Resettable for SSPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
