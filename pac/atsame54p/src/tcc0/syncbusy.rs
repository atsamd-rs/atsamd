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
#[doc = "Field `SWRST` reader - Swrst Busy"]
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
#[doc = "Field `ENABLE` reader - Enable Busy"]
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
#[doc = "Field `CTRLB` reader - Ctrlb Busy"]
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
#[doc = "Field `STATUS` reader - Status Busy"]
pub struct STATUS_R(crate::FieldReader<bool, bool>);
impl STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNT` reader - Count Busy"]
pub struct COUNT_R(crate::FieldReader<bool, bool>);
impl COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATT` reader - Pattern Busy"]
pub struct PATT_R(crate::FieldReader<bool, bool>);
impl PATT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVE` reader - Wave Busy"]
pub struct WAVE_R(crate::FieldReader<bool, bool>);
impl WAVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER` reader - Period Busy"]
pub struct PER_R(crate::FieldReader<bool, bool>);
impl PER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC0` reader - Compare Channel 0 Busy"]
pub struct CC0_R(crate::FieldReader<bool, bool>);
impl CC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC1` reader - Compare Channel 1 Busy"]
pub struct CC1_R(crate::FieldReader<bool, bool>);
impl CC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC2` reader - Compare Channel 2 Busy"]
pub struct CC2_R(crate::FieldReader<bool, bool>);
impl CC2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC3` reader - Compare Channel 3 Busy"]
pub struct CC3_R(crate::FieldReader<bool, bool>);
impl CC3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC4` reader - Compare Channel 4 Busy"]
pub struct CC4_R(crate::FieldReader<bool, bool>);
impl CC4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC5` reader - Compare Channel 5 Busy"]
pub struct CC5_R(crate::FieldReader<bool, bool>);
impl CC5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Swrst Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Ctrlb Busy"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CTRLB_R {
        CTRLB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status Busy"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Count Busy"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pattern Busy"]
    #[inline(always)]
    pub fn patt(&self) -> PATT_R {
        PATT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wave Busy"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Period Busy"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Compare Channel 0 Busy"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Compare Channel 1 Busy"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Compare Channel 2 Busy"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Compare Channel 3 Busy"]
    #[inline(always)]
    pub fn cc3(&self) -> CC3_R {
        CC3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Compare Channel 4 Busy"]
    #[inline(always)]
    pub fn cc4(&self) -> CC4_R {
        CC4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Compare Channel 5 Busy"]
    #[inline(always)]
    pub fn cc5(&self) -> CC5_R {
        CC5_R::new(((self.bits >> 13) & 0x01) != 0)
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
