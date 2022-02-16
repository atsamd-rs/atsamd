#[doc = "Register `TWIHS_RHR` reader"]
pub struct R(crate::R<TWIHS_RHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWIHS_RHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWIHS_RHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWIHS_RHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA` reader - Master or Slave Receive Holding Data"]
pub struct RXDATA_R(crate::FieldReader<u8, u8>);
impl RXDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Master or Slave Receive Holding Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receive Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_rhr](index.html) module"]
pub struct TWIHS_RHR_SPEC;
impl crate::RegisterSpec for TWIHS_RHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twihs_rhr::R](R) reader structure"]
impl crate::Readable for TWIHS_RHR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TWIHS_RHR to value 0"]
impl crate::Resettable for TWIHS_RHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
