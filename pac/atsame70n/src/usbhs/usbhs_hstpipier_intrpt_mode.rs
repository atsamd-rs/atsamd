#[doc = "Register `USBHS_HSTPIPIER_INTRPT_MODE[%s]` writer"]
pub struct W(crate::W<USBHS_HSTPIPIER_INTRPT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_HSTPIPIER_INTRPT_MODE_SPEC>;
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
impl From<crate::W<USBHS_HSTPIPIER_INTRPT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_HSTPIPIER_INTRPT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINES` writer - Received IN Data Interrupt Enable"]
pub struct RXINES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINES_W<'a> {
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
#[doc = "Field `TXOUTES` writer - Transmitted OUT Data Interrupt Enable"]
pub struct TXOUTES_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOUTES_W<'a> {
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
#[doc = "Field `UNDERFIES` writer - Underflow Interrupt Enable"]
pub struct UNDERFIES_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFIES_W<'a> {
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
#[doc = "Field `PERRES` writer - Pipe Error Interrupt Enable"]
pub struct PERRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PERRES_W<'a> {
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
#[doc = "Field `NAKEDES` writer - NAKed Interrupt Enable"]
pub struct NAKEDES_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKEDES_W<'a> {
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
#[doc = "Field `OVERFIES` writer - Overflow Interrupt Enable"]
pub struct OVERFIES_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFIES_W<'a> {
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
#[doc = "Field `RXSTALLDES` writer - Received STALLed Interrupt Enable"]
pub struct RXSTALLDES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTALLDES_W<'a> {
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
#[doc = "Field `SHORTPACKETIES` writer - Short Packet Interrupt Enable"]
pub struct SHORTPACKETIES_W<'a> {
    w: &'a mut W,
}
impl<'a> SHORTPACKETIES_W<'a> {
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
#[doc = "Field `NBUSYBKES` writer - Number of Busy Banks Enable"]
pub struct NBUSYBKES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBUSYBKES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `PDISHDMAS` writer - Pipe Interrupts Disable HDMA Request Enable"]
pub struct PDISHDMAS_W<'a> {
    w: &'a mut W,
}
impl<'a> PDISHDMAS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PFREEZES` writer - Pipe Freeze Enable"]
pub struct PFREEZES_W<'a> {
    w: &'a mut W,
}
impl<'a> PFREEZES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `RSTDTS` writer - Reset Data Toggle Enable"]
pub struct RSTDTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTDTS_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Enable"]
    #[inline(always)]
    pub fn rxines(&mut self) -> RXINES_W {
        RXINES_W { w: self }
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Enable"]
    #[inline(always)]
    pub fn txoutes(&mut self) -> TXOUTES_W {
        TXOUTES_W { w: self }
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn underfies(&mut self) -> UNDERFIES_W {
        UNDERFIES_W { w: self }
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Enable"]
    #[inline(always)]
    pub fn perres(&mut self) -> PERRES_W {
        PERRES_W { w: self }
    }
    #[doc = "Bit 4 - NAKed Interrupt Enable"]
    #[inline(always)]
    pub fn nakedes(&mut self) -> NAKEDES_W {
        NAKEDES_W { w: self }
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overfies(&mut self) -> OVERFIES_W {
        OVERFIES_W { w: self }
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Enable"]
    #[inline(always)]
    pub fn rxstalldes(&mut self) -> RXSTALLDES_W {
        RXSTALLDES_W { w: self }
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    pub fn shortpacketies(&mut self) -> SHORTPACKETIES_W {
        SHORTPACKETIES_W { w: self }
    }
    #[doc = "Bit 12 - Number of Busy Banks Enable"]
    #[inline(always)]
    pub fn nbusybkes(&mut self) -> NBUSYBKES_W {
        NBUSYBKES_W { w: self }
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    pub fn pdishdmas(&mut self) -> PDISHDMAS_W {
        PDISHDMAS_W { w: self }
    }
    #[doc = "Bit 17 - Pipe Freeze Enable"]
    #[inline(always)]
    pub fn pfreezes(&mut self) -> PFREEZES_W {
        PFREEZES_W { w: self }
    }
    #[doc = "Bit 18 - Reset Data Toggle Enable"]
    #[inline(always)]
    pub fn rstdts(&mut self) -> RSTDTS_W {
        RSTDTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Pipe Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipier_intrpt_mode](index.html) module"]
pub struct USBHS_HSTPIPIER_INTRPT_MODE_SPEC;
impl crate::RegisterSpec for USBHS_HSTPIPIER_INTRPT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipier_intrpt_mode::W](W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIER_INTRPT_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_HSTPIPIER_INTRPT_MODE[%s]
to value 0"]
impl crate::Resettable for USBHS_HSTPIPIER_INTRPT_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
