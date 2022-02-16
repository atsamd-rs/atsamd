#[doc = "Register `PWM_FPV2` reader"]
pub struct R(crate::R<PWM_FPV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_FPV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_FPV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_FPV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_FPV2` writer"]
pub struct W(crate::W<PWM_FPV2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_FPV2_SPEC>;
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
impl From<crate::W<PWM_FPV2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_FPV2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPZH0` reader - Fault Protection to Hi-Z for PWMH output on channel 0"]
pub struct FPZH0_R(crate::FieldReader<bool, bool>);
impl FPZH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPZH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPZH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPZH0` writer - Fault Protection to Hi-Z for PWMH output on channel 0"]
pub struct FPZH0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZH0_W<'a> {
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
#[doc = "Field `FPZH1` reader - Fault Protection to Hi-Z for PWMH output on channel 1"]
pub struct FPZH1_R(crate::FieldReader<bool, bool>);
impl FPZH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPZH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPZH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPZH1` writer - Fault Protection to Hi-Z for PWMH output on channel 1"]
pub struct FPZH1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZH1_W<'a> {
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
#[doc = "Field `FPZH2` reader - Fault Protection to Hi-Z for PWMH output on channel 2"]
pub struct FPZH2_R(crate::FieldReader<bool, bool>);
impl FPZH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPZH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPZH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPZH2` writer - Fault Protection to Hi-Z for PWMH output on channel 2"]
pub struct FPZH2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZH2_W<'a> {
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
#[doc = "Field `FPZH3` reader - Fault Protection to Hi-Z for PWMH output on channel 3"]
pub struct FPZH3_R(crate::FieldReader<bool, bool>);
impl FPZH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPZH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPZH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPZH3` writer - Fault Protection to Hi-Z for PWMH output on channel 3"]
pub struct FPZH3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZH3_W<'a> {
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
#[doc = "Field `FPZL0` reader - Fault Protection to Hi-Z for PWML output on channel 0"]
pub struct FPZL0_R(crate::FieldReader<bool, bool>);
impl FPZL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPZL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPZL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPZL0` writer - Fault Protection to Hi-Z for PWML output on channel 0"]
pub struct FPZL0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZL0_W<'a> {
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
#[doc = "Field `FPZL1` reader - Fault Protection to Hi-Z for PWML output on channel 1"]
pub struct FPZL1_R(crate::FieldReader<bool, bool>);
impl FPZL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPZL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPZL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPZL1` writer - Fault Protection to Hi-Z for PWML output on channel 1"]
pub struct FPZL1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZL1_W<'a> {
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
#[doc = "Field `FPZL2` reader - Fault Protection to Hi-Z for PWML output on channel 2"]
pub struct FPZL2_R(crate::FieldReader<bool, bool>);
impl FPZL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPZL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPZL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPZL2` writer - Fault Protection to Hi-Z for PWML output on channel 2"]
pub struct FPZL2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZL2_W<'a> {
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
#[doc = "Field `FPZL3` reader - Fault Protection to Hi-Z for PWML output on channel 3"]
pub struct FPZL3_R(crate::FieldReader<bool, bool>);
impl FPZL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPZL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPZL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPZL3` writer - Fault Protection to Hi-Z for PWML output on channel 3"]
pub struct FPZL3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZL3_W<'a> {
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
    #[doc = "Bit 0 - Fault Protection to Hi-Z for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpzh0(&self) -> FPZH0_R {
        FPZH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault Protection to Hi-Z for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpzh1(&self) -> FPZH1_R {
        FPZH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault Protection to Hi-Z for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpzh2(&self) -> FPZH2_R {
        FPZH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault Protection to Hi-Z for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpzh3(&self) -> FPZH3_R {
        FPZH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fault Protection to Hi-Z for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpzl0(&self) -> FPZL0_R {
        FPZL0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fault Protection to Hi-Z for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpzl1(&self) -> FPZL1_R {
        FPZL1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fault Protection to Hi-Z for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpzl2(&self) -> FPZL2_R {
        FPZL2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fault Protection to Hi-Z for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpzl3(&self) -> FPZL3_R {
        FPZL3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Protection to Hi-Z for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpzh0(&mut self) -> FPZH0_W {
        FPZH0_W { w: self }
    }
    #[doc = "Bit 1 - Fault Protection to Hi-Z for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpzh1(&mut self) -> FPZH1_W {
        FPZH1_W { w: self }
    }
    #[doc = "Bit 2 - Fault Protection to Hi-Z for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpzh2(&mut self) -> FPZH2_W {
        FPZH2_W { w: self }
    }
    #[doc = "Bit 3 - Fault Protection to Hi-Z for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpzh3(&mut self) -> FPZH3_W {
        FPZH3_W { w: self }
    }
    #[doc = "Bit 16 - Fault Protection to Hi-Z for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpzl0(&mut self) -> FPZL0_W {
        FPZL0_W { w: self }
    }
    #[doc = "Bit 17 - Fault Protection to Hi-Z for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpzl1(&mut self) -> FPZL1_W {
        FPZL1_W { w: self }
    }
    #[doc = "Bit 18 - Fault Protection to Hi-Z for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpzl2(&mut self) -> FPZL2_W {
        FPZL2_W { w: self }
    }
    #[doc = "Bit 19 - Fault Protection to Hi-Z for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpzl3(&mut self) -> FPZL3_W {
        FPZL3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Fault Protection Value 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_fpv2](index.html) module"]
pub struct PWM_FPV2_SPEC;
impl crate::RegisterSpec for PWM_FPV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_fpv2::R](R) reader structure"]
impl crate::Readable for PWM_FPV2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_fpv2::W](W) writer structure"]
impl crate::Writable for PWM_FPV2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_FPV2 to value 0"]
impl crate::Resettable for PWM_FPV2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
