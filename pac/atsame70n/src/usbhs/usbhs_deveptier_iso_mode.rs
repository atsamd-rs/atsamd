#[doc = "Register `USBHS_DEVEPTIER_ISO_MODE[%s]` writer"]
pub struct W(crate::W<USBHS_DEVEPTIER_ISO_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_DEVEPTIER_ISO_MODE_SPEC>;
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
impl From<crate::W<USBHS_DEVEPTIER_ISO_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_DEVEPTIER_ISO_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINES` writer - Transmitted IN Data Interrupt Enable"]
pub struct TXINES_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINES_W<'a> {
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
#[doc = "Field `RXOUTES` writer - Received OUT Data Interrupt Enable"]
pub struct RXOUTES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOUTES_W<'a> {
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
#[doc = "Field `UNDERFES` writer - Underflow Interrupt Enable"]
pub struct UNDERFES_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFES_W<'a> {
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
#[doc = "Field `HBISOINERRES` writer - High Bandwidth Isochronous IN Underflow Error Interrupt Enable"]
pub struct HBISOINERRES_W<'a> {
    w: &'a mut W,
}
impl<'a> HBISOINERRES_W<'a> {
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
#[doc = "Field `HBISOFLUSHES` writer - High Bandwidth Isochronous IN Flush Interrupt Enable"]
pub struct HBISOFLUSHES_W<'a> {
    w: &'a mut W,
}
impl<'a> HBISOFLUSHES_W<'a> {
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
#[doc = "Field `OVERFES` writer - Overflow Interrupt Enable"]
pub struct OVERFES_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFES_W<'a> {
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
#[doc = "Field `CRCERRES` writer - CRC Error Interrupt Enable"]
pub struct CRCERRES_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCERRES_W<'a> {
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
#[doc = "Field `SHORTPACKETES` writer - Short Packet Interrupt Enable"]
pub struct SHORTPACKETES_W<'a> {
    w: &'a mut W,
}
impl<'a> SHORTPACKETES_W<'a> {
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
#[doc = "Field `MDATAES` writer - MData Interrupt Enable"]
pub struct MDATAES_W<'a> {
    w: &'a mut W,
}
impl<'a> MDATAES_W<'a> {
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
#[doc = "Field `DATAXES` writer - DataX Interrupt Enable"]
pub struct DATAXES_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAXES_W<'a> {
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
#[doc = "Field `ERRORTRANSES` writer - Transaction Error Interrupt Enable"]
pub struct ERRORTRANSES_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRORTRANSES_W<'a> {
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
#[doc = "Field `NBUSYBKES` writer - Number of Busy Banks Interrupt Enable"]
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
#[doc = "Field `KILLBKS` writer - Kill IN Bank"]
pub struct KILLBKS_W<'a> {
    w: &'a mut W,
}
impl<'a> KILLBKS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `FIFOCONS` writer - FIFO Control"]
pub struct FIFOCONS_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOCONS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `EPDISHDMAS` writer - Endpoint Interrupts Disable HDMA Request Enable"]
pub struct EPDISHDMAS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDISHDMAS_W<'a> {
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
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Enable"]
    #[inline(always)]
    pub fn txines(&mut self) -> TXINES_W {
        TXINES_W { w: self }
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Enable"]
    #[inline(always)]
    pub fn rxoutes(&mut self) -> RXOUTES_W {
        RXOUTES_W { w: self }
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn underfes(&mut self) -> UNDERFES_W {
        UNDERFES_W { w: self }
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Underflow Error Interrupt Enable"]
    #[inline(always)]
    pub fn hbisoinerres(&mut self) -> HBISOINERRES_W {
        HBISOINERRES_W { w: self }
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Enable"]
    #[inline(always)]
    pub fn hbisoflushes(&mut self) -> HBISOFLUSHES_W {
        HBISOFLUSHES_W { w: self }
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overfes(&mut self) -> OVERFES_W {
        OVERFES_W { w: self }
    }
    #[doc = "Bit 6 - CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn crcerres(&mut self) -> CRCERRES_W {
        CRCERRES_W { w: self }
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    pub fn shortpacketes(&mut self) -> SHORTPACKETES_W {
        SHORTPACKETES_W { w: self }
    }
    #[doc = "Bit 8 - MData Interrupt Enable"]
    #[inline(always)]
    pub fn mdataes(&mut self) -> MDATAES_W {
        MDATAES_W { w: self }
    }
    #[doc = "Bit 9 - DataX Interrupt Enable"]
    #[inline(always)]
    pub fn dataxes(&mut self) -> DATAXES_W {
        DATAXES_W { w: self }
    }
    #[doc = "Bit 10 - Transaction Error Interrupt Enable"]
    #[inline(always)]
    pub fn errortranses(&mut self) -> ERRORTRANSES_W {
        ERRORTRANSES_W { w: self }
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Enable"]
    #[inline(always)]
    pub fn nbusybkes(&mut self) -> NBUSYBKES_W {
        NBUSYBKES_W { w: self }
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    pub fn killbks(&mut self) -> KILLBKS_W {
        KILLBKS_W { w: self }
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocons(&mut self) -> FIFOCONS_W {
        FIFOCONS_W { w: self }
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    pub fn epdishdmas(&mut self) -> EPDISHDMAS_W {
        EPDISHDMAS_W { w: self }
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
#[doc = "Device Endpoint Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptier_iso_mode](index.html) module"]
pub struct USBHS_DEVEPTIER_ISO_MODE_SPEC;
impl crate::RegisterSpec for USBHS_DEVEPTIER_ISO_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbhs_deveptier_iso_mode::W](W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIER_ISO_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_DEVEPTIER_ISO_MODE[%s]
to value 0"]
impl crate::Resettable for USBHS_DEVEPTIER_ISO_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
