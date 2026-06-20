#[doc = "Register `TSCV` reader"]
pub struct R(crate::R<TSCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSC` reader - Timestamp Counter"]
pub struct TSC_R(crate::FieldReader<u16, u16>);
impl TSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Timestamp Counter"]
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timestamp Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscv](index.html) module"]
pub struct TSCV_SPEC;
impl crate::RegisterSpec for TSCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tscv::R](R) reader structure"]
impl crate::Readable for TSCV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSCV to value 0"]
impl crate::Resettable for TSCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
