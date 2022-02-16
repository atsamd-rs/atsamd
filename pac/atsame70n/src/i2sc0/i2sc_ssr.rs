#[doc = "Register `I2SC_SSR` writer"]
pub struct W(crate::W<I2SC_SSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SC_SSR_SPEC>;
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
impl From<crate::W<I2SC_SSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SC_SSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXOR` writer - Receive Overrun Status Set"]
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
#[doc = "Field `TXUR` writer - Transmit Underrun Status Set"]
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
#[doc = "Field `RXORCH` writer - Receive Overrun Per Channel Status Set"]
pub struct RXORCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXORCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `TXURCH` writer - Transmit Underrun Per Channel Status Set"]
pub struct TXURCH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXURCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
impl W {
    #[doc = "Bit 2 - Receive Overrun Status Set"]
    #[inline(always)]
    pub fn rxor(&mut self) -> RXOR_W {
        RXOR_W { w: self }
    }
    #[doc = "Bit 6 - Transmit Underrun Status Set"]
    #[inline(always)]
    pub fn txur(&mut self) -> TXUR_W {
        TXUR_W { w: self }
    }
    #[doc = "Bits 8:9 - Receive Overrun Per Channel Status Set"]
    #[inline(always)]
    pub fn rxorch(&mut self) -> RXORCH_W {
        RXORCH_W { w: self }
    }
    #[doc = "Bits 20:21 - Transmit Underrun Per Channel Status Set"]
    #[inline(always)]
    pub fn txurch(&mut self) -> TXURCH_W {
        TXURCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_ssr](index.html) module"]
pub struct I2SC_SSR_SPEC;
impl crate::RegisterSpec for I2SC_SSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [i2sc_ssr::W](W) writer structure"]
impl crate::Writable for I2SC_SSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2SC_SSR to value 0"]
impl crate::Resettable for I2SC_SSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
