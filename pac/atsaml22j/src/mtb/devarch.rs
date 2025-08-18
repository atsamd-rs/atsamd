#[doc = "Register `DEVARCH` reader"]
pub struct R(crate::R<DEVARCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVARCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVARCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVARCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "MTB Device Architecture\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devarch](index.html) module"]
pub struct DEVARCH_SPEC;
impl crate::RegisterSpec for DEVARCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devarch::R](R) reader structure"]
impl crate::Readable for DEVARCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVARCH to value 0"]
impl crate::Resettable for DEVARCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
