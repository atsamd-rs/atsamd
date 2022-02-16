#[doc = "Register `I2SC_IDR` writer"]
pub struct W(crate::W<I2SC_IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SC_IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<I2SC_IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SC_IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXRDY` writer - Receiver Ready Interrupt Disable"]
pub struct RXRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRDY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RXOR` writer - Receiver Overrun Interrupt Disable"]
pub struct RXOR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Disable"]
pub struct TXRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRDY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TXUR` writer - Transmit Underflow Interrupt Disable"]
pub struct TXUR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl W {
    #[doc = "Bit 1 - Receiver Ready Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RXRDY_W {
        RXRDY_W { w: self }
    }
    #[doc = "Bit 2 - Receiver Overrun Interrupt Disable"]
    #[inline(always)]
    pub fn rxor(&mut self) -> RXOR_W {
        RXOR_W { w: self }
    }
    #[doc = "Bit 5 - Transmit Ready Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TXRDY_W {
        TXRDY_W { w: self }
    }
    #[doc = "Bit 6 - Transmit Underflow Interrupt Disable"]
    #[inline(always)]
    pub fn txur(&mut self) -> TXUR_W {
        TXUR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_idr](index.html) module"]
pub struct I2SC_IDR_SPEC;
impl crate::RegisterSpec for I2SC_IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [i2sc_idr::W](W) writer structure"]
impl crate::Writable for I2SC_IDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2SC_IDR to value 0"]
impl crate::Resettable for I2SC_IDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
