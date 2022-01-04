#[doc = "Register `PIDR6` reader"]
pub struct R(crate::R<PIDR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ETM Peripheral Identification Register #6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr6](index.html) module"]
pub struct PIDR6_SPEC;
impl crate::RegisterSpec for PIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr6::R](R) reader structure"]
impl crate::Readable for PIDR6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR6 to value 0"]
impl crate::Resettable for PIDR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
