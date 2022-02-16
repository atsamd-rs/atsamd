#[doc = "Register `PWM_SR` reader"]
pub struct R(crate::R<PWM_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHID0` reader - Channel ID"]
pub struct CHID0_R(crate::FieldReader<bool, bool>);
impl CHID0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHID0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHID0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHID1` reader - Channel ID"]
pub struct CHID1_R(crate::FieldReader<bool, bool>);
impl CHID1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHID1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHID1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHID2` reader - Channel ID"]
pub struct CHID2_R(crate::FieldReader<bool, bool>);
impl CHID2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHID2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHID2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHID3` reader - Channel ID"]
pub struct CHID3_R(crate::FieldReader<bool, bool>);
impl CHID3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHID3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHID3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Channel ID"]
    #[inline(always)]
    pub fn chid0(&self) -> CHID0_R {
        CHID0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel ID"]
    #[inline(always)]
    pub fn chid1(&self) -> CHID1_R {
        CHID1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel ID"]
    #[inline(always)]
    pub fn chid2(&self) -> CHID2_R {
        CHID2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel ID"]
    #[inline(always)]
    pub fn chid3(&self) -> CHID3_R {
        CHID3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "PWM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_sr](index.html) module"]
pub struct PWM_SR_SPEC;
impl crate::RegisterSpec for PWM_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_sr::R](R) reader structure"]
impl crate::Readable for PWM_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWM_SR to value 0"]
impl crate::Resettable for PWM_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
