#[doc = "Register `AUTHSTATUS` reader"]
pub struct R(crate::R<AUTHSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTHSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTHSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTHSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "MTB Authentication Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [authstatus](index.html) module"]
pub struct AUTHSTATUS_SPEC;
impl crate::RegisterSpec for AUTHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [authstatus::R](R) reader structure"]
impl crate::Readable for AUTHSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AUTHSTATUS to value 0"]
impl crate::Resettable for AUTHSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
