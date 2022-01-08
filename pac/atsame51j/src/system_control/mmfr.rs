#[doc = "Register `MMFR[%s]` reader"]
pub struct R(crate::R<MMFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Memory Model Feature Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmfr](index.html) module"]
pub struct MMFR_SPEC;
impl crate::RegisterSpec for MMFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmfr::R](R) reader structure"]
impl crate::Readable for MMFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMFR[%s]
to value 0"]
impl crate::Resettable for MMFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
