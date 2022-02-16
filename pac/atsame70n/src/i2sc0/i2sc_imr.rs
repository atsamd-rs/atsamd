#[doc = "Register `I2SC_IMR` reader"]
pub struct R(crate::R<I2SC_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SC_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SC_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SC_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - Receiver Ready Interrupt Disable"]
pub struct RXRDY_R(crate::FieldReader<bool, bool>);
impl RXRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOR` reader - Receiver Overrun Interrupt Disable"]
pub struct RXOR_R(crate::FieldReader<bool, bool>);
impl RXOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDY` reader - Transmit Ready Interrupt Disable"]
pub struct TXRDY_R(crate::FieldReader<bool, bool>);
impl TXRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUR` reader - Transmit Underflow Interrupt Disable"]
pub struct TXUR_R(crate::FieldReader<bool, bool>);
impl TXUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Receiver Ready Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receiver Overrun Interrupt Disable"]
    #[inline(always)]
    pub fn rxor(&self) -> RXOR_R {
        RXOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Ready Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Underflow Interrupt Disable"]
    #[inline(always)]
    pub fn txur(&self) -> TXUR_R {
        TXUR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_imr](index.html) module"]
pub struct I2SC_IMR_SPEC;
impl crate::RegisterSpec for I2SC_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sc_imr::R](R) reader structure"]
impl crate::Readable for I2SC_IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2SC_IMR to value 0"]
impl crate::Resettable for I2SC_IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
