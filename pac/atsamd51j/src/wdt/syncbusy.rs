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
#[doc = "Field `WEN` reader - Window Enable Synchronization Busy"]
pub struct WEN_R(crate::FieldReader<bool, bool>);
impl WEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALWAYSON` reader - Always-On Synchronization Busy"]
pub struct ALWAYSON_R(crate::FieldReader<bool, bool>);
impl ALWAYSON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALWAYSON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALWAYSON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLEAR` reader - Clear Synchronization Busy"]
pub struct CLEAR_R(crate::FieldReader<bool, bool>);
impl CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Enable Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Window Enable Synchronization Busy"]
    #[inline(always)]
    pub fn wen(&self) -> WEN_R {
        WEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Always-On Synchronization Busy"]
    #[inline(always)]
    pub fn alwayson(&self) -> ALWAYSON_R {
        ALWAYSON_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear Synchronization Busy"]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 4) & 0x01) != 0)
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
