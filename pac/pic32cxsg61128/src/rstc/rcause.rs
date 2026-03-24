#[doc = "Register `RCAUSE` reader"]
pub type R = crate::R<RcauseSpec>;
#[doc = "Field `POR` reader - Power On Reset"]
pub type PorR = crate::BitReader;
#[doc = "Field `BOD12` reader - BOD12 Reset"]
pub type Bod12R = crate::BitReader;
#[doc = "Field `BOD33` reader - BOD33 Reset"]
pub type Bod33R = crate::BitReader;
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
    #[doc = "Bit 1 - BOD12 Reset"]
    #[inline(always)]
    pub fn bod12(&self) -> Bod12R {
        Bod12R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOD33 Reset"]
    #[inline(always)]
    pub fn bod33(&self) -> Bod33R {
        Bod33R::new(((self.bits >> 2) & 1) != 0)
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
impl crate::Resettable for RcauseSpec {
    const RESET_VALUE: u8 = 0;
}
