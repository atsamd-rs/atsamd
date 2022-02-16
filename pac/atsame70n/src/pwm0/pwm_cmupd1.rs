#[doc = "Register `PWM_CMUPD1` writer"]
pub struct W(crate::W<PWM_CMUPD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CMUPD1_SPEC>;
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
impl From<crate::W<PWM_CMUPD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CMUPD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPOLUP` writer - Channel Polarity Update"]
pub struct CPOLUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOLUP_W<'a> {
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
#[doc = "Field `CPOLINVUP` writer - Channel Polarity Inversion Update"]
pub struct CPOLINVUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOLINVUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl W {
    #[doc = "Bit 9 - Channel Polarity Update"]
    #[inline(always)]
    pub fn cpolup(&mut self) -> CPOLUP_W {
        CPOLUP_W { w: self }
    }
    #[doc = "Bit 13 - Channel Polarity Inversion Update"]
    #[inline(always)]
    pub fn cpolinvup(&mut self) -> CPOLINVUP_W {
        CPOLINVUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Mode Update Register (ch_num = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmupd1](index.html) module"]
pub struct PWM_CMUPD1_SPEC;
impl crate::RegisterSpec for PWM_CMUPD1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwm_cmupd1::W](W) writer structure"]
impl crate::Writable for PWM_CMUPD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CMUPD1 to value 0"]
impl crate::Resettable for PWM_CMUPD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
