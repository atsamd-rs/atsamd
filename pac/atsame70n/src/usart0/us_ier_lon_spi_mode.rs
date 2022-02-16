#[doc = "Register `US_IER_LON_SPI_MODE` writer"]
pub struct W(crate::W<US_IER_LON_SPI_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_IER_LON_SPI_MODE_SPEC>;
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
impl From<crate::W<US_IER_LON_SPI_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_IER_LON_SPI_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub struct OVRE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRE_W<'a> {
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
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Enable"]
pub struct TXEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEMPTY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `UNRE` writer - Underrun Error Interrupt Enable"]
pub struct UNRE_W<'a> {
    w: &'a mut W,
}
impl<'a> UNRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RXRDY_W {
        RXRDY_W { w: self }
    }
    #[doc = "Bit 1 - TXRDY Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TXRDY_W {
        TXRDY_W { w: self }
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OVRE_W {
        OVRE_W { w: self }
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Enable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TXEMPTY_W {
        TXEMPTY_W { w: self }
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn unre(&mut self) -> UNRE_W {
        UNRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_ier_lon_spi_mode](index.html) module"]
pub struct US_IER_LON_SPI_MODE_SPEC;
impl crate::RegisterSpec for US_IER_LON_SPI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [us_ier_lon_spi_mode::W](W) writer structure"]
impl crate::Writable for US_IER_LON_SPI_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_IER_LON_SPI_MODE to value 0"]
impl crate::Resettable for US_IER_LON_SPI_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
