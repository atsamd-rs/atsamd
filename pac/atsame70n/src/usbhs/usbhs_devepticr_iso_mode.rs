#[doc = "Register `USBHS_DEVEPTICR_ISO_MODE[%s]` writer"]
pub struct W(crate::W<USBHS_DEVEPTICR_ISO_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_DEVEPTICR_ISO_MODE_SPEC>;
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
impl From<crate::W<USBHS_DEVEPTICR_ISO_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_DEVEPTICR_ISO_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINIC` writer - Transmitted IN Data Interrupt Clear"]
pub struct TXINIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINIC_W<'a> {
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
#[doc = "Field `RXOUTIC` writer - Received OUT Data Interrupt Clear"]
pub struct RXOUTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOUTIC_W<'a> {
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
#[doc = "Field `UNDERFIC` writer - Underflow Interrupt Clear"]
pub struct UNDERFIC_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFIC_W<'a> {
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
#[doc = "Field `HBISOINERRIC` writer - High Bandwidth Isochronous IN Underflow Error Interrupt Clear"]
pub struct HBISOINERRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> HBISOINERRIC_W<'a> {
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
#[doc = "Field `HBISOFLUSHIC` writer - High Bandwidth Isochronous IN Flush Interrupt Clear"]
pub struct HBISOFLUSHIC_W<'a> {
    w: &'a mut W,
}
impl<'a> HBISOFLUSHIC_W<'a> {
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
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub struct OVERFIC_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFIC_W<'a> {
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
#[doc = "Field `CRCERRIC` writer - CRC Error Interrupt Clear"]
pub struct CRCERRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCERRIC_W<'a> {
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
#[doc = "Field `SHORTPACKETC` writer - Short Packet Interrupt Clear"]
pub struct SHORTPACKETC_W<'a> {
    w: &'a mut W,
}
impl<'a> SHORTPACKETC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Clear"]
    #[inline(always)]
    pub fn txinic(&mut self) -> TXINIC_W {
        TXINIC_W { w: self }
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    pub fn rxoutic(&mut self) -> RXOUTIC_W {
        RXOUTIC_W { w: self }
    }
    #[doc = "Bit 2 - Underflow Interrupt Clear"]
    #[inline(always)]
    pub fn underfic(&mut self) -> UNDERFIC_W {
        UNDERFIC_W { w: self }
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Underflow Error Interrupt Clear"]
    #[inline(always)]
    pub fn hbisoinerric(&mut self) -> HBISOINERRIC_W {
        HBISOINERRIC_W { w: self }
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Clear"]
    #[inline(always)]
    pub fn hbisoflushic(&mut self) -> HBISOFLUSHIC_W {
        HBISOFLUSHIC_W { w: self }
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn overfic(&mut self) -> OVERFIC_W {
        OVERFIC_W { w: self }
    }
    #[doc = "Bit 6 - CRC Error Interrupt Clear"]
    #[inline(always)]
    pub fn crcerric(&mut self) -> CRCERRIC_W {
        CRCERRIC_W { w: self }
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    pub fn shortpacketc(&mut self) -> SHORTPACKETC_W {
        SHORTPACKETC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devepticr_iso_mode](index.html) module"]
pub struct USBHS_DEVEPTICR_ISO_MODE_SPEC;
impl crate::RegisterSpec for USBHS_DEVEPTICR_ISO_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbhs_devepticr_iso_mode::W](W) writer structure"]
impl crate::Writable for USBHS_DEVEPTICR_ISO_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_DEVEPTICR_ISO_MODE[%s]
to value 0"]
impl crate::Resettable for USBHS_DEVEPTICR_ISO_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
