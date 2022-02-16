#[doc = "Register `PWM_ENA` writer"]
pub struct W(crate::W<PWM_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_ENA_SPEC>;
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
impl From<crate::W<PWM_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHID0` writer - Channel ID"]
pub struct CHID0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHID0_W<'a> {
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
#[doc = "Field `CHID1` writer - Channel ID"]
pub struct CHID1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHID1_W<'a> {
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
#[doc = "Field `CHID2` writer - Channel ID"]
pub struct CHID2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHID2_W<'a> {
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
#[doc = "Field `CHID3` writer - Channel ID"]
pub struct CHID3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHID3_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Channel ID"]
    #[inline(always)]
    pub fn chid0(&mut self) -> CHID0_W {
        CHID0_W { w: self }
    }
    #[doc = "Bit 1 - Channel ID"]
    #[inline(always)]
    pub fn chid1(&mut self) -> CHID1_W {
        CHID1_W { w: self }
    }
    #[doc = "Bit 2 - Channel ID"]
    #[inline(always)]
    pub fn chid2(&mut self) -> CHID2_W {
        CHID2_W { w: self }
    }
    #[doc = "Bit 3 - Channel ID"]
    #[inline(always)]
    pub fn chid3(&mut self) -> CHID3_W {
        CHID3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ena](index.html) module"]
pub struct PWM_ENA_SPEC;
impl crate::RegisterSpec for PWM_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwm_ena::W](W) writer structure"]
impl crate::Writable for PWM_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_ENA to value 0"]
impl crate::Resettable for PWM_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
