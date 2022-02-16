#[doc = "Register `PWM_CDTY` reader"]
pub struct R(crate::R<PWM_CDTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CDTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CDTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CDTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CDTY` writer"]
pub struct W(crate::W<PWM_CDTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CDTY_SPEC>;
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
impl From<crate::W<PWM_CDTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CDTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDTY` reader - Channel Duty-Cycle"]
pub struct CDTY_R(crate::FieldReader<u32, u32>);
impl CDTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CDTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDTY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDTY` writer - Channel Duty-Cycle"]
pub struct CDTY_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    pub fn cdty(&self) -> CDTY_R {
        CDTY_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    pub fn cdty(&mut self) -> CDTY_W {
        CDTY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Duty Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cdty](index.html) module"]
pub struct PWM_CDTY_SPEC;
impl crate::RegisterSpec for PWM_CDTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_cdty::R](R) reader structure"]
impl crate::Readable for PWM_CDTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_cdty::W](W) writer structure"]
impl crate::Writable for PWM_CDTY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CDTY to value 0"]
impl crate::Resettable for PWM_CDTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
