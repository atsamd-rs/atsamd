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
#[doc = "Field `PER` reader - Period busy"]
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
#[doc = "Field `PATTB` reader - Pattern Buffer Busy"]
pub struct PATTB_R(crate::FieldReader<bool, bool>);
impl PATTB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATTB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVEB` reader - Wave Buffer Busy"]
pub struct WAVEB_R(crate::FieldReader<bool, bool>);
impl WAVEB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAVEB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAVEB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERB` reader - Period Buffer Busy"]
pub struct PERB_R(crate::FieldReader<bool, bool>);
impl PERB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PERB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCB0` reader - Compare Channel Buffer 0 Busy"]
pub struct CCB0_R(crate::FieldReader<bool, bool>);
impl CCB0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCB1` reader - Compare Channel Buffer 1 Busy"]
pub struct CCB1_R(crate::FieldReader<bool, bool>);
impl CCB1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCB2` reader - Compare Channel Buffer 2 Busy"]
pub struct CCB2_R(crate::FieldReader<bool, bool>);
impl CCB2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCB3` reader - Compare Channel Buffer 3 Busy"]
pub struct CCB3_R(crate::FieldReader<bool, bool>);
impl CCB3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCB3_R {
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
    #[doc = "Bit 7 - Period busy"]
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
    #[doc = "Bit 16 - Pattern Buffer Busy"]
    #[inline(always)]
    pub fn pattb(&self) -> PATTB_R {
        PATTB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Wave Buffer Busy"]
    #[inline(always)]
    pub fn waveb(&self) -> WAVEB_R {
        WAVEB_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Period Buffer Busy"]
    #[inline(always)]
    pub fn perb(&self) -> PERB_R {
        PERB_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Compare Channel Buffer 0 Busy"]
    #[inline(always)]
    pub fn ccb0(&self) -> CCB0_R {
        CCB0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Compare Channel Buffer 1 Busy"]
    #[inline(always)]
    pub fn ccb1(&self) -> CCB1_R {
        CCB1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Compare Channel Buffer 2 Busy"]
    #[inline(always)]
    pub fn ccb2(&self) -> CCB2_R {
        CCB2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Compare Channel Buffer 3 Busy"]
    #[inline(always)]
    pub fn ccb3(&self) -> CCB3_R {
        CCB3_R::new(((self.bits >> 22) & 0x01) != 0)
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
