#[doc = "Register `PWM_SCUP` reader"]
pub struct R(crate::R<PWM_SCUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_SCUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_SCUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_SCUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_SCUP` writer"]
pub struct W(crate::W<PWM_SCUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_SCUP_SPEC>;
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
impl From<crate::W<PWM_SCUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_SCUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPR` reader - Update Period"]
pub struct UPR_R(crate::FieldReader<u8, u8>);
impl UPR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPR` writer - Update Period"]
pub struct UPR_W<'a> {
    w: &'a mut W,
}
impl<'a> UPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `UPRCNT` reader - Update Period Counter"]
pub struct UPRCNT_R(crate::FieldReader<u8, u8>);
impl UPRCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UPRCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPRCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPRCNT` writer - Update Period Counter"]
pub struct UPRCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Update Period"]
    #[inline(always)]
    pub fn upr(&self) -> UPR_R {
        UPR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Update Period Counter"]
    #[inline(always)]
    pub fn uprcnt(&self) -> UPRCNT_R {
        UPRCNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Update Period"]
    #[inline(always)]
    pub fn upr(&mut self) -> UPR_W {
        UPR_W { w: self }
    }
    #[doc = "Bits 4:7 - Update Period Counter"]
    #[inline(always)]
    pub fn uprcnt(&mut self) -> UPRCNT_W {
        UPRCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Sync Channels Update Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_scup](index.html) module"]
pub struct PWM_SCUP_SPEC;
impl crate::RegisterSpec for PWM_SCUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_scup::R](R) reader structure"]
impl crate::Readable for PWM_SCUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_scup::W](W) writer structure"]
impl crate::Writable for PWM_SCUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_SCUP to value 0"]
impl crate::Resettable for PWM_SCUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
