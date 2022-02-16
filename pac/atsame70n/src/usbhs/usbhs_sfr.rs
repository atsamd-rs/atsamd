#[doc = "Register `USBHS_SFR` writer"]
pub struct W(crate::W<USBHS_SFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_SFR_SPEC>;
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
impl From<crate::W<USBHS_SFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_SFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDERRIS` writer - Remote Device Connection Error Interrupt Set"]
pub struct RDERRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RDERRIS_W<'a> {
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
#[doc = "Field `VBUSRQS` writer - VBUS Request Set"]
pub struct VBUSRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSRQS_W<'a> {
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
impl W {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Set"]
    #[inline(always)]
    pub fn rderris(&mut self) -> RDERRIS_W {
        RDERRIS_W { w: self }
    }
    #[doc = "Bit 9 - VBUS Request Set"]
    #[inline(always)]
    pub fn vbusrqs(&mut self) -> VBUSRQS_W {
        VBUSRQS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Status Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_sfr](index.html) module"]
pub struct USBHS_SFR_SPEC;
impl crate::RegisterSpec for USBHS_SFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbhs_sfr::W](W) writer structure"]
impl crate::Writable for USBHS_SFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_SFR to value 0"]
impl crate::Resettable for USBHS_SFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
