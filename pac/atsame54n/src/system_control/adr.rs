#[doc = "Register `ADR` reader"]
pub struct R(crate::R<ADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Auxiliary Feature Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adr](index.html) module"]
pub struct ADR_SPEC;
impl crate::RegisterSpec for ADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adr::R](R) reader structure"]
impl crate::Readable for ADR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADR to value 0"]
impl crate::Resettable for ADR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
