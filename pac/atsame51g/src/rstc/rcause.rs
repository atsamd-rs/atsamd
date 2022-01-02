#[doc = "Register `RCAUSE` reader"]
pub struct R(crate::R<RCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `POR` reader - Power On Reset"]
pub struct POR_R(crate::FieldReader<bool, bool>);
impl POR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODCORE` reader - Brown Out CORE Detector Reset"]
pub struct BODCORE_R(crate::FieldReader<bool, bool>);
impl BODCORE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODCORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODCORE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODVDD` reader - Brown Out VDD Detector Reset"]
pub struct BODVDD_R(crate::FieldReader<bool, bool>);
impl BODVDD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODVDD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODVDD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVM` reader - NVM Reset"]
pub struct NVM_R(crate::FieldReader<bool, bool>);
impl NVM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NVM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT` reader - External Reset"]
pub struct EXT_R(crate::FieldReader<bool, bool>);
impl EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT` reader - Watchdog Reset"]
pub struct WDT_R(crate::FieldReader<bool, bool>);
impl WDT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYST` reader - System Reset Request"]
pub struct SYST_R(crate::FieldReader<bool, bool>);
impl SYST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP` reader - Backup Reset"]
pub struct BACKUP_R(crate::FieldReader<bool, bool>);
impl BACKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BACKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Power On Reset"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Brown Out CORE Detector Reset"]
    #[inline(always)]
    pub fn bodcore(&self) -> BODCORE_R {
        BODCORE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Brown Out VDD Detector Reset"]
    #[inline(always)]
    pub fn bodvdd(&self) -> BODVDD_R {
        BODVDD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NVM Reset"]
    #[inline(always)]
    pub fn nvm(&self) -> NVM_R {
        NVM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Reset"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Watchdog Reset"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - System Reset Request"]
    #[inline(always)]
    pub fn syst(&self) -> SYST_R {
        SYST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Backup Reset"]
    #[inline(always)]
    pub fn backup(&self) -> BACKUP_R {
        BACKUP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Reset Cause\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcause](index.html) module"]
pub struct RCAUSE_SPEC;
impl crate::RegisterSpec for RCAUSE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rcause::R](R) reader structure"]
impl crate::Readable for RCAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RCAUSE to value 0"]
impl crate::Resettable for RCAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
