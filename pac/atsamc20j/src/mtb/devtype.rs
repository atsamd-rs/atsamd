#[doc = "Register `DEVTYPE` reader"]
pub struct R(crate::R<DEVTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "MTB Device Type\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devtype](index.html) module"]
pub struct DEVTYPE_SPEC;
impl crate::RegisterSpec for DEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devtype::R](R) reader structure"]
impl crate::Readable for DEVTYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVTYPE to value 0"]
impl crate::Resettable for DEVTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
