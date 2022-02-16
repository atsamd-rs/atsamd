#[doc = "Register `UART_RHR` reader"]
pub struct R(crate::R<UART_RHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_RHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_RHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_RHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXCHR` reader - Received Character"]
pub struct RXCHR_R(crate::FieldReader<u8, u8>);
impl RXCHR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXCHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCHR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Received Character"]
    #[inline(always)]
    pub fn rxchr(&self) -> RXCHR_R {
        RXCHR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receive Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rhr](index.html) module"]
pub struct UART_RHR_SPEC;
impl crate::RegisterSpec for UART_RHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_rhr::R](R) reader structure"]
impl crate::Readable for UART_RHR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_RHR to value 0"]
impl crate::Resettable for UART_RHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
