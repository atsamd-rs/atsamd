#[doc = "Register `USBHS_DEVEPTICR_INTRPT_MODE[%s]` writer"]
pub struct W(crate::W<USBHS_DEVEPTICR_INTRPT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_DEVEPTICR_INTRPT_MODE_SPEC>;
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
impl From<crate::W<USBHS_DEVEPTICR_INTRPT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_DEVEPTICR_INTRPT_MODE_SPEC>) -> Self {
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
#[doc = "Field `RXSTPIC` writer - Received SETUP Interrupt Clear"]
pub struct RXSTPIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTPIC_W<'a> {
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
#[doc = "Field `NAKOUTIC` writer - NAKed OUT Interrupt Clear"]
pub struct NAKOUTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKOUTIC_W<'a> {
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
#[doc = "Field `NAKINIC` writer - NAKed IN Interrupt Clear"]
pub struct NAKINIC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKINIC_W<'a> {
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
#[doc = "Field `STALLEDIC` writer - STALLed Interrupt Clear"]
pub struct STALLEDIC_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLEDIC_W<'a> {
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
    #[doc = "Bit 2 - Received SETUP Interrupt Clear"]
    #[inline(always)]
    pub fn rxstpic(&mut self) -> RXSTPIC_W {
        RXSTPIC_W { w: self }
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Clear"]
    #[inline(always)]
    pub fn nakoutic(&mut self) -> NAKOUTIC_W {
        NAKOUTIC_W { w: self }
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Clear"]
    #[inline(always)]
    pub fn nakinic(&mut self) -> NAKINIC_W {
        NAKINIC_W { w: self }
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn overfic(&mut self) -> OVERFIC_W {
        OVERFIC_W { w: self }
    }
    #[doc = "Bit 6 - STALLed Interrupt Clear"]
    #[inline(always)]
    pub fn stalledic(&mut self) -> STALLEDIC_W {
        STALLEDIC_W { w: self }
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
#[doc = "Device Endpoint Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devepticr_intrpt_mode](index.html) module"]
pub struct USBHS_DEVEPTICR_INTRPT_MODE_SPEC;
impl crate::RegisterSpec for USBHS_DEVEPTICR_INTRPT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbhs_devepticr_intrpt_mode::W](W) writer structure"]
impl crate::Writable for USBHS_DEVEPTICR_INTRPT_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_DEVEPTICR_INTRPT_MODE[%s]
to value 0"]
impl crate::Resettable for USBHS_DEVEPTICR_INTRPT_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
