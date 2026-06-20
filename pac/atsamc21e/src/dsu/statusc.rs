#[doc = "Register `STATUSC` reader"]
pub struct R(crate::R<STATUSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATE` reader - State"]
pub struct STATE_R(crate::FieldReader<u8, u8>);
impl STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - State"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "Status C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusc](index.html) module"]
pub struct STATUSC_SPEC;
impl crate::RegisterSpec for STATUSC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [statusc::R](R) reader structure"]
impl crate::Readable for STATUSC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSC to value 0"]
impl crate::Resettable for STATUSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
