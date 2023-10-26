#[doc = "Register `RCAUSE` reader"]
pub type R = crate::R<RCAUSE_SPEC>;
#[doc = "Field `POR` reader - Power On Reset"]
pub type POR_R = crate::BitReader;
#[doc = "Field `BODCORE` reader - Brown Out CORE Detector Reset"]
pub type BODCORE_R = crate::BitReader;
#[doc = "Field `BODVDD` reader - Brown Out VDD Detector Reset"]
pub type BODVDD_R = crate::BitReader;
#[doc = "Field `NVM` reader - NVM Reset"]
pub type NVM_R = crate::BitReader;
#[doc = "Field `EXT` reader - External Reset"]
pub type EXT_R = crate::BitReader;
#[doc = "Field `WDT` reader - Watchdog Reset"]
pub type WDT_R = crate::BitReader;
#[doc = "Field `SYST` reader - System Reset Request"]
pub type SYST_R = crate::BitReader;
#[doc = "Field `BACKUP` reader - Backup Reset"]
pub type BACKUP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Power On Reset"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Brown Out CORE Detector Reset"]
    #[inline(always)]
    pub fn bodcore(&self) -> BODCORE_R {
        BODCORE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Brown Out VDD Detector Reset"]
    #[inline(always)]
    pub fn bodvdd(&self) -> BODVDD_R {
        BODVDD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NVM Reset"]
    #[inline(always)]
    pub fn nvm(&self) -> NVM_R {
        NVM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Reset"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Watchdog Reset"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - System Reset Request"]
    #[inline(always)]
    pub fn syst(&self) -> SYST_R {
        SYST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Backup Reset"]
    #[inline(always)]
    pub fn backup(&self) -> BACKUP_R {
        BACKUP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Reset Cause\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcause::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCAUSE_SPEC;
impl crate::RegisterSpec for RCAUSE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rcause::R`](R) reader structure"]
impl crate::Readable for RCAUSE_SPEC {}
#[doc = "`reset()` method sets RCAUSE to value 0"]
impl crate::Resettable for RCAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
