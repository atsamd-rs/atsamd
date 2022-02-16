#[doc = "Register `PWM_SSPR` reader"]
pub struct R(crate::R<PWM_SSPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_SSPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_SSPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_SSPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_SSPR` writer"]
pub struct W(crate::W<PWM_SSPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_SSPR_SPEC>;
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
impl From<crate::W<PWM_SSPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_SSPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPRD` reader - Spread Spectrum Limit Value"]
pub struct SPRD_R(crate::FieldReader<u32, u32>);
impl SPRD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SPRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPRD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPRD` writer - Spread Spectrum Limit Value"]
pub struct SPRD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Field `SPRDM` reader - Spread Spectrum Counter Mode"]
pub struct SPRDM_R(crate::FieldReader<bool, bool>);
impl SPRDM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPRDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPRDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPRDM` writer - Spread Spectrum Counter Mode"]
pub struct SPRDM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRDM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Spread Spectrum Limit Value"]
    #[inline(always)]
    pub fn sprd(&self) -> SPRD_R {
        SPRD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Spread Spectrum Counter Mode"]
    #[inline(always)]
    pub fn sprdm(&self) -> SPRDM_R {
        SPRDM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Spread Spectrum Limit Value"]
    #[inline(always)]
    pub fn sprd(&mut self) -> SPRD_W {
        SPRD_W { w: self }
    }
    #[doc = "Bit 24 - Spread Spectrum Counter Mode"]
    #[inline(always)]
    pub fn sprdm(&mut self) -> SPRDM_W {
        SPRDM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Spread Spectrum Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_sspr](index.html) module"]
pub struct PWM_SSPR_SPEC;
impl crate::RegisterSpec for PWM_SSPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_sspr::R](R) reader structure"]
impl crate::Readable for PWM_SSPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_sspr::W](W) writer structure"]
impl crate::Writable for PWM_SSPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_SSPR to value 0"]
impl crate::Resettable for PWM_SSPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
