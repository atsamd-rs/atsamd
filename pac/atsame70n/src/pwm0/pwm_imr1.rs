#[doc = "Register `PWM_IMR1` reader"]
pub struct R(crate::R<PWM_IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_IMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHID0` reader - Counter Event on Channel 0 Interrupt Mask"]
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
#[doc = "Field `CHID1` reader - Counter Event on Channel 1 Interrupt Mask"]
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
#[doc = "Field `CHID2` reader - Counter Event on Channel 2 Interrupt Mask"]
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
#[doc = "Field `CHID3` reader - Counter Event on Channel 3 Interrupt Mask"]
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
#[doc = "Field `FCHID0` reader - Fault Protection Trigger on Channel 0 Interrupt Mask"]
pub struct FCHID0_R(crate::FieldReader<bool, bool>);
impl FCHID0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCHID0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCHID0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCHID1` reader - Fault Protection Trigger on Channel 1 Interrupt Mask"]
pub struct FCHID1_R(crate::FieldReader<bool, bool>);
impl FCHID1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCHID1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCHID1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCHID2` reader - Fault Protection Trigger on Channel 2 Interrupt Mask"]
pub struct FCHID2_R(crate::FieldReader<bool, bool>);
impl FCHID2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCHID2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCHID2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCHID3` reader - Fault Protection Trigger on Channel 3 Interrupt Mask"]
pub struct FCHID3_R(crate::FieldReader<bool, bool>);
impl FCHID3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCHID3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCHID3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Counter Event on Channel 0 Interrupt Mask"]
    #[inline(always)]
    pub fn chid0(&self) -> CHID0_R {
        CHID0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter Event on Channel 1 Interrupt Mask"]
    #[inline(always)]
    pub fn chid1(&self) -> CHID1_R {
        CHID1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Counter Event on Channel 2 Interrupt Mask"]
    #[inline(always)]
    pub fn chid2(&self) -> CHID2_R {
        CHID2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Counter Event on Channel 3 Interrupt Mask"]
    #[inline(always)]
    pub fn chid3(&self) -> CHID3_R {
        CHID3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fault Protection Trigger on Channel 0 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid0(&self) -> FCHID0_R {
        FCHID0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fault Protection Trigger on Channel 1 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid1(&self) -> FCHID1_R {
        FCHID1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fault Protection Trigger on Channel 2 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid2(&self) -> FCHID2_R {
        FCHID2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fault Protection Trigger on Channel 3 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid3(&self) -> FCHID3_R {
        FCHID3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
#[doc = "PWM Interrupt Mask Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_imr1](index.html) module"]
pub struct PWM_IMR1_SPEC;
impl crate::RegisterSpec for PWM_IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_imr1::R](R) reader structure"]
impl crate::Readable for PWM_IMR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWM_IMR1 to value 0"]
impl crate::Resettable for PWM_IMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
