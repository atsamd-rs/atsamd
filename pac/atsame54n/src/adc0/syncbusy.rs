#[doc = "Register `SYNCBUSY` reader"]
pub struct R(crate::R<SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SWRST` reader - SWRST Synchronization Busy"]
pub struct SWRST_R(crate::FieldReader<bool, bool>);
impl SWRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` reader - ENABLE Synchronization Busy"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPUTCTRL` reader - Input Control Synchronization Busy"]
pub struct INPUTCTRL_R(crate::FieldReader<bool, bool>);
impl INPUTCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INPUTCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUTCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLB` reader - Control B Synchronization Busy"]
pub struct CTRLB_R(crate::FieldReader<bool, bool>);
impl CTRLB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTRLB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFCTRL` reader - Reference Control Synchronization Busy"]
pub struct REFCTRL_R(crate::FieldReader<bool, bool>);
impl REFCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REFCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVGCTRL` reader - Average Control Synchronization Busy"]
pub struct AVGCTRL_R(crate::FieldReader<bool, bool>);
impl AVGCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AVGCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVGCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMPCTRL` reader - Sampling Time Control Synchronization Busy"]
pub struct SAMPCTRL_R(crate::FieldReader<bool, bool>);
impl SAMPCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAMPCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMPCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINLT` reader - Window Monitor Lower Threshold Synchronization Busy"]
pub struct WINLT_R(crate::FieldReader<bool, bool>);
impl WINLT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WINLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINLT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINUT` reader - Window Monitor Upper Threshold Synchronization Busy"]
pub struct WINUT_R(crate::FieldReader<bool, bool>);
impl WINUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WINUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAINCORR` reader - Gain Correction Synchronization Busy"]
pub struct GAINCORR_R(crate::FieldReader<bool, bool>);
impl GAINCORR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GAINCORR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAINCORR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSETCORR` reader - Offset Correction Synchronization Busy"]
pub struct OFFSETCORR_R(crate::FieldReader<bool, bool>);
impl OFFSETCORR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFFSETCORR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSETCORR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG` reader - Software Trigger Synchronization Busy"]
pub struct SWTRIG_R(crate::FieldReader<bool, bool>);
impl SWTRIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWTRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - SWRST Synchronization Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input Control Synchronization Busy"]
    #[inline(always)]
    pub fn inputctrl(&self) -> INPUTCTRL_R {
        INPUTCTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control B Synchronization Busy"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CTRLB_R {
        CTRLB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reference Control Synchronization Busy"]
    #[inline(always)]
    pub fn refctrl(&self) -> REFCTRL_R {
        REFCTRL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Average Control Synchronization Busy"]
    #[inline(always)]
    pub fn avgctrl(&self) -> AVGCTRL_R {
        AVGCTRL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sampling Time Control Synchronization Busy"]
    #[inline(always)]
    pub fn sampctrl(&self) -> SAMPCTRL_R {
        SAMPCTRL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Window Monitor Lower Threshold Synchronization Busy"]
    #[inline(always)]
    pub fn winlt(&self) -> WINLT_R {
        WINLT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Window Monitor Upper Threshold Synchronization Busy"]
    #[inline(always)]
    pub fn winut(&self) -> WINUT_R {
        WINUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Gain Correction Synchronization Busy"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Offset Correction Synchronization Busy"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software Trigger Synchronization Busy"]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
#[doc = "Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
