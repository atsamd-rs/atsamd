#[doc = "Register `TC_CV` reader"]
pub struct R(crate::R<TC_CV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TC_CV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TC_CV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TC_CV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CV` reader - Counter Value"]
pub struct CV_R(crate::FieldReader<u32, u32>);
impl CV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Counter Value"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new(self.bits as u32)
    }
}
#[doc = "Counter Value (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_cv](index.html) module"]
pub struct TC_CV_SPEC;
impl crate::RegisterSpec for TC_CV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tc_cv::R](R) reader structure"]
impl crate::Readable for TC_CV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TC_CV to value 0"]
impl crate::Resettable for TC_CV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
