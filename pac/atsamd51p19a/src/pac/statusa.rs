#[doc = "Reader of register STATUSA"]
pub type R = crate::R<u32, super::STATUSA>;
#[doc = "Reader of field `PAC_`"]
pub type PAC__R = crate::R<bool, bool>;
#[doc = "Reader of field `PM_`"]
pub type PM__R = crate::R<bool, bool>;
#[doc = "Reader of field `MCLK_`"]
pub type MCLK__R = crate::R<bool, bool>;
#[doc = "Reader of field `RSTC_`"]
pub type RSTC__R = crate::R<bool, bool>;
#[doc = "Reader of field `OSCCTRL_`"]
pub type OSCCTRL__R = crate::R<bool, bool>;
#[doc = "Reader of field `OSC32KCTRL_`"]
pub type OSC32KCTRL__R = crate::R<bool, bool>;
#[doc = "Reader of field `SUPC_`"]
pub type SUPC__R = crate::R<bool, bool>;
#[doc = "Reader of field `GCLK_`"]
pub type GCLK__R = crate::R<bool, bool>;
#[doc = "Reader of field `WDT_`"]
pub type WDT__R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_`"]
pub type RTC__R = crate::R<bool, bool>;
#[doc = "Reader of field `EIC_`"]
pub type EIC__R = crate::R<bool, bool>;
#[doc = "Reader of field `FREQM_`"]
pub type FREQM__R = crate::R<bool, bool>;
#[doc = "Reader of field `SERCOM0_`"]
pub type SERCOM0__R = crate::R<bool, bool>;
#[doc = "Reader of field `SERCOM1_`"]
pub type SERCOM1__R = crate::R<bool, bool>;
#[doc = "Reader of field `TC0_`"]
pub type TC0__R = crate::R<bool, bool>;
#[doc = "Reader of field `TC1_`"]
pub type TC1__R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - PAC APB Protect Enable"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PM APB Protect Enable"]
    #[inline(always)]
    pub fn pm_(&self) -> PM__R {
        PM__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MCLK APB Protect Enable"]
    #[inline(always)]
    pub fn mclk_(&self) -> MCLK__R {
        MCLK__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RSTC APB Protect Enable"]
    #[inline(always)]
    pub fn rstc_(&self) -> RSTC__R {
        RSTC__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OSCCTRL APB Protect Enable"]
    #[inline(always)]
    pub fn oscctrl_(&self) -> OSCCTRL__R {
        OSCCTRL__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OSC32KCTRL APB Protect Enable"]
    #[inline(always)]
    pub fn osc32kctrl_(&self) -> OSC32KCTRL__R {
        OSC32KCTRL__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SUPC APB Protect Enable"]
    #[inline(always)]
    pub fn supc_(&self) -> SUPC__R {
        SUPC__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GCLK APB Protect Enable"]
    #[inline(always)]
    pub fn gclk_(&self) -> GCLK__R {
        GCLK__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - WDT APB Protect Enable"]
    #[inline(always)]
    pub fn wdt_(&self) -> WDT__R {
        WDT__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RTC APB Protect Enable"]
    #[inline(always)]
    pub fn rtc_(&self) -> RTC__R {
        RTC__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EIC APB Protect Enable"]
    #[inline(always)]
    pub fn eic_(&self) -> EIC__R {
        EIC__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - FREQM APB Protect Enable"]
    #[inline(always)]
    pub fn freqm_(&self) -> FREQM__R {
        FREQM__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SERCOM0 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SERCOM1 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
