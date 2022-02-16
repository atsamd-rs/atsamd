#[doc = "Register `I2SC_RHR` reader"]
pub struct R(crate::R<I2SC_RHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SC_RHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SC_RHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SC_RHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RHR` reader - Receiver Holding Register"]
pub struct RHR_R(crate::FieldReader<u32, u32>);
impl RHR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RHR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Receiver Holding Register"]
    #[inline(always)]
    pub fn rhr(&self) -> RHR_R {
        RHR_R::new(self.bits as u32)
    }
}
#[doc = "Receiver Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_rhr](index.html) module"]
pub struct I2SC_RHR_SPEC;
impl crate::RegisterSpec for I2SC_RHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sc_rhr::R](R) reader structure"]
impl crate::Readable for I2SC_RHR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2SC_RHR to value 0"]
impl crate::Resettable for I2SC_RHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
