#[doc = "Register `PIDR1` reader"]
pub struct R(crate::R<PIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ETM Peripheral Identification Register #1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr1](index.html) module"]
pub struct PIDR1_SPEC;
impl crate::RegisterSpec for PIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr1::R](R) reader structure"]
impl crate::Readable for PIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR1 to value 0xb9"]
impl crate::Resettable for PIDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb9
    }
}
