#[doc = "Register `DFR` reader"]
pub struct R(crate::R<DFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Debug Feature Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfr](index.html) module"]
pub struct DFR_SPEC;
impl crate::RegisterSpec for DFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfr::R](R) reader structure"]
impl crate::Readable for DFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFR to value 0"]
impl crate::Resettable for DFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
