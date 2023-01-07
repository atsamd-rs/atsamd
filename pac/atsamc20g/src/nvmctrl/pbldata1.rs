#[doc = "Register `PBLDATA1` reader"]
pub struct R(crate::R<PBLDATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBLDATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBLDATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBLDATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Page Buffer Load Data 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbldata1](index.html) module"]
pub struct PBLDATA1_SPEC;
impl crate::RegisterSpec for PBLDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbldata1::R](R) reader structure"]
impl crate::Readable for PBLDATA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PBLDATA1 to value 0"]
impl crate::Resettable for PBLDATA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
