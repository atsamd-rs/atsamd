#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADCBUSY` reader - ADC Busy Status"]
pub struct ADCBUSY_R(crate::FieldReader<bool, bool>);
impl ADCBUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADCBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCC` reader - Window Comparator Counter"]
pub struct WCC_R(crate::FieldReader<u8, u8>);
impl WCC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WCC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - ADC Busy Status"]
    #[inline(always)]
    pub fn adcbusy(&self) -> ADCBUSY_R {
        ADCBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:7 - Window Comparator Counter"]
    #[inline(always)]
    pub fn wcc(&self) -> WCC_R {
        WCC_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
