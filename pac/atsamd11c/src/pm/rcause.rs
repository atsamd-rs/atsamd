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
#[doc = "Field `BOD12` reader - Brown Out 12 Detector Reset"]
pub struct BOD12_R(crate::FieldReader<bool, bool>);
impl BOD12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOD12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD33` reader - Brown Out 33 Detector Reset"]
pub struct BOD33_R(crate::FieldReader<bool, bool>);
impl BOD33_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOD33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD33_R {
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
impl R {
    #[doc = "Bit 0 - Power On Reset"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Brown Out 12 Detector Reset"]
    #[inline(always)]
    pub fn bod12(&self) -> BOD12_R {
        BOD12_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Brown Out 33 Detector Reset"]
    #[inline(always)]
    pub fn bod33(&self) -> BOD33_R {
        BOD33_R::new(((self.bits >> 2) & 0x01) != 0)
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
#[doc = "`reset()` method sets RCAUSE to value 0x01"]
impl crate::Resettable for RCAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
