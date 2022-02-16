#[doc = "Register `PWM_FPE` reader"]
pub struct R(crate::R<PWM_FPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_FPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_FPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_FPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_FPE` writer"]
pub struct W(crate::W<PWM_FPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_FPE_SPEC>;
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
impl From<crate::W<PWM_FPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_FPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPE0` reader - Fault Protection Enable for channel 0"]
pub struct FPE0_R(crate::FieldReader<u8, u8>);
impl FPE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FPE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPE0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPE0` writer - Fault Protection Enable for channel 0"]
pub struct FPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `FPE1` reader - Fault Protection Enable for channel 1"]
pub struct FPE1_R(crate::FieldReader<u8, u8>);
impl FPE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FPE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPE1` writer - Fault Protection Enable for channel 1"]
pub struct FPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `FPE2` reader - Fault Protection Enable for channel 2"]
pub struct FPE2_R(crate::FieldReader<u8, u8>);
impl FPE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FPE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPE2` writer - Fault Protection Enable for channel 2"]
pub struct FPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `FPE3` reader - Fault Protection Enable for channel 3"]
pub struct FPE3_R(crate::FieldReader<u8, u8>);
impl FPE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FPE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPE3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPE3` writer - Fault Protection Enable for channel 3"]
pub struct FPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Fault Protection Enable for channel 0"]
    #[inline(always)]
    pub fn fpe0(&self) -> FPE0_R {
        FPE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Protection Enable for channel 1"]
    #[inline(always)]
    pub fn fpe1(&self) -> FPE1_R {
        FPE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Fault Protection Enable for channel 2"]
    #[inline(always)]
    pub fn fpe2(&self) -> FPE2_R {
        FPE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Fault Protection Enable for channel 3"]
    #[inline(always)]
    pub fn fpe3(&self) -> FPE3_R {
        FPE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Protection Enable for channel 0"]
    #[inline(always)]
    pub fn fpe0(&mut self) -> FPE0_W {
        FPE0_W { w: self }
    }
    #[doc = "Bits 8:15 - Fault Protection Enable for channel 1"]
    #[inline(always)]
    pub fn fpe1(&mut self) -> FPE1_W {
        FPE1_W { w: self }
    }
    #[doc = "Bits 16:23 - Fault Protection Enable for channel 2"]
    #[inline(always)]
    pub fn fpe2(&mut self) -> FPE2_W {
        FPE2_W { w: self }
    }
    #[doc = "Bits 24:31 - Fault Protection Enable for channel 3"]
    #[inline(always)]
    pub fn fpe3(&mut self) -> FPE3_W {
        FPE3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Fault Protection Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_fpe](index.html) module"]
pub struct PWM_FPE_SPEC;
impl crate::RegisterSpec for PWM_FPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_fpe::R](R) reader structure"]
impl crate::Readable for PWM_FPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_fpe::W](W) writer structure"]
impl crate::Writable for PWM_FPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_FPE to value 0"]
impl crate::Resettable for PWM_FPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
