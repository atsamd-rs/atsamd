#[doc = "Register `PBLDATA0` reader"]
pub struct R(crate::R<PBLDATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBLDATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBLDATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBLDATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Page Buffer Load Data 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbldata0](index.html) module"]
pub struct PBLDATA0_SPEC;
impl crate::RegisterSpec for PBLDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbldata0::R](R) reader structure"]
impl crate::Readable for PBLDATA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PBLDATA0 to value 0"]
impl crate::Resettable for PBLDATA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
