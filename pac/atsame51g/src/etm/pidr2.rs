#[doc = "Register `PIDR2` reader"]
pub struct R(crate::R<PIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ETM Peripheral Identification Register #2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr2](index.html) module"]
pub struct PIDR2_SPEC;
impl crate::RegisterSpec for PIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr2::R](R) reader structure"]
impl crate::Readable for PIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR2 to value 0x0b"]
impl crate::Resettable for PIDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b
    }
}
