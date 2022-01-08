#[doc = "Register `DCRSR` writer"]
pub struct W(crate::W<DCRSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCRSR_SPEC>;
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
impl From<crate::W<DCRSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCRSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGSEL` writer - "]
pub struct REGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `REGWnR` writer - "]
pub struct REGWNR_W<'a> {
    w: &'a mut W,
}
impl<'a> REGWNR_W<'a> {
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
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn regsel(&mut self) -> REGSEL_W {
        REGSEL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn regwn_r(&mut self) -> REGWNR_W {
        REGWNR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Core Register Selector Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrsr](index.html) module"]
pub struct DCRSR_SPEC;
impl crate::RegisterSpec for DCRSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dcrsr::W](W) writer structure"]
impl crate::Writable for DCRSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCRSR to value 0"]
impl crate::Resettable for DCRSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
