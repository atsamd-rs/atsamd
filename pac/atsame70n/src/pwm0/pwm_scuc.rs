#[doc = "Register `PWM_SCUC` reader"]
pub struct R(crate::R<PWM_SCUC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_SCUC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_SCUC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_SCUC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_SCUC` writer"]
pub struct W(crate::W<PWM_SCUC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_SCUC_SPEC>;
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
impl From<crate::W<PWM_SCUC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_SCUC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDULOCK` reader - Synchronous Channels Update Unlock"]
pub struct UPDULOCK_R(crate::FieldReader<bool, bool>);
impl UPDULOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPDULOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPDULOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDULOCK` writer - Synchronous Channels Update Unlock"]
pub struct UPDULOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDULOCK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Synchronous Channels Update Unlock"]
    #[inline(always)]
    pub fn updulock(&self) -> UPDULOCK_R {
        UPDULOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous Channels Update Unlock"]
    #[inline(always)]
    pub fn updulock(&mut self) -> UPDULOCK_W {
        UPDULOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Sync Channels Update Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_scuc](index.html) module"]
pub struct PWM_SCUC_SPEC;
impl crate::RegisterSpec for PWM_SCUC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_scuc::R](R) reader structure"]
impl crate::Readable for PWM_SCUC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_scuc::W](W) writer structure"]
impl crate::Writable for PWM_SCUC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_SCUC to value 0"]
impl crate::Resettable for PWM_SCUC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
