#[doc = "Register `ORLO` reader"]
pub struct R(crate::R<ORLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ORLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ORLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ORLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXO` reader - Received Octets"]
pub struct RXO_R(crate::FieldReader<u32, u32>);
impl RXO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RXO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Received Octets"]
    #[inline(always)]
    pub fn rxo(&self) -> RXO_R {
        RXO_R::new(self.bits as u32)
    }
}
#[doc = "Octets Received \\[31:0\\]
Received\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [orlo](index.html) module"]
pub struct ORLO_SPEC;
impl crate::RegisterSpec for ORLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [orlo::R](R) reader structure"]
impl crate::Readable for ORLO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ORLO to value 0"]
impl crate::Resettable for ORLO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
