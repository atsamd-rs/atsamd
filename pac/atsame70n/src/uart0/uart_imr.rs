#[doc = "Register `UART_IMR` reader"]
pub struct R(crate::R<UART_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - Mask RXRDY Interrupt"]
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
#[doc = "Field `TXRDY` reader - Disable TXRDY Interrupt"]
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
#[doc = "Field `OVRE` reader - Mask Overrun Error Interrupt"]
pub struct OVRE_R(crate::FieldReader<bool, bool>);
impl OVRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAME` reader - Mask Framing Error Interrupt"]
pub struct FRAME_R(crate::FieldReader<bool, bool>);
impl FRAME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRAME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARE` reader - Mask Parity Error Interrupt"]
pub struct PARE_R(crate::FieldReader<bool, bool>);
impl PARE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEMPTY` reader - Mask TXEMPTY Interrupt"]
pub struct TXEMPTY_R(crate::FieldReader<bool, bool>);
impl TXEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP` reader - Mask Comparison Interrupt"]
pub struct CMP_R(crate::FieldReader<bool, bool>);
impl CMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Mask RXRDY Interrupt"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable TXRDY Interrupt"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask Overrun Error Interrupt"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mask Framing Error Interrupt"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mask Parity Error Interrupt"]
    #[inline(always)]
    pub fn pare(&self) -> PARE_R {
        PARE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Mask TXEMPTY Interrupt"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Mask Comparison Interrupt"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_imr](index.html) module"]
pub struct UART_IMR_SPEC;
impl crate::RegisterSpec for UART_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_imr::R](R) reader structure"]
impl crate::Readable for UART_IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_IMR to value 0"]
impl crate::Resettable for UART_IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
