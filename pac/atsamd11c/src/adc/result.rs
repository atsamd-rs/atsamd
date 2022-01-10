#[doc = "Register `RESULT` reader"]
pub struct R(crate::R<RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT` reader - Result Conversion Value"]
pub struct RESULT_R(crate::FieldReader<u16, u16>);
impl RESULT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RESULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESULT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Result Conversion Value"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(self.bits as u16)
    }
}
#[doc = "Result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](index.html) module"]
pub struct RESULT_SPEC;
impl crate::RegisterSpec for RESULT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [result::R](R) reader structure"]
impl crate::Readable for RESULT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for RESULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
