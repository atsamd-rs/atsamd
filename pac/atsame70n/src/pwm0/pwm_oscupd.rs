#[doc = "Register `PWM_OSCUPD` writer"]
pub struct W(crate::W<PWM_OSCUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_OSCUPD_SPEC>;
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
impl From<crate::W<PWM_OSCUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_OSCUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSCUPH0` writer - Output Selection Clear for PWMH output of the channel 0"]
pub struct OSCUPH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCUPH0_W<'a> {
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
#[doc = "Field `OSCUPH1` writer - Output Selection Clear for PWMH output of the channel 1"]
pub struct OSCUPH1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCUPH1_W<'a> {
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
#[doc = "Field `OSCUPH2` writer - Output Selection Clear for PWMH output of the channel 2"]
pub struct OSCUPH2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCUPH2_W<'a> {
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
#[doc = "Field `OSCUPH3` writer - Output Selection Clear for PWMH output of the channel 3"]
pub struct OSCUPH3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCUPH3_W<'a> {
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
#[doc = "Field `OSCUPL0` writer - Output Selection Clear for PWML output of the channel 0"]
pub struct OSCUPL0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCUPL0_W<'a> {
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
#[doc = "Field `OSCUPL1` writer - Output Selection Clear for PWML output of the channel 1"]
pub struct OSCUPL1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCUPL1_W<'a> {
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
#[doc = "Field `OSCUPL2` writer - Output Selection Clear for PWML output of the channel 2"]
pub struct OSCUPL2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCUPL2_W<'a> {
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
#[doc = "Field `OSCUPL3` writer - Output Selection Clear for PWML output of the channel 3"]
pub struct OSCUPL3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCUPL3_W<'a> {
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
    #[doc = "Bit 0 - Output Selection Clear for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn oscuph0(&mut self) -> OSCUPH0_W {
        OSCUPH0_W { w: self }
    }
    #[doc = "Bit 1 - Output Selection Clear for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn oscuph1(&mut self) -> OSCUPH1_W {
        OSCUPH1_W { w: self }
    }
    #[doc = "Bit 2 - Output Selection Clear for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn oscuph2(&mut self) -> OSCUPH2_W {
        OSCUPH2_W { w: self }
    }
    #[doc = "Bit 3 - Output Selection Clear for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn oscuph3(&mut self) -> OSCUPH3_W {
        OSCUPH3_W { w: self }
    }
    #[doc = "Bit 16 - Output Selection Clear for PWML output of the channel 0"]
    #[inline(always)]
    pub fn oscupl0(&mut self) -> OSCUPL0_W {
        OSCUPL0_W { w: self }
    }
    #[doc = "Bit 17 - Output Selection Clear for PWML output of the channel 1"]
    #[inline(always)]
    pub fn oscupl1(&mut self) -> OSCUPL1_W {
        OSCUPL1_W { w: self }
    }
    #[doc = "Bit 18 - Output Selection Clear for PWML output of the channel 2"]
    #[inline(always)]
    pub fn oscupl2(&mut self) -> OSCUPL2_W {
        OSCUPL2_W { w: self }
    }
    #[doc = "Bit 19 - Output Selection Clear for PWML output of the channel 3"]
    #[inline(always)]
    pub fn oscupl3(&mut self) -> OSCUPL3_W {
        OSCUPL3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Selection Clear Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_oscupd](index.html) module"]
pub struct PWM_OSCUPD_SPEC;
impl crate::RegisterSpec for PWM_OSCUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwm_oscupd::W](W) writer structure"]
impl crate::Writable for PWM_OSCUPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_OSCUPD to value 0"]
impl crate::Resettable for PWM_OSCUPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
