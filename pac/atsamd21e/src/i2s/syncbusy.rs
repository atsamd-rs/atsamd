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
#[doc = "Field `SWRST` reader - Software Reset Synchronization Status"]
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
#[doc = "Field `ENABLE` reader - Enable Synchronization Status"]
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
#[doc = "Field `CKEN0` reader - Clock Unit 0 Enable Synchronization Status"]
pub struct CKEN0_R(crate::FieldReader<bool, bool>);
impl CKEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CKEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKEN1` reader - Clock Unit 1 Enable Synchronization Status"]
pub struct CKEN1_R(crate::FieldReader<bool, bool>);
impl CKEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CKEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEREN0` reader - Serializer 0 Enable Synchronization Status"]
pub struct SEREN0_R(crate::FieldReader<bool, bool>);
impl SEREN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEREN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEREN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEREN1` reader - Serializer 1 Enable Synchronization Status"]
pub struct SEREN1_R(crate::FieldReader<bool, bool>);
impl SEREN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEREN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEREN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA0` reader - Data 0 Synchronization Status"]
pub struct DATA0_R(crate::FieldReader<bool, bool>);
impl DATA0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA1` reader - Data 1 Synchronization Status"]
pub struct DATA1_R(crate::FieldReader<bool, bool>);
impl DATA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset Synchronization Status"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Synchronization Status"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clock Unit 0 Enable Synchronization Status"]
    #[inline(always)]
    pub fn cken0(&self) -> CKEN0_R {
        CKEN0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clock Unit 1 Enable Synchronization Status"]
    #[inline(always)]
    pub fn cken1(&self) -> CKEN1_R {
        CKEN1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Serializer 0 Enable Synchronization Status"]
    #[inline(always)]
    pub fn seren0(&self) -> SEREN0_R {
        SEREN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Serializer 1 Enable Synchronization Status"]
    #[inline(always)]
    pub fn seren1(&self) -> SEREN1_R {
        SEREN1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Data 0 Synchronization Status"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data 1 Synchronization Status"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
#[doc = "Synchronization Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u16;
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
