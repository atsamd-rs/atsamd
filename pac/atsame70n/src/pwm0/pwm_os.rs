#[doc = "Register `PWM_OS` reader"]
pub struct R(crate::R<PWM_OS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_OS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_OS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_OS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_OS` writer"]
pub struct W(crate::W<PWM_OS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_OS_SPEC>;
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
impl From<crate::W<PWM_OS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_OS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSH0` reader - Output Selection for PWMH output of the channel 0"]
pub struct OSH0_R(crate::FieldReader<bool, bool>);
impl OSH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSH0` writer - Output Selection for PWMH output of the channel 0"]
pub struct OSH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSH0_W<'a> {
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
#[doc = "Field `OSH1` reader - Output Selection for PWMH output of the channel 1"]
pub struct OSH1_R(crate::FieldReader<bool, bool>);
impl OSH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSH1` writer - Output Selection for PWMH output of the channel 1"]
pub struct OSH1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSH1_W<'a> {
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
#[doc = "Field `OSH2` reader - Output Selection for PWMH output of the channel 2"]
pub struct OSH2_R(crate::FieldReader<bool, bool>);
impl OSH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSH2` writer - Output Selection for PWMH output of the channel 2"]
pub struct OSH2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSH2_W<'a> {
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
#[doc = "Field `OSH3` reader - Output Selection for PWMH output of the channel 3"]
pub struct OSH3_R(crate::FieldReader<bool, bool>);
impl OSH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSH3` writer - Output Selection for PWMH output of the channel 3"]
pub struct OSH3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSH3_W<'a> {
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
#[doc = "Field `OSL0` reader - Output Selection for PWML output of the channel 0"]
pub struct OSL0_R(crate::FieldReader<bool, bool>);
impl OSL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSL0` writer - Output Selection for PWML output of the channel 0"]
pub struct OSL0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSL0_W<'a> {
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
#[doc = "Field `OSL1` reader - Output Selection for PWML output of the channel 1"]
pub struct OSL1_R(crate::FieldReader<bool, bool>);
impl OSL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSL1` writer - Output Selection for PWML output of the channel 1"]
pub struct OSL1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSL1_W<'a> {
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
#[doc = "Field `OSL2` reader - Output Selection for PWML output of the channel 2"]
pub struct OSL2_R(crate::FieldReader<bool, bool>);
impl OSL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSL2` writer - Output Selection for PWML output of the channel 2"]
pub struct OSL2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSL2_W<'a> {
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
#[doc = "Field `OSL3` reader - Output Selection for PWML output of the channel 3"]
pub struct OSL3_R(crate::FieldReader<bool, bool>);
impl OSL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSL3` writer - Output Selection for PWML output of the channel 3"]
pub struct OSL3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSL3_W<'a> {
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
    #[doc = "Bit 0 - Output Selection for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn osh0(&self) -> OSH0_R {
        OSH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output Selection for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn osh1(&self) -> OSH1_R {
        OSH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output Selection for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn osh2(&self) -> OSH2_R {
        OSH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output Selection for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn osh3(&self) -> OSH3_R {
        OSH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Output Selection for PWML output of the channel 0"]
    #[inline(always)]
    pub fn osl0(&self) -> OSL0_R {
        OSL0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Output Selection for PWML output of the channel 1"]
    #[inline(always)]
    pub fn osl1(&self) -> OSL1_R {
        OSL1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output Selection for PWML output of the channel 2"]
    #[inline(always)]
    pub fn osl2(&self) -> OSL2_R {
        OSL2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Output Selection for PWML output of the channel 3"]
    #[inline(always)]
    pub fn osl3(&self) -> OSL3_R {
        OSL3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Selection for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn osh0(&mut self) -> OSH0_W {
        OSH0_W { w: self }
    }
    #[doc = "Bit 1 - Output Selection for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn osh1(&mut self) -> OSH1_W {
        OSH1_W { w: self }
    }
    #[doc = "Bit 2 - Output Selection for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn osh2(&mut self) -> OSH2_W {
        OSH2_W { w: self }
    }
    #[doc = "Bit 3 - Output Selection for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn osh3(&mut self) -> OSH3_W {
        OSH3_W { w: self }
    }
    #[doc = "Bit 16 - Output Selection for PWML output of the channel 0"]
    #[inline(always)]
    pub fn osl0(&mut self) -> OSL0_W {
        OSL0_W { w: self }
    }
    #[doc = "Bit 17 - Output Selection for PWML output of the channel 1"]
    #[inline(always)]
    pub fn osl1(&mut self) -> OSL1_W {
        OSL1_W { w: self }
    }
    #[doc = "Bit 18 - Output Selection for PWML output of the channel 2"]
    #[inline(always)]
    pub fn osl2(&mut self) -> OSL2_W {
        OSL2_W { w: self }
    }
    #[doc = "Bit 19 - Output Selection for PWML output of the channel 3"]
    #[inline(always)]
    pub fn osl3(&mut self) -> OSL3_W {
        OSL3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_os](index.html) module"]
pub struct PWM_OS_SPEC;
impl crate::RegisterSpec for PWM_OS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_os::R](R) reader structure"]
impl crate::Readable for PWM_OS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_os::W](W) writer structure"]
impl crate::Writable for PWM_OS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_OS to value 0"]
impl crate::Resettable for PWM_OS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
