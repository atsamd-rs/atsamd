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
#[doc = "Field `SWRST` reader - Software Reset Synchroniation Busy bit"]
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
#[doc = "Field `GENCTRL0` reader - Generic Clock Generator Control 0 Synchronization Busy bits"]
pub struct GENCTRL0_R(crate::FieldReader<bool, bool>);
impl GENCTRL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENCTRL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENCTRL1` reader - Generic Clock Generator Control 1 Synchronization Busy bits"]
pub struct GENCTRL1_R(crate::FieldReader<bool, bool>);
impl GENCTRL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENCTRL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENCTRL2` reader - Generic Clock Generator Control 2 Synchronization Busy bits"]
pub struct GENCTRL2_R(crate::FieldReader<bool, bool>);
impl GENCTRL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENCTRL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENCTRL3` reader - Generic Clock Generator Control 3 Synchronization Busy bits"]
pub struct GENCTRL3_R(crate::FieldReader<bool, bool>);
impl GENCTRL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENCTRL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENCTRL4` reader - Generic Clock Generator Control 4 Synchronization Busy bits"]
pub struct GENCTRL4_R(crate::FieldReader<bool, bool>);
impl GENCTRL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENCTRL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENCTRL5` reader - Generic Clock Generator Control 5 Synchronization Busy bits"]
pub struct GENCTRL5_R(crate::FieldReader<bool, bool>);
impl GENCTRL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENCTRL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENCTRL6` reader - Generic Clock Generator Control 6 Synchronization Busy bits"]
pub struct GENCTRL6_R(crate::FieldReader<bool, bool>);
impl GENCTRL6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENCTRL6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENCTRL7` reader - Generic Clock Generator Control 7 Synchronization Busy bits"]
pub struct GENCTRL7_R(crate::FieldReader<bool, bool>);
impl GENCTRL7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENCTRL7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENCTRL8` reader - Generic Clock Generator Control 8 Synchronization Busy bits"]
pub struct GENCTRL8_R(crate::FieldReader<bool, bool>);
impl GENCTRL8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GENCTRL8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENCTRL8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset Synchroniation Busy bit"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Generic Clock Generator Control 0 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl0(&self) -> GENCTRL0_R {
        GENCTRL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Generic Clock Generator Control 1 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl1(&self) -> GENCTRL1_R {
        GENCTRL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Generic Clock Generator Control 2 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl2(&self) -> GENCTRL2_R {
        GENCTRL2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Generic Clock Generator Control 3 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl3(&self) -> GENCTRL3_R {
        GENCTRL3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Generic Clock Generator Control 4 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl4(&self) -> GENCTRL4_R {
        GENCTRL4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Generic Clock Generator Control 5 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl5(&self) -> GENCTRL5_R {
        GENCTRL5_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Generic Clock Generator Control 6 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl6(&self) -> GENCTRL6_R {
        GENCTRL6_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Generic Clock Generator Control 7 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl7(&self) -> GENCTRL7_R {
        GENCTRL7_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Generic Clock Generator Control 8 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl8(&self) -> GENCTRL8_R {
        GENCTRL8_R::new(((self.bits >> 10) & 0x01) != 0)
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
