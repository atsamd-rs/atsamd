#[doc = "Register `PWM_CMPMUPD` writer"]
pub struct W(crate::W<PWM_CMPMUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CMPMUPD_SPEC>;
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
impl From<crate::W<PWM_CMPMUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CMPMUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CENUPD` writer - Comparison x Enable Update"]
pub struct CENUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CENUPD_W<'a> {
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
#[doc = "Field `CTRUPD` writer - Comparison x Trigger Update"]
pub struct CTRUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CPRUPD` writer - Comparison x Period Update"]
pub struct CPRUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CPRUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `CUPRUPD` writer - Comparison x Update Period Update"]
pub struct CUPRUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CUPRUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Comparison x Enable Update"]
    #[inline(always)]
    pub fn cenupd(&mut self) -> CENUPD_W {
        CENUPD_W { w: self }
    }
    #[doc = "Bits 4:7 - Comparison x Trigger Update"]
    #[inline(always)]
    pub fn ctrupd(&mut self) -> CTRUPD_W {
        CTRUPD_W { w: self }
    }
    #[doc = "Bits 8:11 - Comparison x Period Update"]
    #[inline(always)]
    pub fn cprupd(&mut self) -> CPRUPD_W {
        CPRUPD_W { w: self }
    }
    #[doc = "Bits 16:19 - Comparison x Update Period Update"]
    #[inline(always)]
    pub fn cuprupd(&mut self) -> CUPRUPD_W {
        CUPRUPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Comparison 0 Mode Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmpmupd](index.html) module"]
pub struct PWM_CMPMUPD_SPEC;
impl crate::RegisterSpec for PWM_CMPMUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwm_cmpmupd::W](W) writer structure"]
impl crate::Writable for PWM_CMPMUPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CMPMUPD to value 0"]
impl crate::Resettable for PWM_CMPMUPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
