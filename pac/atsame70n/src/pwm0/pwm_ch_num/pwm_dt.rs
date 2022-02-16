#[doc = "Register `PWM_DT` reader"]
pub struct R(crate::R<PWM_DT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_DT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_DT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_DT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_DT` writer"]
pub struct W(crate::W<PWM_DT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_DT_SPEC>;
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
impl From<crate::W<PWM_DT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_DT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTH` reader - Dead-Time Value for PWMHx Output"]
pub struct DTH_R(crate::FieldReader<u16, u16>);
impl DTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTH` writer - Dead-Time Value for PWMHx Output"]
pub struct DTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `DTL` reader - Dead-Time Value for PWMLx Output"]
pub struct DTL_R(crate::FieldReader<u16, u16>);
impl DTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTL` writer - Dead-Time Value for PWMLx Output"]
pub struct DTL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Dead-Time Value for PWMHx Output"]
    #[inline(always)]
    pub fn dth(&self) -> DTH_R {
        DTH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Dead-Time Value for PWMLx Output"]
    #[inline(always)]
    pub fn dtl(&self) -> DTL_R {
        DTL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Dead-Time Value for PWMHx Output"]
    #[inline(always)]
    pub fn dth(&mut self) -> DTH_W {
        DTH_W { w: self }
    }
    #[doc = "Bits 16:31 - Dead-Time Value for PWMLx Output"]
    #[inline(always)]
    pub fn dtl(&mut self) -> DTL_W {
        DTL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Dead Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_dt](index.html) module"]
pub struct PWM_DT_SPEC;
impl crate::RegisterSpec for PWM_DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_dt::R](R) reader structure"]
impl crate::Readable for PWM_DT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_dt::W](W) writer structure"]
impl crate::Writable for PWM_DT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_DT to value 0"]
impl crate::Resettable for PWM_DT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
