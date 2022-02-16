#[doc = "Register `USBHS_HSTPIPIFR_INTRPT_MODE[%s]` writer"]
pub struct W(crate::W<USBHS_HSTPIPIFR_INTRPT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_HSTPIPIFR_INTRPT_MODE_SPEC>;
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
impl From<crate::W<USBHS_HSTPIPIFR_INTRPT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_HSTPIPIFR_INTRPT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINIS` writer - Received IN Data Interrupt Set"]
pub struct RXINIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINIS_W<'a> {
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
#[doc = "Field `TXOUTIS` writer - Transmitted OUT Data Interrupt Set"]
pub struct TXOUTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOUTIS_W<'a> {
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
#[doc = "Field `UNDERFIS` writer - Underflow Interrupt Set"]
pub struct UNDERFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFIS_W<'a> {
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
#[doc = "Field `PERRIS` writer - Pipe Error Interrupt Set"]
pub struct PERRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PERRIS_W<'a> {
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
#[doc = "Field `NAKEDIS` writer - NAKed Interrupt Set"]
pub struct NAKEDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKEDIS_W<'a> {
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
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub struct OVERFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFIS_W<'a> {
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
#[doc = "Field `RXSTALLDIS` writer - Received STALLed Interrupt Set"]
pub struct RXSTALLDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTALLDIS_W<'a> {
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
#[doc = "Field `SHORTPACKETIS` writer - Short Packet Interrupt Set"]
pub struct SHORTPACKETIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SHORTPACKETIS_W<'a> {
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
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Set"]
pub struct NBUSYBKS_W<'a> {
    w: &'a mut W,
}
impl<'a> NBUSYBKS_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Set"]
    #[inline(always)]
    pub fn rxinis(&mut self) -> RXINIS_W {
        RXINIS_W { w: self }
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Set"]
    #[inline(always)]
    pub fn txoutis(&mut self) -> TXOUTIS_W {
        TXOUTIS_W { w: self }
    }
    #[doc = "Bit 2 - Underflow Interrupt Set"]
    #[inline(always)]
    pub fn underfis(&mut self) -> UNDERFIS_W {
        UNDERFIS_W { w: self }
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Set"]
    #[inline(always)]
    pub fn perris(&mut self) -> PERRIS_W {
        PERRIS_W { w: self }
    }
    #[doc = "Bit 4 - NAKed Interrupt Set"]
    #[inline(always)]
    pub fn nakedis(&mut self) -> NAKEDIS_W {
        NAKEDIS_W { w: self }
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    pub fn overfis(&mut self) -> OVERFIS_W {
        OVERFIS_W { w: self }
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Set"]
    #[inline(always)]
    pub fn rxstalldis(&mut self) -> RXSTALLDIS_W {
        RXSTALLDIS_W { w: self }
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    pub fn shortpacketis(&mut self) -> SHORTPACKETIS_W {
        SHORTPACKETIS_W { w: self }
    }
    #[doc = "Bit 12 - Number of Busy Banks Set"]
    #[inline(always)]
    pub fn nbusybks(&mut self) -> NBUSYBKS_W {
        NBUSYBKS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Pipe Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipifr_intrpt_mode](index.html) module"]
pub struct USBHS_HSTPIPIFR_INTRPT_MODE_SPEC;
impl crate::RegisterSpec for USBHS_HSTPIPIFR_INTRPT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipifr_intrpt_mode::W](W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIFR_INTRPT_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_HSTPIPIFR_INTRPT_MODE[%s]
to value 0"]
impl crate::Resettable for USBHS_HSTPIPIFR_INTRPT_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
