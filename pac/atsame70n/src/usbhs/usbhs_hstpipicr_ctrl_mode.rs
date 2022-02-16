#[doc = "Register `USBHS_HSTPIPICR_CTRL_MODE[%s]` writer"]
pub struct W(crate::W<USBHS_HSTPIPICR_CTRL_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_HSTPIPICR_CTRL_MODE_SPEC>;
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
impl From<crate::W<USBHS_HSTPIPICR_CTRL_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_HSTPIPICR_CTRL_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINIC` writer - Received IN Data Interrupt Clear"]
pub struct RXINIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINIC_W<'a> {
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
#[doc = "Field `TXOUTIC` writer - Transmitted OUT Data Interrupt Clear"]
pub struct TXOUTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOUTIC_W<'a> {
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
#[doc = "Field `TXSTPIC` writer - Transmitted SETUP Interrupt Clear"]
pub struct TXSTPIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTPIC_W<'a> {
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
#[doc = "Field `NAKEDIC` writer - NAKed Interrupt Clear"]
pub struct NAKEDIC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKEDIC_W<'a> {
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
#[doc = "Field `RXSTALLDIC` writer - Received STALLed Interrupt Clear"]
pub struct RXSTALLDIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTALLDIC_W<'a> {
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
#[doc = "Field `SHORTPACKETIC` writer - Short Packet Interrupt Clear"]
pub struct SHORTPACKETIC_W<'a> {
    w: &'a mut W,
}
impl<'a> SHORTPACKETIC_W<'a> {
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
    #[doc = "Bit 0 - Received IN Data Interrupt Clear"]
    #[inline(always)]
    pub fn rxinic(&mut self) -> RXINIC_W {
        RXINIC_W { w: self }
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Clear"]
    #[inline(always)]
    pub fn txoutic(&mut self) -> TXOUTIC_W {
        TXOUTIC_W { w: self }
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Clear"]
    #[inline(always)]
    pub fn txstpic(&mut self) -> TXSTPIC_W {
        TXSTPIC_W { w: self }
    }
    #[doc = "Bit 4 - NAKed Interrupt Clear"]
    #[inline(always)]
    pub fn nakedic(&mut self) -> NAKEDIC_W {
        NAKEDIC_W { w: self }
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn overfic(&mut self) -> OVERFIC_W {
        OVERFIC_W { w: self }
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Clear"]
    #[inline(always)]
    pub fn rxstalldic(&mut self) -> RXSTALLDIC_W {
        RXSTALLDIC_W { w: self }
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    pub fn shortpacketic(&mut self) -> SHORTPACKETIC_W {
        SHORTPACKETIC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Pipe Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipicr_ctrl_mode](index.html) module"]
pub struct USBHS_HSTPIPICR_CTRL_MODE_SPEC;
impl crate::RegisterSpec for USBHS_HSTPIPICR_CTRL_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipicr_ctrl_mode::W](W) writer structure"]
impl crate::Writable for USBHS_HSTPIPICR_CTRL_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_HSTPIPICR_CTRL_MODE[%s]
to value 0"]
impl crate::Resettable for USBHS_HSTPIPICR_CTRL_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
