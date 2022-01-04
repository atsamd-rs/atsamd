#[doc = "Register `PDSR` reader"]
pub struct R(crate::R<PDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ETM Device Power-Down Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsr](index.html) module"]
pub struct PDSR_SPEC;
impl crate::RegisterSpec for PDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdsr::R](R) reader structure"]
impl crate::Readable for PDSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDSR to value 0x01"]
impl crate::Resettable for PDSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
