#[doc = "Reader of register RCAUSE"]
pub type R = crate::R<u8, super::RCAUSE>;
#[doc = "Reader of field `POR`"]
pub type POR_R = crate::R<bool, bool>;
#[doc = "Reader of field `BODCORE`"]
pub type BODCORE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BODVDD`"]
pub type BODVDD_R = crate::R<bool, bool>;
#[doc = "Reader of field `NVM`"]
pub type NVM_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXT`"]
pub type EXT_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDT`"]
pub type WDT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYST`"]
pub type SYST_R = crate::R<bool, bool>;
#[doc = "Reader of field `BACKUP`"]
pub type BACKUP_R = crate::R<bool, bool>;
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
