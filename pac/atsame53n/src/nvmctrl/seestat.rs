#[doc = "Register `SEESTAT` reader"]
pub struct R(crate::R<SEESTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEESTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEESTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEESTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ASEES` reader - Active SmartEEPROM Sector"]
pub struct ASEES_R(crate::FieldReader<bool, bool>);
impl ASEES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ASEES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASEES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOAD` reader - Page Buffer Loaded"]
pub struct LOAD_R(crate::FieldReader<bool, bool>);
impl LOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` reader - Busy"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` reader - SmartEEPROM Write Access Is Locked"]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLOCK` reader - SmartEEPROM Write Access To Register Address Space Is Locked"]
pub struct RLOCK_R(crate::FieldReader<bool, bool>);
impl RLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBLK` reader - Blocks Number In a Sector"]
pub struct SBLK_R(crate::FieldReader<u8, u8>);
impl SBLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SBLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBLK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSZ` reader - SmartEEPROM Page Size"]
pub struct PSZ_R(crate::FieldReader<u8, u8>);
impl PSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Active SmartEEPROM Sector"]
    #[inline(always)]
    pub fn asees(&self) -> ASEES_R {
        ASEES_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Page Buffer Loaded"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SmartEEPROM Write Access Is Locked"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SmartEEPROM Write Access To Register Address Space Is Locked"]
    #[inline(always)]
    pub fn rlock(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Blocks Number In a Sector"]
    #[inline(always)]
    pub fn sblk(&self) -> SBLK_R {
        SBLK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - SmartEEPROM Page Size"]
    #[inline(always)]
    pub fn psz(&self) -> PSZ_R {
        PSZ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
#[doc = "SmartEEPROM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seestat](index.html) module"]
pub struct SEESTAT_SPEC;
impl crate::RegisterSpec for SEESTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seestat::R](R) reader structure"]
impl crate::Readable for SEESTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SEESTAT to value 0"]
impl crate::Resettable for SEESTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
