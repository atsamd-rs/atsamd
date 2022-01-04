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
#[doc = "Field `SWRST` reader - Software Reset Bit Busy"]
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
#[doc = "Field `ENABLE` reader - Enable Bit Busy"]
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
#[doc = "Field `FREQCORR` reader - FREQCORR Register Busy"]
pub struct FREQCORR_R(crate::FieldReader<bool, bool>);
impl FREQCORR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FREQCORR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQCORR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOCK` reader - CLOCK Register Busy"]
pub struct CLOCK_R(crate::FieldReader<bool, bool>);
impl CLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARM0` reader - ALARM 0 Register Busy"]
pub struct ALARM0_R(crate::FieldReader<bool, bool>);
impl ALARM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALARM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARM1` reader - ALARM 1 Register Busy"]
pub struct ALARM1_R(crate::FieldReader<bool, bool>);
impl ALARM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALARM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK0` reader - MASK 0 Register Busy"]
pub struct MASK0_R(crate::FieldReader<bool, bool>);
impl MASK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK1` reader - MASK 1 Register Busy"]
pub struct MASK1_R(crate::FieldReader<bool, bool>);
impl MASK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOCKSYNC` reader - Clock Synchronization Enable Bit Busy"]
pub struct CLOCKSYNC_R(crate::FieldReader<bool, bool>);
impl CLOCKSYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLOCKSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOCKSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP0` reader - General Purpose 0 Register Busy"]
pub struct GP0_R(crate::FieldReader<bool, bool>);
impl GP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP1` reader - General Purpose 1 Register Busy"]
pub struct GP1_R(crate::FieldReader<bool, bool>);
impl GP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP2` reader - General Purpose 2 Register Busy"]
pub struct GP2_R(crate::FieldReader<bool, bool>);
impl GP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP3` reader - General Purpose 3 Register Busy"]
pub struct GP3_R(crate::FieldReader<bool, bool>);
impl GP3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset Bit Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Bit Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FREQCORR Register Busy"]
    #[inline(always)]
    pub fn freqcorr(&self) -> FREQCORR_R {
        FREQCORR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CLOCK Register Busy"]
    #[inline(always)]
    pub fn clock(&self) -> CLOCK_R {
        CLOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ALARM 0 Register Busy"]
    #[inline(always)]
    pub fn alarm0(&self) -> ALARM0_R {
        ALARM0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ALARM 1 Register Busy"]
    #[inline(always)]
    pub fn alarm1(&self) -> ALARM1_R {
        ALARM1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MASK 0 Register Busy"]
    #[inline(always)]
    pub fn mask0(&self) -> MASK0_R {
        MASK0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MASK 1 Register Busy"]
    #[inline(always)]
    pub fn mask1(&self) -> MASK1_R {
        MASK1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Clock Synchronization Enable Bit Busy"]
    #[inline(always)]
    pub fn clocksync(&self) -> CLOCKSYNC_R {
        CLOCKSYNC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - General Purpose 0 Register Busy"]
    #[inline(always)]
    pub fn gp0(&self) -> GP0_R {
        GP0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - General Purpose 1 Register Busy"]
    #[inline(always)]
    pub fn gp1(&self) -> GP1_R {
        GP1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - General Purpose 2 Register Busy"]
    #[inline(always)]
    pub fn gp2(&self) -> GP2_R {
        GP2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - General Purpose 3 Register Busy"]
    #[inline(always)]
    pub fn gp3(&self) -> GP3_R {
        GP3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
#[doc = "MODE2 Synchronization Busy Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
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
