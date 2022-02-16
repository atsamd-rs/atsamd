#[doc = "Register `PWM_CMPVUPD` writer"]
pub struct W(crate::W<PWM_CMPVUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CMPVUPD_SPEC>;
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
impl From<crate::W<PWM_CMPVUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CMPVUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVUPD` writer - Comparison x Value Update"]
pub struct CVUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CVUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Field `CVMUPD` writer - Comparison x Value Mode Update"]
pub struct CVMUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CVMUPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:23 - Comparison x Value Update"]
    #[inline(always)]
    pub fn cvupd(&mut self) -> CVUPD_W {
        CVUPD_W { w: self }
    }
    #[doc = "Bit 24 - Comparison x Value Mode Update"]
    #[inline(always)]
    pub fn cvmupd(&mut self) -> CVMUPD_W {
        CVMUPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Comparison 0 Value Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmpvupd](index.html) module"]
pub struct PWM_CMPVUPD_SPEC;
impl crate::RegisterSpec for PWM_CMPVUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwm_cmpvupd::W](W) writer structure"]
impl crate::Writable for PWM_CMPVUPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CMPVUPD to value 0"]
impl crate::Resettable for PWM_CMPVUPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
