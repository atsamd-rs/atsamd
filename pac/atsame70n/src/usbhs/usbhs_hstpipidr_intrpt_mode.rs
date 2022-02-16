#[doc = "Register `USBHS_HSTPIPIDR_INTRPT_MODE[%s]` writer"]
pub struct W(crate::W<USBHS_HSTPIPIDR_INTRPT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_HSTPIPIDR_INTRPT_MODE_SPEC>;
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
impl From<crate::W<USBHS_HSTPIPIDR_INTRPT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_HSTPIPIDR_INTRPT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINEC` writer - Received IN Data Interrupt Disable"]
pub struct RXINEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINEC_W<'a> {
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
#[doc = "Field `TXOUTEC` writer - Transmitted OUT Data Interrupt Disable"]
pub struct TXOUTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOUTEC_W<'a> {
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
#[doc = "Field `UNDERFIEC` writer - Underflow Interrupt Disable"]
pub struct UNDERFIEC_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFIEC_W<'a> {
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
#[doc = "Field `PERREC` writer - Pipe Error Interrupt Disable"]
pub struct PERREC_W<'a> {
    w: &'a mut W,
}
impl<'a> PERREC_W<'a> {
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
#[doc = "Field `NAKEDEC` writer - NAKed Interrupt Disable"]
pub struct NAKEDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKEDEC_W<'a> {
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
#[doc = "Field `OVERFIEC` writer - Overflow Interrupt Disable"]
pub struct OVERFIEC_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFIEC_W<'a> {
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
#[doc = "Field `RXSTALLDEC` writer - Received STALLed Interrupt Disable"]
pub struct RXSTALLDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTALLDEC_W<'a> {
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
#[doc = "Field `SHORTPACKETIEC` writer - Short Packet Interrupt Disable"]
pub struct SHORTPACKETIEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SHORTPACKETIEC_W<'a> {
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
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Disable"]
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
#[doc = "Field `FIFOCONC` writer - FIFO Control Disable"]
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
#[doc = "Field `PDISHDMAC` writer - Pipe Interrupts Disable HDMA Request Disable"]
pub struct PDISHDMAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDISHDMAC_W<'a> {
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
#[doc = "Field `PFREEZEC` writer - Pipe Freeze Disable"]
pub struct PFREEZEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PFREEZEC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Disable"]
    #[inline(always)]
    pub fn rxinec(&mut self) -> RXINEC_W {
        RXINEC_W { w: self }
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Disable"]
    #[inline(always)]
    pub fn txoutec(&mut self) -> TXOUTEC_W {
        TXOUTEC_W { w: self }
    }
    #[doc = "Bit 2 - Underflow Interrupt Disable"]
    #[inline(always)]
    pub fn underfiec(&mut self) -> UNDERFIEC_W {
        UNDERFIEC_W { w: self }
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Disable"]
    #[inline(always)]
    pub fn perrec(&mut self) -> PERREC_W {
        PERREC_W { w: self }
    }
    #[doc = "Bit 4 - NAKed Interrupt Disable"]
    #[inline(always)]
    pub fn nakedec(&mut self) -> NAKEDEC_W {
        NAKEDEC_W { w: self }
    }
    #[doc = "Bit 5 - Overflow Interrupt Disable"]
    #[inline(always)]
    pub fn overfiec(&mut self) -> OVERFIEC_W {
        OVERFIEC_W { w: self }
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Disable"]
    #[inline(always)]
    pub fn rxstalldec(&mut self) -> RXSTALLDEC_W {
        RXSTALLDEC_W { w: self }
    }
    #[doc = "Bit 7 - Short Packet Interrupt Disable"]
    #[inline(always)]
    pub fn shortpacketiec(&mut self) -> SHORTPACKETIEC_W {
        SHORTPACKETIEC_W { w: self }
    }
    #[doc = "Bit 12 - Number of Busy Banks Disable"]
    #[inline(always)]
    pub fn nbusybkec(&mut self) -> NBUSYBKEC_W {
        NBUSYBKEC_W { w: self }
    }
    #[doc = "Bit 14 - FIFO Control Disable"]
    #[inline(always)]
    pub fn fifoconc(&mut self) -> FIFOCONC_W {
        FIFOCONC_W { w: self }
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Disable"]
    #[inline(always)]
    pub fn pdishdmac(&mut self) -> PDISHDMAC_W {
        PDISHDMAC_W { w: self }
    }
    #[doc = "Bit 17 - Pipe Freeze Disable"]
    #[inline(always)]
    pub fn pfreezec(&mut self) -> PFREEZEC_W {
        PFREEZEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Pipe Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipidr_intrpt_mode](index.html) module"]
pub struct USBHS_HSTPIPIDR_INTRPT_MODE_SPEC;
impl crate::RegisterSpec for USBHS_HSTPIPIDR_INTRPT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipidr_intrpt_mode::W](W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIDR_INTRPT_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_HSTPIPIDR_INTRPT_MODE[%s]
to value 0"]
impl crate::Resettable for USBHS_HSTPIPIDR_INTRPT_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
