#[doc = "Register `ACTIVE` reader"]
pub struct R(crate::R<ACTIVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTIVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTIVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTIVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LVLEX0` reader - Level 0 Channel Trigger Request Executing"]
pub struct LVLEX0_R(crate::FieldReader<bool, bool>);
impl LVLEX0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVLEX0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVLEX0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVLEX1` reader - Level 1 Channel Trigger Request Executing"]
pub struct LVLEX1_R(crate::FieldReader<bool, bool>);
impl LVLEX1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVLEX1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVLEX1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVLEX2` reader - Level 2 Channel Trigger Request Executing"]
pub struct LVLEX2_R(crate::FieldReader<bool, bool>);
impl LVLEX2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVLEX2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVLEX2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVLEX3` reader - Level 3 Channel Trigger Request Executing"]
pub struct LVLEX3_R(crate::FieldReader<bool, bool>);
impl LVLEX3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVLEX3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVLEX3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID` reader - Active Channel ID"]
pub struct ID_R(crate::FieldReader<u8, u8>);
impl ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABUSY` reader - Active Channel Busy"]
pub struct ABUSY_R(crate::FieldReader<bool, bool>);
impl ABUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTCNT` reader - Active Channel Block Transfer Count"]
pub struct BTCNT_R(crate::FieldReader<u16, u16>);
impl BTCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Level 0 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex0(&self) -> LVLEX0_R {
        LVLEX0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Level 1 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex1(&self) -> LVLEX1_R {
        LVLEX1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Level 2 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex2(&self) -> LVLEX2_R {
        LVLEX2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Level 3 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex3(&self) -> LVLEX3_R {
        LVLEX3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Active Channel ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Active Channel Busy"]
    #[inline(always)]
    pub fn abusy(&self) -> ABUSY_R {
        ABUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Active Channel Block Transfer Count"]
    #[inline(always)]
    pub fn btcnt(&self) -> BTCNT_R {
        BTCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Active Channel and Levels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [active](index.html) module"]
pub struct ACTIVE_SPEC;
impl crate::RegisterSpec for ACTIVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [active::R](R) reader structure"]
impl crate::Readable for ACTIVE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACTIVE to value 0"]
impl crate::Resettable for ACTIVE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
