#[doc = "Register `REM` reader"]
pub struct R(crate::R<REM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REM` reader - REM"]
pub struct REM_R(crate::FieldReader<u32, u32>);
impl REM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        REM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - REM"]
    #[inline(always)]
    pub fn rem(&self) -> REM_R {
        REM_R::new(self.bits as u32)
    }
}
#[doc = "Remainder\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rem](index.html) module"]
pub struct REM_SPEC;
impl crate::RegisterSpec for REM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rem::R](R) reader structure"]
impl crate::Readable for REM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REM to value 0"]
impl crate::Resettable for REM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
