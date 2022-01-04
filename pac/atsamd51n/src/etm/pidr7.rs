#[doc = "Register `PIDR7` reader"]
pub struct R(crate::R<PIDR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ETM Peripheral Identification Register #7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr7](index.html) module"]
pub struct PIDR7_SPEC;
impl crate::RegisterSpec for PIDR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr7::R](R) reader structure"]
impl crate::Readable for PIDR7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR7 to value 0"]
impl crate::Resettable for PIDR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
