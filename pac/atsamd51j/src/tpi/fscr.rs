#[doc = "Register `FSCR` reader"]
pub struct R(crate::R<FSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Formatter Synchronization Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fscr](index.html) module"]
pub struct FSCR_SPEC;
impl crate::RegisterSpec for FSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fscr::R](R) reader structure"]
impl crate::Readable for FSCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSCR to value 0"]
impl crate::Resettable for FSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
