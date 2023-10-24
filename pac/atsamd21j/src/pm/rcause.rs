#[doc = "Register `RCAUSE` reader"]
pub type R = crate::R<RCAUSE_SPEC>;
#[doc = "Field `POR` reader - Power On Reset"]
pub type POR_R = crate::BitReader;
#[doc = "Field `BOD12` reader - Brown Out 12 Detector Reset"]
pub type BOD12_R = crate::BitReader;
#[doc = "Field `BOD33` reader - Brown Out 33 Detector Reset"]
pub type BOD33_R = crate::BitReader;
#[doc = "Field `EXT` reader - External Reset"]
pub type EXT_R = crate::BitReader;
#[doc = "Field `WDT` reader - Watchdog Reset"]
pub type WDT_R = crate::BitReader;
#[doc = "Field `SYST` reader - System Reset Request"]
pub type SYST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Power On Reset"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Brown Out 12 Detector Reset"]
    #[inline(always)]
    pub fn bod12(&self) -> BOD12_R {
        BOD12_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Brown Out 33 Detector Reset"]
    #[inline(always)]
    pub fn bod33(&self) -> BOD33_R {
        BOD33_R::new(((self.bits >> 2) & 1) != 0)
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
}
#[doc = "Reset Cause\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcause::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCAUSE_SPEC;
impl crate::RegisterSpec for RCAUSE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rcause::R`](R) reader structure"]
impl crate::Readable for RCAUSE_SPEC {}
#[doc = "`reset()` method sets RCAUSE to value 0x01"]
impl crate::Resettable for RCAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
