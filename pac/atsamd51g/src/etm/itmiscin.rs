#[doc = "Register `ITMISCIN` reader"]
pub struct R(crate::R<ITMISCIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITMISCIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITMISCIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITMISCIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ETM Integration Test Miscellaneous Inputs\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itmiscin](index.html) module"]
pub struct ITMISCIN_SPEC;
impl crate::RegisterSpec for ITMISCIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itmiscin::R](R) reader structure"]
impl crate::Readable for ITMISCIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITMISCIN to value 0"]
impl crate::Resettable for ITMISCIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
