#[doc = "Register `US_CR_SPI_MODE` writer"]
pub struct W(crate::W<US_CR_SPI_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_CR_SPI_MODE_SPEC>;
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
impl From<crate::W<US_CR_SPI_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_CR_SPI_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTRX` writer - Reset Receiver"]
pub struct RSTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTRX_W<'a> {
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
#[doc = "Field `RSTTX` writer - Reset Transmitter"]
pub struct RSTTX_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTTX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub struct RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub struct RXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDIS_W<'a> {
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
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub struct TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_W<'a> {
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
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub struct TXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RSTSTA` writer - Reset Status Bits"]
pub struct RSTSTA_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSTA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `FCS` writer - Force SPI Chip Select"]
pub struct FCS_W<'a> {
    w: &'a mut W,
}
impl<'a> FCS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `RCS` writer - Release SPI Chip Select"]
pub struct RCS_W<'a> {
    w: &'a mut W,
}
impl<'a> RCS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl W {
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline(always)]
    pub fn rstrx(&mut self) -> RSTRX_W {
        RSTRX_W { w: self }
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline(always)]
    pub fn rsttx(&mut self) -> RSTTX_W {
        RSTTX_W { w: self }
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W { w: self }
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline(always)]
    pub fn rxdis(&mut self) -> RXDIS_W {
        RXDIS_W { w: self }
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W { w: self }
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline(always)]
    pub fn txdis(&mut self) -> TXDIS_W {
        TXDIS_W { w: self }
    }
    #[doc = "Bit 8 - Reset Status Bits"]
    #[inline(always)]
    pub fn rststa(&mut self) -> RSTSTA_W {
        RSTSTA_W { w: self }
    }
    #[doc = "Bit 18 - Force SPI Chip Select"]
    #[inline(always)]
    pub fn fcs(&mut self) -> FCS_W {
        FCS_W { w: self }
    }
    #[doc = "Bit 19 - Release SPI Chip Select"]
    #[inline(always)]
    pub fn rcs(&mut self) -> RCS_W {
        RCS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_cr_spi_mode](index.html) module"]
pub struct US_CR_SPI_MODE_SPEC;
impl crate::RegisterSpec for US_CR_SPI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [us_cr_spi_mode::W](W) writer structure"]
impl crate::Writable for US_CR_SPI_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_CR_SPI_MODE to value 0"]
impl crate::Resettable for US_CR_SPI_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
