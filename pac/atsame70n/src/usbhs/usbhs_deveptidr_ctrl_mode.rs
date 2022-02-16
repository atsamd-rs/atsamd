#[doc = "Register `USBHS_DEVEPTIDR_CTRL_MODE[%s]` writer"]
pub struct W(crate::W<USBHS_DEVEPTIDR_CTRL_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_DEVEPTIDR_CTRL_MODE_SPEC>;
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
impl From<crate::W<USBHS_DEVEPTIDR_CTRL_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_DEVEPTIDR_CTRL_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINEC` writer - Transmitted IN Interrupt Clear"]
pub struct TXINEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINEC_W<'a> {
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
#[doc = "Field `RXOUTEC` writer - Received OUT Data Interrupt Clear"]
pub struct RXOUTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOUTEC_W<'a> {
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
#[doc = "Field `RXSTPEC` writer - Received SETUP Interrupt Clear"]
pub struct RXSTPEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTPEC_W<'a> {
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
#[doc = "Field `NAKOUTEC` writer - NAKed OUT Interrupt Clear"]
pub struct NAKOUTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKOUTEC_W<'a> {
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
#[doc = "Field `NAKINEC` writer - NAKed IN Interrupt Clear"]
pub struct NAKINEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKINEC_W<'a> {
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
#[doc = "Field `OVERFEC` writer - Overflow Interrupt Clear"]
pub struct OVERFEC_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFEC_W<'a> {
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
#[doc = "Field `STALLEDEC` writer - STALLed Interrupt Clear"]
pub struct STALLEDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLEDEC_W<'a> {
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
#[doc = "Field `SHORTPACKETEC` writer - Shortpacket Interrupt Clear"]
pub struct SHORTPACKETEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SHORTPACKETEC_W<'a> {
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
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Interrupt Clear"]
pub struct NBUSYBKEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NBUSYBKEC_W<'a> {
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
#[doc = "Field `FIFOCONC` writer - FIFO Control Clear"]
pub struct FIFOCONC_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOCONC_W<'a> {
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
#[doc = "Field `EPDISHDMAC` writer - Endpoint Interrupts Disable HDMA Request Clear"]
pub struct EPDISHDMAC_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDISHDMAC_W<'a> {
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
#[doc = "Field `NYETDISC` writer - NYET Token Disable Clear"]
pub struct NYETDISC_W<'a> {
    w: &'a mut W,
}
impl<'a> NYETDISC_W<'a> {
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
#[doc = "Field `STALLRQC` writer - STALL Request Clear"]
pub struct STALLRQC_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLRQC_W<'a> {
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
    #[doc = "Bit 0 - Transmitted IN Interrupt Clear"]
    #[inline(always)]
    pub fn txinec(&mut self) -> TXINEC_W {
        TXINEC_W { w: self }
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    pub fn rxoutec(&mut self) -> RXOUTEC_W {
        RXOUTEC_W { w: self }
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Clear"]
    #[inline(always)]
    pub fn rxstpec(&mut self) -> RXSTPEC_W {
        RXSTPEC_W { w: self }
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Clear"]
    #[inline(always)]
    pub fn nakoutec(&mut self) -> NAKOUTEC_W {
        NAKOUTEC_W { w: self }
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Clear"]
    #[inline(always)]
    pub fn nakinec(&mut self) -> NAKINEC_W {
        NAKINEC_W { w: self }
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn overfec(&mut self) -> OVERFEC_W {
        OVERFEC_W { w: self }
    }
    #[doc = "Bit 6 - STALLed Interrupt Clear"]
    #[inline(always)]
    pub fn stalledec(&mut self) -> STALLEDEC_W {
        STALLEDEC_W { w: self }
    }
    #[doc = "Bit 7 - Shortpacket Interrupt Clear"]
    #[inline(always)]
    pub fn shortpacketec(&mut self) -> SHORTPACKETEC_W {
        SHORTPACKETEC_W { w: self }
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Clear"]
    #[inline(always)]
    pub fn nbusybkec(&mut self) -> NBUSYBKEC_W {
        NBUSYBKEC_W { w: self }
    }
    #[doc = "Bit 14 - FIFO Control Clear"]
    #[inline(always)]
    pub fn fifoconc(&mut self) -> FIFOCONC_W {
        FIFOCONC_W { w: self }
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Clear"]
    #[inline(always)]
    pub fn epdishdmac(&mut self) -> EPDISHDMAC_W {
        EPDISHDMAC_W { w: self }
    }
    #[doc = "Bit 17 - NYET Token Disable Clear"]
    #[inline(always)]
    pub fn nyetdisc(&mut self) -> NYETDISC_W {
        NYETDISC_W { w: self }
    }
    #[doc = "Bit 19 - STALL Request Clear"]
    #[inline(always)]
    pub fn stallrqc(&mut self) -> STALLRQC_W {
        STALLRQC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptidr_ctrl_mode](index.html) module"]
pub struct USBHS_DEVEPTIDR_CTRL_MODE_SPEC;
impl crate::RegisterSpec for USBHS_DEVEPTIDR_CTRL_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbhs_deveptidr_ctrl_mode::W](W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIDR_CTRL_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_DEVEPTIDR_CTRL_MODE[%s]
to value 0"]
impl crate::Resettable for USBHS_DEVEPTIDR_CTRL_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
