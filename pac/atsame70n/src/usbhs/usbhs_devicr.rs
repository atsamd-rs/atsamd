#[doc = "Register `USBHS_DEVICR` writer"]
pub struct W(crate::W<USBHS_DEVICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_DEVICR_SPEC>;
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
impl From<crate::W<USBHS_DEVICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_DEVICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSPC` writer - Suspend Interrupt Clear"]
pub struct SUSPC_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPC_W<'a> {
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
#[doc = "Field `MSOFC` writer - Micro Start of Frame Interrupt Clear"]
pub struct MSOFC_W<'a> {
    w: &'a mut W,
}
impl<'a> MSOFC_W<'a> {
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
#[doc = "Field `SOFC` writer - Start of Frame Interrupt Clear"]
pub struct SOFC_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFC_W<'a> {
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
#[doc = "Field `EORSTC` writer - End of Reset Interrupt Clear"]
pub struct EORSTC_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSTC_W<'a> {
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
#[doc = "Field `WAKEUPC` writer - Wake-Up Interrupt Clear"]
pub struct WAKEUPC_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPC_W<'a> {
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
#[doc = "Field `EORSMC` writer - End of Resume Interrupt Clear"]
pub struct EORSMC_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSMC_W<'a> {
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
#[doc = "Field `UPRSMC` writer - Upstream Resume Interrupt Clear"]
pub struct UPRSMC_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRSMC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Clear"]
    #[inline(always)]
    pub fn suspc(&mut self) -> SUSPC_W {
        SUSPC_W { w: self }
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Clear"]
    #[inline(always)]
    pub fn msofc(&mut self) -> MSOFC_W {
        MSOFC_W { w: self }
    }
    #[doc = "Bit 2 - Start of Frame Interrupt Clear"]
    #[inline(always)]
    pub fn sofc(&mut self) -> SOFC_W {
        SOFC_W { w: self }
    }
    #[doc = "Bit 3 - End of Reset Interrupt Clear"]
    #[inline(always)]
    pub fn eorstc(&mut self) -> EORSTC_W {
        EORSTC_W { w: self }
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Clear"]
    #[inline(always)]
    pub fn wakeupc(&mut self) -> WAKEUPC_W {
        WAKEUPC_W { w: self }
    }
    #[doc = "Bit 5 - End of Resume Interrupt Clear"]
    #[inline(always)]
    pub fn eorsmc(&mut self) -> EORSMC_W {
        EORSMC_W { w: self }
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Clear"]
    #[inline(always)]
    pub fn uprsmc(&mut self) -> UPRSMC_W {
        UPRSMC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Global Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devicr](index.html) module"]
pub struct USBHS_DEVICR_SPEC;
impl crate::RegisterSpec for USBHS_DEVICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbhs_devicr::W](W) writer structure"]
impl crate::Writable for USBHS_DEVICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_DEVICR to value 0"]
impl crate::Resettable for USBHS_DEVICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
