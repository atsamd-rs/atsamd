#[doc = "Register `PWM_SMMR` reader"]
pub struct R(crate::R<PWM_SMMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_SMMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_SMMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_SMMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_SMMR` writer"]
pub struct W(crate::W<PWM_SMMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_SMMR_SPEC>;
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
impl From<crate::W<PWM_SMMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_SMMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GCEN0` reader - Gray Count ENable"]
pub struct GCEN0_R(crate::FieldReader<bool, bool>);
impl GCEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GCEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCEN0` writer - Gray Count ENable"]
pub struct GCEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> GCEN0_W<'a> {
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
#[doc = "Field `GCEN1` reader - Gray Count ENable"]
pub struct GCEN1_R(crate::FieldReader<bool, bool>);
impl GCEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GCEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCEN1` writer - Gray Count ENable"]
pub struct GCEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> GCEN1_W<'a> {
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
#[doc = "Field `DOWN0` reader - DOWN Count"]
pub struct DOWN0_R(crate::FieldReader<bool, bool>);
impl DOWN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOWN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOWN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOWN0` writer - DOWN Count"]
pub struct DOWN0_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWN0_W<'a> {
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
#[doc = "Field `DOWN1` reader - DOWN Count"]
pub struct DOWN1_R(crate::FieldReader<bool, bool>);
impl DOWN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOWN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOWN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOWN1` writer - DOWN Count"]
pub struct DOWN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWN1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen0(&self) -> GCEN0_R {
        GCEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen1(&self) -> GCEN1_R {
        GCEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DOWN Count"]
    #[inline(always)]
    pub fn down0(&self) -> DOWN0_R {
        DOWN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DOWN Count"]
    #[inline(always)]
    pub fn down1(&self) -> DOWN1_R {
        DOWN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen0(&mut self) -> GCEN0_W {
        GCEN0_W { w: self }
    }
    #[doc = "Bit 1 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen1(&mut self) -> GCEN1_W {
        GCEN1_W { w: self }
    }
    #[doc = "Bit 16 - DOWN Count"]
    #[inline(always)]
    pub fn down0(&mut self) -> DOWN0_W {
        DOWN0_W { w: self }
    }
    #[doc = "Bit 17 - DOWN Count"]
    #[inline(always)]
    pub fn down1(&mut self) -> DOWN1_W {
        DOWN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Stepper Motor Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_smmr](index.html) module"]
pub struct PWM_SMMR_SPEC;
impl crate::RegisterSpec for PWM_SMMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_smmr::R](R) reader structure"]
impl crate::Readable for PWM_SMMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_smmr::W](W) writer structure"]
impl crate::Writable for PWM_SMMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_SMMR to value 0"]
impl crate::Resettable for PWM_SMMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
