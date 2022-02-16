#[doc = "Register `DACC_CHSR` reader"]
pub struct R(crate::R<DACC_CHSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACC_CHSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACC_CHSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACC_CHSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0` reader - Channel 0 Status"]
pub struct CH0_R(crate::FieldReader<bool, bool>);
impl CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1` reader - Channel 1 Status"]
pub struct CH1_R(crate::FieldReader<bool, bool>);
impl CH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACRDY0` reader - DAC Ready Flag"]
pub struct DACRDY0_R(crate::FieldReader<bool, bool>);
impl DACRDY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DACRDY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACRDY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACRDY1` reader - DAC Ready Flag"]
pub struct DACRDY1_R(crate::FieldReader<bool, bool>);
impl DACRDY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DACRDY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACRDY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Status"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Status"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DAC Ready Flag"]
    #[inline(always)]
    pub fn dacrdy0(&self) -> DACRDY0_R {
        DACRDY0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DAC Ready Flag"]
    #[inline(always)]
    pub fn dacrdy1(&self) -> DACRDY1_R {
        DACRDY1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_chsr](index.html) module"]
pub struct DACC_CHSR_SPEC;
impl crate::RegisterSpec for DACC_CHSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dacc_chsr::R](R) reader structure"]
impl crate::Readable for DACC_CHSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DACC_CHSR to value 0"]
impl crate::Resettable for DACC_CHSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
