#[doc = "Register `SPI_TDR` writer"]
pub struct W(crate::W<SPI_TDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_TDR_SPEC>;
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
impl From<crate::W<SPI_TDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_TDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TD` writer - Transmit Data"]
pub struct TD_W<'a> {
    w: &'a mut W,
}
impl<'a> TD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Peripheral Chip Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCS_AW {
    #[doc = "14: NPCS0 as Chip Select"]
    NPCS0 = 14,
    #[doc = "13: NPCS1 as Chip Select"]
    NPCS1 = 13,
    #[doc = "11: NPCS2 as Chip Select"]
    NPCS2 = 11,
    #[doc = "7: NPCS3 as Chip Select"]
    NPCS3 = 7,
}
impl From<PCS_AW> for u8 {
    #[inline(always)]
    fn from(variant: PCS_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `PCS` writer - Peripheral Chip Select"]
pub struct PCS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "NPCS0 as Chip Select"]
    #[inline(always)]
    pub fn npcs0(self) -> &'a mut W {
        self.variant(PCS_AW::NPCS0)
    }
    #[doc = "NPCS1 as Chip Select"]
    #[inline(always)]
    pub fn npcs1(self) -> &'a mut W {
        self.variant(PCS_AW::NPCS1)
    }
    #[doc = "NPCS2 as Chip Select"]
    #[inline(always)]
    pub fn npcs2(self) -> &'a mut W {
        self.variant(PCS_AW::NPCS2)
    }
    #[doc = "NPCS3 as Chip Select"]
    #[inline(always)]
    pub fn npcs3(self) -> &'a mut W {
        self.variant(PCS_AW::NPCS3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub struct LASTXFER_W<'a> {
    w: &'a mut W,
}
impl<'a> LASTXFER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn td(&mut self) -> TD_W {
        TD_W { w: self }
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PCS_W {
        PCS_W { w: self }
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline(always)]
    pub fn lastxfer(&mut self) -> LASTXFER_W {
        LASTXFER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_tdr](index.html) module"]
pub struct SPI_TDR_SPEC;
impl crate::RegisterSpec for SPI_TDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [spi_tdr::W](W) writer structure"]
impl crate::Writable for SPI_TDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_TDR to value 0"]
impl crate::Resettable for SPI_TDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
