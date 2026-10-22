#[doc = "Register `LOCKSTATUS` reader"]
pub struct R(crate::R<LOCKSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "MTB Lock Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockstatus](index.html) module"]
pub struct LOCKSTATUS_SPEC;
impl crate::RegisterSpec for LOCKSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockstatus::R](R) reader structure"]
impl crate::Readable for LOCKSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOCKSTATUS to value 0"]
impl crate::Resettable for LOCKSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
