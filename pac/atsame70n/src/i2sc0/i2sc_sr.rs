#[doc = "Register `I2SC_SR` reader"]
pub struct R(crate::R<I2SC_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SC_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SC_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SC_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXEN` reader - Receiver Enabled"]
pub struct RXEN_R(crate::FieldReader<bool, bool>);
impl RXEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRDY` reader - Receive Ready"]
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
#[doc = "Field `RXOR` reader - Receive Overrun"]
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
#[doc = "Field `TXEN` reader - Transmitter Enabled"]
pub struct TXEN_R(crate::FieldReader<bool, bool>);
impl TXEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDY` reader - Transmit Ready"]
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
#[doc = "Field `TXUR` reader - Transmit Underrun"]
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
#[doc = "Field `RXORCH` reader - Receive Overrun Channel"]
pub struct RXORCH_R(crate::FieldReader<u8, u8>);
impl RXORCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXORCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXORCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXURCH` reader - Transmit Underrun Channel"]
pub struct TXURCH_R(crate::FieldReader<u8, u8>);
impl TXURCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXURCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXURCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Enabled"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline(always)]
    pub fn rxor(&self) -> RXOR_R {
        RXOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmitter Enabled"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn txur(&self) -> TXUR_R {
        TXUR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Receive Overrun Channel"]
    #[inline(always)]
    pub fn rxorch(&self) -> RXORCH_R {
        RXORCH_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Transmit Underrun Channel"]
    #[inline(always)]
    pub fn txurch(&self) -> TXURCH_R {
        TXURCH_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_sr](index.html) module"]
pub struct I2SC_SR_SPEC;
impl crate::RegisterSpec for I2SC_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sc_sr::R](R) reader structure"]
impl crate::Readable for I2SC_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2SC_SR to value 0"]
impl crate::Resettable for I2SC_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
