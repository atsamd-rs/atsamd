#[doc = "Register `PIDR3` reader"]
pub struct R(crate::R<PIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ETM Peripheral Identification Register #3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr3](index.html) module"]
pub struct PIDR3_SPEC;
impl crate::RegisterSpec for PIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr3::R](R) reader structure"]
impl crate::Readable for PIDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR3 to value 0"]
impl crate::Resettable for PIDR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
