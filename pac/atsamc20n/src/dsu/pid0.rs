#[doc = "Register `PID0` reader"]
pub struct R(crate::R<PID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARTNBL` reader - Part Number Low"]
pub struct PARTNBL_R(crate::FieldReader<u8, u8>);
impl PARTNBL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PARTNBL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTNBL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Part Number Low"]
    #[inline(always)]
    pub fn partnbl(&self) -> PARTNBL_R {
        PARTNBL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral Identification 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid0](index.html) module"]
pub struct PID0_SPEC;
impl crate::RegisterSpec for PID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid0::R](R) reader structure"]
impl crate::Readable for PID0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PID0 to value 0xd0"]
impl crate::Resettable for PID0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xd0
    }
}
