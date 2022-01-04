#[doc = "Register `PIDR0` reader"]
pub struct R(crate::R<PIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ETM Peripheral Identification Register #0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr0](index.html) module"]
pub struct PIDR0_SPEC;
impl crate::RegisterSpec for PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr0::R](R) reader structure"]
impl crate::Readable for PIDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR0 to value 0x25"]
impl crate::Resettable for PIDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x25
    }
}
