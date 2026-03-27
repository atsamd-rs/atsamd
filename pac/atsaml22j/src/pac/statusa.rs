#[doc = "Register `STATUSA` reader"]
pub struct R(crate::R<STATUSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PAC_` reader - PAC APB Protect Enable"]
pub type PAC__R = crate::BitReader<bool>;
#[doc = "Field `PM_` reader - PM APB Protect Enable"]
pub type PM__R = crate::BitReader<bool>;
#[doc = "Field `MCLK_` reader - MCLK APB Protect Enable"]
pub type MCLK__R = crate::BitReader<bool>;
#[doc = "Field `RSTC_` reader - RSTC APB Protect Enable"]
pub type RSTC__R = crate::BitReader<bool>;
#[doc = "Field `OSCCTRL_` reader - OSCCTRL APB Protect Enable"]
pub type OSCCTRL__R = crate::BitReader<bool>;
#[doc = "Field `OSC32KCTRL_` reader - OSC32KCTRL APB Protect Enable"]
pub type OSC32KCTRL__R = crate::BitReader<bool>;
#[doc = "Field `SUPC_` reader - SUPC APB Protect Enable"]
pub type SUPC__R = crate::BitReader<bool>;
#[doc = "Field `GCLK_` reader - GCLK APB Protect Enable"]
pub type GCLK__R = crate::BitReader<bool>;
#[doc = "Field `WDT_` reader - WDT APB Protect Enable"]
pub type WDT__R = crate::BitReader<bool>;
#[doc = "Field `RTC_` reader - RTC APB Protect Enable"]
pub type RTC__R = crate::BitReader<bool>;
#[doc = "Field `EIC_` reader - EIC APB Protect Enable"]
pub type EIC__R = crate::BitReader<bool>;
#[doc = "Field `FREQM_` reader - FREQM APB Protect Enable"]
pub type FREQM__R = crate::BitReader<bool>;
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
}
#[doc = "Peripheral write protection status - Bridge A\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusa](index.html) module"]
pub struct STATUSA_SPEC;
impl crate::RegisterSpec for STATUSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusa::R](R) reader structure"]
impl crate::Readable for STATUSA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSA to value 0"]
impl crate::Resettable for STATUSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
