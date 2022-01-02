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
#[doc = "Field `SWRST` reader - Software Reset Synchronization Busy"]
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
#[doc = "Field `ENABLE` reader - Enable Synchronization Busy"]
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
#[doc = "Field `COMPCTRL0` reader - COMPCTRL 0 Synchronization Busy"]
pub struct COMPCTRL0_R(crate::FieldReader<bool, bool>);
impl COMPCTRL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPCTRL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPCTRL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPCTRL1` reader - COMPCTRL 1 Synchronization Busy"]
pub struct COMPCTRL1_R(crate::FieldReader<bool, bool>);
impl COMPCTRL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPCTRL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPCTRL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset Synchronization Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WINCTRL Synchronization Busy"]
    #[inline(always)]
    pub fn winctrl(&self) -> WINCTRL_R {
        WINCTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - COMPCTRL 0 Synchronization Busy"]
    #[inline(always)]
    pub fn compctrl0(&self) -> COMPCTRL0_R {
        COMPCTRL0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - COMPCTRL 1 Synchronization Busy"]
    #[inline(always)]
    pub fn compctrl1(&self) -> COMPCTRL1_R {
        COMPCTRL1_R::new(((self.bits >> 4) & 0x01) != 0)
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
