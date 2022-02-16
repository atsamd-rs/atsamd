#[doc = "Register `PWM_FPV1` reader"]
pub struct R(crate::R<PWM_FPV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_FPV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_FPV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_FPV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_FPV1` writer"]
pub struct W(crate::W<PWM_FPV1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_FPV1_SPEC>;
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
impl From<crate::W<PWM_FPV1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_FPV1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPVH0` reader - Fault Protection Value for PWMH output on channel 0"]
pub struct FPVH0_R(crate::FieldReader<bool, bool>);
impl FPVH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVH0` writer - Fault Protection Value for PWMH output on channel 0"]
pub struct FPVH0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH0_W<'a> {
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
#[doc = "Field `FPVH1` reader - Fault Protection Value for PWMH output on channel 1"]
pub struct FPVH1_R(crate::FieldReader<bool, bool>);
impl FPVH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVH1` writer - Fault Protection Value for PWMH output on channel 1"]
pub struct FPVH1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH1_W<'a> {
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
#[doc = "Field `FPVH2` reader - Fault Protection Value for PWMH output on channel 2"]
pub struct FPVH2_R(crate::FieldReader<bool, bool>);
impl FPVH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVH2` writer - Fault Protection Value for PWMH output on channel 2"]
pub struct FPVH2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH2_W<'a> {
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
#[doc = "Field `FPVH3` reader - Fault Protection Value for PWMH output on channel 3"]
pub struct FPVH3_R(crate::FieldReader<bool, bool>);
impl FPVH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVH3` writer - Fault Protection Value for PWMH output on channel 3"]
pub struct FPVH3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH3_W<'a> {
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
#[doc = "Field `FPVL0` reader - Fault Protection Value for PWML output on channel 0"]
pub struct FPVL0_R(crate::FieldReader<bool, bool>);
impl FPVL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVL0` writer - Fault Protection Value for PWML output on channel 0"]
pub struct FPVL0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL0_W<'a> {
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
#[doc = "Field `FPVL1` reader - Fault Protection Value for PWML output on channel 1"]
pub struct FPVL1_R(crate::FieldReader<bool, bool>);
impl FPVL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVL1` writer - Fault Protection Value for PWML output on channel 1"]
pub struct FPVL1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL1_W<'a> {
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
#[doc = "Field `FPVL2` reader - Fault Protection Value for PWML output on channel 2"]
pub struct FPVL2_R(crate::FieldReader<bool, bool>);
impl FPVL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVL2` writer - Fault Protection Value for PWML output on channel 2"]
pub struct FPVL2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL2_W<'a> {
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
#[doc = "Field `FPVL3` reader - Fault Protection Value for PWML output on channel 3"]
pub struct FPVL3_R(crate::FieldReader<bool, bool>);
impl FPVL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPVL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPVL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPVL3` writer - Fault Protection Value for PWML output on channel 3"]
pub struct FPVL3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL3_W<'a> {
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
    #[doc = "Bit 0 - Fault Protection Value for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpvh0(&self) -> FPVH0_R {
        FPVH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpvh1(&self) -> FPVH1_R {
        FPVH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpvh2(&self) -> FPVH2_R {
        FPVH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpvh3(&self) -> FPVH3_R {
        FPVH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpvl0(&self) -> FPVL0_R {
        FPVL0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpvl1(&self) -> FPVL1_R {
        FPVL1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpvl2(&self) -> FPVL2_R {
        FPVL2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpvl3(&self) -> FPVL3_R {
        FPVL3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Protection Value for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpvh0(&mut self) -> FPVH0_W {
        FPVH0_W { w: self }
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpvh1(&mut self) -> FPVH1_W {
        FPVH1_W { w: self }
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpvh2(&mut self) -> FPVH2_W {
        FPVH2_W { w: self }
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpvh3(&mut self) -> FPVH3_W {
        FPVH3_W { w: self }
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpvl0(&mut self) -> FPVL0_W {
        FPVL0_W { w: self }
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpvl1(&mut self) -> FPVL1_W {
        FPVL1_W { w: self }
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpvl2(&mut self) -> FPVL2_W {
        FPVL2_W { w: self }
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpvl3(&mut self) -> FPVL3_W {
        FPVL3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Fault Protection Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_fpv1](index.html) module"]
pub struct PWM_FPV1_SPEC;
impl crate::RegisterSpec for PWM_FPV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_fpv1::R](R) reader structure"]
impl crate::Readable for PWM_FPV1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_fpv1::W](W) writer structure"]
impl crate::Writable for PWM_FPV1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_FPV1 to value 0"]
impl crate::Resettable for PWM_FPV1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
