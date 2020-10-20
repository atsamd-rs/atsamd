#[doc = "Reader of register RCAUSE"]
pub type R = crate::R<u8, super::RCAUSE>;
#[doc = "Reader of field `POR`"]
pub type POR_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD12`"]
pub type BOD12_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD33`"]
pub type BOD33_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXT`"]
pub type EXT_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDT`"]
pub type WDT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYST`"]
pub type SYST_R = crate::R<bool, bool>;
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
