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
pub type RXO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Received Octets"]
    #[inline(always)]
    pub fn rxo(&self) -> RXO_R {
        RXO_R::new(self.bits)
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
    const RESET_VALUE: Self::Ux = 0;
}
