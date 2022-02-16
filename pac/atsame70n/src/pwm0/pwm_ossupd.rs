#[doc = "Register `PWM_OSSUPD` writer"]
pub struct W(crate::W<PWM_OSSUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_OSSUPD_SPEC>;
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
impl From<crate::W<PWM_OSSUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_OSSUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSSUPH0` writer - Output Selection Set for PWMH output of the channel 0"]
pub struct OSSUPH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSUPH0_W<'a> {
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
#[doc = "Field `OSSUPH1` writer - Output Selection Set for PWMH output of the channel 1"]
pub struct OSSUPH1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSUPH1_W<'a> {
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
#[doc = "Field `OSSUPH2` writer - Output Selection Set for PWMH output of the channel 2"]
pub struct OSSUPH2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSUPH2_W<'a> {
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
#[doc = "Field `OSSUPH3` writer - Output Selection Set for PWMH output of the channel 3"]
pub struct OSSUPH3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSUPH3_W<'a> {
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
#[doc = "Field `OSSUPL0` writer - Output Selection Set for PWML output of the channel 0"]
pub struct OSSUPL0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSUPL0_W<'a> {
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
#[doc = "Field `OSSUPL1` writer - Output Selection Set for PWML output of the channel 1"]
pub struct OSSUPL1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSUPL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `OSSUPL2` writer - Output Selection Set for PWML output of the channel 2"]
pub struct OSSUPL2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSUPL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `OSSUPL3` writer - Output Selection Set for PWML output of the channel 3"]
pub struct OSSUPL3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSUPL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Output Selection Set for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn ossuph0(&mut self) -> OSSUPH0_W {
        OSSUPH0_W { w: self }
    }
    #[doc = "Bit 1 - Output Selection Set for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn ossuph1(&mut self) -> OSSUPH1_W {
        OSSUPH1_W { w: self }
    }
    #[doc = "Bit 2 - Output Selection Set for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn ossuph2(&mut self) -> OSSUPH2_W {
        OSSUPH2_W { w: self }
    }
    #[doc = "Bit 3 - Output Selection Set for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn ossuph3(&mut self) -> OSSUPH3_W {
        OSSUPH3_W { w: self }
    }
    #[doc = "Bit 16 - Output Selection Set for PWML output of the channel 0"]
    #[inline(always)]
    pub fn ossupl0(&mut self) -> OSSUPL0_W {
        OSSUPL0_W { w: self }
    }
    #[doc = "Bit 17 - Output Selection Set for PWML output of the channel 1"]
    #[inline(always)]
    pub fn ossupl1(&mut self) -> OSSUPL1_W {
        OSSUPL1_W { w: self }
    }
    #[doc = "Bit 18 - Output Selection Set for PWML output of the channel 2"]
    #[inline(always)]
    pub fn ossupl2(&mut self) -> OSSUPL2_W {
        OSSUPL2_W { w: self }
    }
    #[doc = "Bit 19 - Output Selection Set for PWML output of the channel 3"]
    #[inline(always)]
    pub fn ossupl3(&mut self) -> OSSUPL3_W {
        OSSUPL3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Selection Set Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ossupd](index.html) module"]
pub struct PWM_OSSUPD_SPEC;
impl crate::RegisterSpec for PWM_OSSUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwm_ossupd::W](W) writer structure"]
impl crate::Writable for PWM_OSSUPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_OSSUPD to value 0"]
impl crate::Resettable for PWM_OSSUPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
