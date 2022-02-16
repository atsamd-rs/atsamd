#[doc = "Register `PWM_DTUPD` writer"]
pub struct W(crate::W<PWM_DTUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_DTUPD_SPEC>;
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
impl From<crate::W<PWM_DTUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_DTUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTHUPD` writer - Dead-Time Value Update for PWMHx Output"]
pub struct DTHUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `DTLUPD` writer - Dead-Time Value Update for PWMLx Output"]
pub struct DTLUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DTLUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Dead-Time Value Update for PWMHx Output"]
    #[inline(always)]
    pub fn dthupd(&mut self) -> DTHUPD_W {
        DTHUPD_W { w: self }
    }
    #[doc = "Bits 16:31 - Dead-Time Value Update for PWMLx Output"]
    #[inline(always)]
    pub fn dtlupd(&mut self) -> DTLUPD_W {
        DTLUPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Dead Time Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_dtupd](index.html) module"]
pub struct PWM_DTUPD_SPEC;
impl crate::RegisterSpec for PWM_DTUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwm_dtupd::W](W) writer structure"]
impl crate::Writable for PWM_DTUPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_DTUPD to value 0"]
impl crate::Resettable for PWM_DTUPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
