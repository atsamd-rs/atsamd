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
#[doc = "Field `CTRLC` reader - CTRLC Synchronization Busy"]
pub struct CTRLC_R(crate::FieldReader<bool, bool>);
impl CTRLC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTRLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPUTCTRL` reader - INPUTCTRL Synchronization Busy"]
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
#[doc = "Field `WINCTRL` reader - WINCTRL Synchronization Busy"]
pub struct WINCTRL_R(crate::FieldReader<bool, bool>);
impl WINCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WINCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINLT` reader - WINLT Synchronization Busy"]
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
#[doc = "Field `WINUT` reader - WINUT Synchronization Busy"]
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
#[doc = "Field `OFFSETCORR` reader - OFFSETCTRL Synchronization Busy"]
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
#[doc = "Field `GAINCORR` reader - GAINCORR Synchronization Busy"]
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
#[doc = "Field `SHIFTCORR` reader - SHIFTCORR Synchronization Busy"]
pub struct SHIFTCORR_R(crate::FieldReader<bool, bool>);
impl SHIFTCORR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHIFTCORR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFTCORR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTRIG` reader - SWTRG Synchronization Busy"]
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
#[doc = "Field `ANACTRL` reader - ANACTRL Synchronization Busy"]
pub struct ANACTRL_R(crate::FieldReader<bool, bool>);
impl ANACTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ANACTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANACTRL_R {
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
    #[doc = "Bit 2 - CTRLC Synchronization Busy"]
    #[inline(always)]
    pub fn ctrlc(&self) -> CTRLC_R {
        CTRLC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - INPUTCTRL Synchronization Busy"]
    #[inline(always)]
    pub fn inputctrl(&self) -> INPUTCTRL_R {
        INPUTCTRL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WINCTRL Synchronization Busy"]
    #[inline(always)]
    pub fn winctrl(&self) -> WINCTRL_R {
        WINCTRL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WINLT Synchronization Busy"]
    #[inline(always)]
    pub fn winlt(&self) -> WINLT_R {
        WINLT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WINUT Synchronization Busy"]
    #[inline(always)]
    pub fn winut(&self) -> WINUT_R {
        WINUT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - OFFSETCTRL Synchronization Busy"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GAINCORR Synchronization Busy"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SHIFTCORR Synchronization Busy"]
    #[inline(always)]
    pub fn shiftcorr(&self) -> SHIFTCORR_R {
        SHIFTCORR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SWTRG Synchronization Busy"]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ANACTRL Synchronization Busy"]
    #[inline(always)]
    pub fn anactrl(&self) -> ANACTRL_R {
        ANACTRL_R::new(((self.bits >> 11) & 0x01) != 0)
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
