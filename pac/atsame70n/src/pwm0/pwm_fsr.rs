#[doc = "Register `PWM_FSR` reader"]
pub struct R(crate::R<PWM_FSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_FSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_FSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_FSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIV` reader - Fault Input Value"]
pub struct FIV_R(crate::FieldReader<u8, u8>);
impl FIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FS` reader - Fault Status"]
pub struct FS_R(crate::FieldReader<u8, u8>);
impl FS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Fault Input Value"]
    #[inline(always)]
    pub fn fiv(&self) -> FIV_R {
        FIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Status"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "PWM Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_fsr](index.html) module"]
pub struct PWM_FSR_SPEC;
impl crate::RegisterSpec for PWM_FSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_fsr::R](R) reader structure"]
impl crate::Readable for PWM_FSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWM_FSR to value 0"]
impl crate::Resettable for PWM_FSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
