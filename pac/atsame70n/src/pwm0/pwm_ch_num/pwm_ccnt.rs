#[doc = "Register `PWM_CCNT` reader"]
pub struct R(crate::R<PWM_CCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - Channel Counter Register"]
pub struct CNT_R(crate::FieldReader<u32, u32>);
impl CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - Channel Counter Register"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "PWM Channel Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ccnt](index.html) module"]
pub struct PWM_CCNT_SPEC;
impl crate::RegisterSpec for PWM_CCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_ccnt::R](R) reader structure"]
impl crate::Readable for PWM_CCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWM_CCNT to value 0"]
impl crate::Resettable for PWM_CCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
