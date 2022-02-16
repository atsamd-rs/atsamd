#[doc = "Register `HSMCI_CR` writer"]
pub struct W(crate::W<HSMCI_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSMCI_CR_SPEC>;
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
impl From<crate::W<HSMCI_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSMCI_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCIEN` writer - Multi-Media Interface Enable"]
pub struct MCIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCIEN_W<'a> {
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
#[doc = "Field `MCIDIS` writer - Multi-Media Interface Disable"]
pub struct MCIDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MCIDIS_W<'a> {
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
#[doc = "Field `PWSEN` writer - Power Save Mode Enable"]
pub struct PWSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWSEN_W<'a> {
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
#[doc = "Field `PWSDIS` writer - Power Save Mode Disable"]
pub struct PWSDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWSDIS_W<'a> {
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
#[doc = "Field `SWRST` writer - Software Reset"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
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
    #[doc = "Bit 0 - Multi-Media Interface Enable"]
    #[inline(always)]
    pub fn mcien(&mut self) -> MCIEN_W {
        MCIEN_W { w: self }
    }
    #[doc = "Bit 1 - Multi-Media Interface Disable"]
    #[inline(always)]
    pub fn mcidis(&mut self) -> MCIDIS_W {
        MCIDIS_W { w: self }
    }
    #[doc = "Bit 2 - Power Save Mode Enable"]
    #[inline(always)]
    pub fn pwsen(&mut self) -> PWSEN_W {
        PWSEN_W { w: self }
    }
    #[doc = "Bit 3 - Power Save Mode Disable"]
    #[inline(always)]
    pub fn pwsdis(&mut self) -> PWSDIS_W {
        PWSDIS_W { w: self }
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_cr](index.html) module"]
pub struct HSMCI_CR_SPEC;
impl crate::RegisterSpec for HSMCI_CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hsmci_cr::W](W) writer structure"]
impl crate::Writable for HSMCI_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSMCI_CR to value 0"]
impl crate::Resettable for HSMCI_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
