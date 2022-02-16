#[doc = "Register `PWM_FMR` reader"]
pub struct R(crate::R<PWM_FMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_FMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_FMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_FMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_FMR` writer"]
pub struct W(crate::W<PWM_FMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_FMR_SPEC>;
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
impl From<crate::W<PWM_FMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_FMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPOL` reader - Fault Polarity"]
pub struct FPOL_R(crate::FieldReader<u8, u8>);
impl FPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPOL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPOL` writer - Fault Polarity"]
pub struct FPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> FPOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `FMOD` reader - Fault Activation Mode"]
pub struct FMOD_R(crate::FieldReader<u8, u8>);
impl FMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMOD` writer - Fault Activation Mode"]
pub struct FMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `FFIL` reader - Fault Filtering"]
pub struct FFIL_R(crate::FieldReader<u8, u8>);
impl FFIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FFIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFIL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFIL` writer - Fault Filtering"]
pub struct FFIL_W<'a> {
    w: &'a mut W,
}
impl<'a> FFIL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Fault Polarity"]
    #[inline(always)]
    pub fn fpol(&self) -> FPOL_R {
        FPOL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Activation Mode"]
    #[inline(always)]
    pub fn fmod(&self) -> FMOD_R {
        FMOD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Fault Filtering"]
    #[inline(always)]
    pub fn ffil(&self) -> FFIL_R {
        FFIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Polarity"]
    #[inline(always)]
    pub fn fpol(&mut self) -> FPOL_W {
        FPOL_W { w: self }
    }
    #[doc = "Bits 8:15 - Fault Activation Mode"]
    #[inline(always)]
    pub fn fmod(&mut self) -> FMOD_W {
        FMOD_W { w: self }
    }
    #[doc = "Bits 16:23 - Fault Filtering"]
    #[inline(always)]
    pub fn ffil(&mut self) -> FFIL_W {
        FFIL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Fault Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_fmr](index.html) module"]
pub struct PWM_FMR_SPEC;
impl crate::RegisterSpec for PWM_FMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_fmr::R](R) reader structure"]
impl crate::Readable for PWM_FMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_fmr::W](W) writer structure"]
impl crate::Writable for PWM_FMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_FMR to value 0"]
impl crate::Resettable for PWM_FMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
