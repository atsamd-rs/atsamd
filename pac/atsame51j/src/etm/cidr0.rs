#[doc = "Register `CIDR0` reader"]
pub struct R(crate::R<CIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ETM Component Identification Register #0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr0](index.html) module"]
pub struct CIDR0_SPEC;
impl crate::RegisterSpec for CIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cidr0::R](R) reader structure"]
impl crate::Readable for CIDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIDR0 to value 0x0d"]
impl crate::Resettable for CIDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0d
    }
}
