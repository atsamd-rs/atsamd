#[doc = "Register `PWM_IER1` writer"]
pub struct W(crate::W<PWM_IER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_IER1_SPEC>;
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
impl From<crate::W<PWM_IER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_IER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHID0` writer - Counter Event on Channel 0 Interrupt Enable"]
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
#[doc = "Field `CHID1` writer - Counter Event on Channel 1 Interrupt Enable"]
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
#[doc = "Field `CHID2` writer - Counter Event on Channel 2 Interrupt Enable"]
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
#[doc = "Field `CHID3` writer - Counter Event on Channel 3 Interrupt Enable"]
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
#[doc = "Field `FCHID0` writer - Fault Protection Trigger on Channel 0 Interrupt Enable"]
pub struct FCHID0_W<'a> {
    w: &'a mut W,
}
impl<'a> FCHID0_W<'a> {
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
#[doc = "Field `FCHID1` writer - Fault Protection Trigger on Channel 1 Interrupt Enable"]
pub struct FCHID1_W<'a> {
    w: &'a mut W,
}
impl<'a> FCHID1_W<'a> {
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
#[doc = "Field `FCHID2` writer - Fault Protection Trigger on Channel 2 Interrupt Enable"]
pub struct FCHID2_W<'a> {
    w: &'a mut W,
}
impl<'a> FCHID2_W<'a> {
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
#[doc = "Field `FCHID3` writer - Fault Protection Trigger on Channel 3 Interrupt Enable"]
pub struct FCHID3_W<'a> {
    w: &'a mut W,
}
impl<'a> FCHID3_W<'a> {
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
    #[doc = "Bit 0 - Counter Event on Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn chid0(&mut self) -> CHID0_W {
        CHID0_W { w: self }
    }
    #[doc = "Bit 1 - Counter Event on Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn chid1(&mut self) -> CHID1_W {
        CHID1_W { w: self }
    }
    #[doc = "Bit 2 - Counter Event on Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn chid2(&mut self) -> CHID2_W {
        CHID2_W { w: self }
    }
    #[doc = "Bit 3 - Counter Event on Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn chid3(&mut self) -> CHID3_W {
        CHID3_W { w: self }
    }
    #[doc = "Bit 16 - Fault Protection Trigger on Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn fchid0(&mut self) -> FCHID0_W {
        FCHID0_W { w: self }
    }
    #[doc = "Bit 17 - Fault Protection Trigger on Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn fchid1(&mut self) -> FCHID1_W {
        FCHID1_W { w: self }
    }
    #[doc = "Bit 18 - Fault Protection Trigger on Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn fchid2(&mut self) -> FCHID2_W {
        FCHID2_W { w: self }
    }
    #[doc = "Bit 19 - Fault Protection Trigger on Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn fchid3(&mut self) -> FCHID3_W {
        FCHID3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Interrupt Enable Register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ier1](index.html) module"]
pub struct PWM_IER1_SPEC;
impl crate::RegisterSpec for PWM_IER1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwm_ier1::W](W) writer structure"]
impl crate::Writable for PWM_IER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_IER1 to value 0"]
impl crate::Resettable for PWM_IER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
