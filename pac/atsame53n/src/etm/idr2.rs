#[doc = "Register `IDR2` reader"]
pub struct R(crate::R<IDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ETM ID Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr2](index.html) module"]
pub struct IDR2_SPEC;
impl crate::RegisterSpec for IDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idr2::R](R) reader structure"]
impl crate::Readable for IDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDR2 to value 0"]
impl crate::Resettable for IDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
