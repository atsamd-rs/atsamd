#[doc = "Register `PWM_DMAR` writer"]
pub struct W(crate::W<PWM_DMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_DMAR_SPEC>;
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
impl From<crate::W<PWM_DMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_DMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMADUTY` writer - Duty-Cycle Holding Register for DMA Access"]
pub struct DMADUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADUTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:23 - Duty-Cycle Holding Register for DMA Access"]
    #[inline(always)]
    pub fn dmaduty(&mut self) -> DMADUTY_W {
        DMADUTY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM DMA Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_dmar](index.html) module"]
pub struct PWM_DMAR_SPEC;
impl crate::RegisterSpec for PWM_DMAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwm_dmar::W](W) writer structure"]
impl crate::Writable for PWM_DMAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_DMAR to value 0"]
impl crate::Resettable for PWM_DMAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
