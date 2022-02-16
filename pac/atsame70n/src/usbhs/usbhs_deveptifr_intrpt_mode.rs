#[doc = "Register `USBHS_DEVEPTIFR_INTRPT_MODE[%s]` writer"]
pub struct W(crate::W<USBHS_DEVEPTIFR_INTRPT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_DEVEPTIFR_INTRPT_MODE_SPEC>;
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
impl From<crate::W<USBHS_DEVEPTIFR_INTRPT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_DEVEPTIFR_INTRPT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINIS` writer - Transmitted IN Data Interrupt Set"]
pub struct TXINIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINIS_W<'a> {
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
#[doc = "Field `RXOUTIS` writer - Received OUT Data Interrupt Set"]
pub struct RXOUTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOUTIS_W<'a> {
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
#[doc = "Field `RXSTPIS` writer - Received SETUP Interrupt Set"]
pub struct RXSTPIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTPIS_W<'a> {
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
#[doc = "Field `NAKOUTIS` writer - NAKed OUT Interrupt Set"]
pub struct NAKOUTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKOUTIS_W<'a> {
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
#[doc = "Field `NAKINIS` writer - NAKed IN Interrupt Set"]
pub struct NAKINIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKINIS_W<'a> {
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
#[doc = "Field `STALLEDIS` writer - STALLed Interrupt Set"]
pub struct STALLEDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLEDIS_W<'a> {
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
#[doc = "Field `SHORTPACKETS` writer - Short Packet Interrupt Set"]
pub struct SHORTPACKETS_W<'a> {
    w: &'a mut W,
}
impl<'a> SHORTPACKETS_W<'a> {
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
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Interrupt Set"]
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
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Set"]
    #[inline(always)]
    pub fn txinis(&mut self) -> TXINIS_W {
        TXINIS_W { w: self }
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Set"]
    #[inline(always)]
    pub fn rxoutis(&mut self) -> RXOUTIS_W {
        RXOUTIS_W { w: self }
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Set"]
    #[inline(always)]
    pub fn rxstpis(&mut self) -> RXSTPIS_W {
        RXSTPIS_W { w: self }
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Set"]
    #[inline(always)]
    pub fn nakoutis(&mut self) -> NAKOUTIS_W {
        NAKOUTIS_W { w: self }
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Set"]
    #[inline(always)]
    pub fn nakinis(&mut self) -> NAKINIS_W {
        NAKINIS_W { w: self }
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    pub fn overfis(&mut self) -> OVERFIS_W {
        OVERFIS_W { w: self }
    }
    #[doc = "Bit 6 - STALLed Interrupt Set"]
    #[inline(always)]
    pub fn stalledis(&mut self) -> STALLEDIS_W {
        STALLEDIS_W { w: self }
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    pub fn shortpackets(&mut self) -> SHORTPACKETS_W {
        SHORTPACKETS_W { w: self }
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Set"]
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
#[doc = "Device Endpoint Interrupt Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptifr_intrpt_mode](index.html) module"]
pub struct USBHS_DEVEPTIFR_INTRPT_MODE_SPEC;
impl crate::RegisterSpec for USBHS_DEVEPTIFR_INTRPT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbhs_deveptifr_intrpt_mode::W](W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIFR_INTRPT_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_DEVEPTIFR_INTRPT_MODE[%s]
to value 0"]
impl crate::Resettable for USBHS_DEVEPTIFR_INTRPT_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
