#[doc = "Register `STATUSA` reader"]
pub type R = crate::R<STATUSA_SPEC>;
#[doc = "Field `PAC_` reader - PAC APB Protect Enable"]
pub type PAC__R = crate::BitReader;
#[doc = "Field `PM_` reader - PM APB Protect Enable"]
pub type PM__R = crate::BitReader;
#[doc = "Field `MCLK_` reader - MCLK APB Protect Enable"]
pub type MCLK__R = crate::BitReader;
#[doc = "Field `RSTC_` reader - RSTC APB Protect Enable"]
pub type RSTC__R = crate::BitReader;
#[doc = "Field `OSCCTRL_` reader - OSCCTRL APB Protect Enable"]
pub type OSCCTRL__R = crate::BitReader;
#[doc = "Field `OSC32KCTRL_` reader - OSC32KCTRL APB Protect Enable"]
pub type OSC32KCTRL__R = crate::BitReader;
#[doc = "Field `SUPC_` reader - SUPC APB Protect Enable"]
pub type SUPC__R = crate::BitReader;
#[doc = "Field `GCLK_` reader - GCLK APB Protect Enable"]
pub type GCLK__R = crate::BitReader;
#[doc = "Field `WDT_` reader - WDT APB Protect Enable"]
pub type WDT__R = crate::BitReader;
#[doc = "Field `RTC_` reader - RTC APB Protect Enable"]
pub type RTC__R = crate::BitReader;
#[doc = "Field `EIC_` reader - EIC APB Protect Enable"]
pub type EIC__R = crate::BitReader;
#[doc = "Field `FREQM_` reader - FREQM APB Protect Enable"]
pub type FREQM__R = crate::BitReader;
#[doc = "Field `SERCOM0_` reader - SERCOM0 APB Protect Enable"]
pub type SERCOM0__R = crate::BitReader;
#[doc = "Field `SERCOM1_` reader - SERCOM1 APB Protect Enable"]
pub type SERCOM1__R = crate::BitReader;
#[doc = "Field `TC0_` reader - TC0 APB Protect Enable"]
pub type TC0__R = crate::BitReader;
#[doc = "Field `TC1_` reader - TC1 APB Protect Enable"]
pub type TC1__R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PAC APB Protect Enable"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PM APB Protect Enable"]
    #[inline(always)]
    pub fn pm_(&self) -> PM__R {
        PM__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCLK APB Protect Enable"]
    #[inline(always)]
    pub fn mclk_(&self) -> MCLK__R {
        MCLK__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSTC APB Protect Enable"]
    #[inline(always)]
    pub fn rstc_(&self) -> RSTC__R {
        RSTC__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OSCCTRL APB Protect Enable"]
    #[inline(always)]
    pub fn oscctrl_(&self) -> OSCCTRL__R {
        OSCCTRL__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OSC32KCTRL APB Protect Enable"]
    #[inline(always)]
    pub fn osc32kctrl_(&self) -> OSC32KCTRL__R {
        OSC32KCTRL__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SUPC APB Protect Enable"]
    #[inline(always)]
    pub fn supc_(&self) -> SUPC__R {
        SUPC__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GCLK APB Protect Enable"]
    #[inline(always)]
    pub fn gclk_(&self) -> GCLK__R {
        GCLK__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WDT APB Protect Enable"]
    #[inline(always)]
    pub fn wdt_(&self) -> WDT__R {
        WDT__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RTC APB Protect Enable"]
    #[inline(always)]
    pub fn rtc_(&self) -> RTC__R {
        RTC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EIC APB Protect Enable"]
    #[inline(always)]
    pub fn eic_(&self) -> EIC__R {
        EIC__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FREQM APB Protect Enable"]
    #[inline(always)]
    pub fn freqm_(&self) -> FREQM__R {
        FREQM__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SERCOM0 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SERCOM1 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusa::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUSA_SPEC;
impl crate::RegisterSpec for STATUSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusa::R`](R) reader structure"]
impl crate::Readable for STATUSA_SPEC {}
#[doc = "`reset()` method sets STATUSA to value 0x0001_0000"]
impl crate::Resettable for STATUSA_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
