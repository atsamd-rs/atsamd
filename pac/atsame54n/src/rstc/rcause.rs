#[doc = "Register `RCAUSE` reader"]
pub type R = crate::R<RcauseSpec>;
#[doc = "Field `POR` reader - Power On Reset"]
pub type PorR = crate::BitReader;
#[doc = "Field `BODCORE` reader - Brown Out CORE Detector Reset"]
pub type BodcoreR = crate::BitReader;
#[doc = "Field `BODVDD` reader - Brown Out VDD Detector Reset"]
pub type BodvddR = crate::BitReader;
#[doc = "Field `NVM` reader - NVM Reset"]
pub type NvmR = crate::BitReader;
#[doc = "Field `EXT` reader - External Reset"]
pub type ExtR = crate::BitReader;
#[doc = "Field `WDT` reader - Watchdog Reset"]
pub type WdtR = crate::BitReader;
#[doc = "Field `SYST` reader - System Reset Request"]
pub type SystR = crate::BitReader;
#[doc = "Field `BACKUP` reader - Backup Reset"]
pub type BackupR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Power On Reset"]
    #[inline(always)]
    pub fn por(&self) -> PorR {
        PorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Brown Out CORE Detector Reset"]
    #[inline(always)]
    pub fn bodcore(&self) -> BodcoreR {
        BodcoreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Brown Out VDD Detector Reset"]
    #[inline(always)]
    pub fn bodvdd(&self) -> BodvddR {
        BodvddR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NVM Reset"]
    #[inline(always)]
    pub fn nvm(&self) -> NvmR {
        NvmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Reset"]
    #[inline(always)]
    pub fn ext(&self) -> ExtR {
        ExtR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Watchdog Reset"]
    #[inline(always)]
    pub fn wdt(&self) -> WdtR {
        WdtR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - System Reset Request"]
    #[inline(always)]
    pub fn syst(&self) -> SystR {
        SystR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Backup Reset"]
    #[inline(always)]
    pub fn backup(&self) -> BackupR {
        BackupR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Reset Cause\n\nYou can [`read`](crate::Reg::read) this register and get [`rcause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcauseSpec;
impl crate::RegisterSpec for RcauseSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rcause::R`](R) reader structure"]
impl crate::Readable for RcauseSpec {}
#[doc = "`reset()` method sets RCAUSE to value 0"]
impl crate::Resettable for RcauseSpec {}
