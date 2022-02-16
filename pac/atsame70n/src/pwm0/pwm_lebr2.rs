#[doc = "Register `PWM_LEBR2` reader"]
pub struct R(crate::R<PWM_LEBR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_LEBR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_LEBR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_LEBR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_LEBR2` writer"]
pub struct W(crate::W<PWM_LEBR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_LEBR2_SPEC>;
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
impl From<crate::W<PWM_LEBR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_LEBR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEBDELAY` reader - Leading-Edge Blanking Delay for TRGINx"]
pub struct LEBDELAY_R(crate::FieldReader<u8, u8>);
impl LEBDELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LEBDELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEBDELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEBDELAY` writer - Leading-Edge Blanking Delay for TRGINx"]
pub struct LEBDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> LEBDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `PWMLFEN` reader - PWML Falling Edge Enable"]
pub struct PWMLFEN_R(crate::FieldReader<bool, bool>);
impl PWMLFEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMLFEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMLFEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMLFEN` writer - PWML Falling Edge Enable"]
pub struct PWMLFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMLFEN_W<'a> {
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
#[doc = "Field `PWMLREN` reader - PWML Rising Edge Enable"]
pub struct PWMLREN_R(crate::FieldReader<bool, bool>);
impl PWMLREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMLREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMLREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMLREN` writer - PWML Rising Edge Enable"]
pub struct PWMLREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMLREN_W<'a> {
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
#[doc = "Field `PWMHFEN` reader - PWMH Falling Edge Enable"]
pub struct PWMHFEN_R(crate::FieldReader<bool, bool>);
impl PWMHFEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMHFEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMHFEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMHFEN` writer - PWMH Falling Edge Enable"]
pub struct PWMHFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMHFEN_W<'a> {
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
#[doc = "Field `PWMHREN` reader - PWMH Rising Edge Enable"]
pub struct PWMHREN_R(crate::FieldReader<bool, bool>);
impl PWMHREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMHREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMHREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMHREN` writer - PWMH Rising Edge Enable"]
pub struct PWMHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMHREN_W<'a> {
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
impl R {
    #[doc = "Bits 0:6 - Leading-Edge Blanking Delay for TRGINx"]
    #[inline(always)]
    pub fn lebdelay(&self) -> LEBDELAY_R {
        LEBDELAY_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - PWML Falling Edge Enable"]
    #[inline(always)]
    pub fn pwmlfen(&self) -> PWMLFEN_R {
        PWMLFEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PWML Rising Edge Enable"]
    #[inline(always)]
    pub fn pwmlren(&self) -> PWMLREN_R {
        PWMLREN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PWMH Falling Edge Enable"]
    #[inline(always)]
    pub fn pwmhfen(&self) -> PWMHFEN_R {
        PWMHFEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PWMH Rising Edge Enable"]
    #[inline(always)]
    pub fn pwmhren(&self) -> PWMHREN_R {
        PWMHREN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Leading-Edge Blanking Delay for TRGINx"]
    #[inline(always)]
    pub fn lebdelay(&mut self) -> LEBDELAY_W {
        LEBDELAY_W { w: self }
    }
    #[doc = "Bit 16 - PWML Falling Edge Enable"]
    #[inline(always)]
    pub fn pwmlfen(&mut self) -> PWMLFEN_W {
        PWMLFEN_W { w: self }
    }
    #[doc = "Bit 17 - PWML Rising Edge Enable"]
    #[inline(always)]
    pub fn pwmlren(&mut self) -> PWMLREN_W {
        PWMLREN_W { w: self }
    }
    #[doc = "Bit 18 - PWMH Falling Edge Enable"]
    #[inline(always)]
    pub fn pwmhfen(&mut self) -> PWMHFEN_W {
        PWMHFEN_W { w: self }
    }
    #[doc = "Bit 19 - PWMH Rising Edge Enable"]
    #[inline(always)]
    pub fn pwmhren(&mut self) -> PWMHREN_W {
        PWMHREN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_lebr2](index.html) module"]
pub struct PWM_LEBR2_SPEC;
impl crate::RegisterSpec for PWM_LEBR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_lebr2::R](R) reader structure"]
impl crate::Readable for PWM_LEBR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_lebr2::W](W) writer structure"]
impl crate::Writable for PWM_LEBR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_LEBR2 to value 0"]
impl crate::Resettable for PWM_LEBR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
