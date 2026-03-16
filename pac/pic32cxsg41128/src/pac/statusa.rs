#[doc = "Register `STATUSA` reader"]
pub type R = crate::R<StatusaSpec>;
#[doc = "Field `PAC_` reader - PAC APB Protect Enable"]
pub type Pac_R = crate::BitReader;
#[doc = "Field `PM_` reader - PM APB Protect Enable"]
pub type Pm_R = crate::BitReader;
#[doc = "Field `MCLK_` reader - MCLK APB Protect Enable"]
pub type Mclk_R = crate::BitReader;
#[doc = "Field `RSTC_` reader - RSTC APB Protect Enable"]
pub type Rstc_R = crate::BitReader;
#[doc = "Field `OSCCTRL_` reader - OSCCTRL APB Protect Enable"]
pub type Oscctrl_R = crate::BitReader;
#[doc = "Field `OSC32KCTRL_` reader - OSC32KCTRL APB Protect Enable"]
pub type Osc32kctrl_R = crate::BitReader;
#[doc = "Field `SUPC_` reader - SUPC APB Protect Enable"]
pub type Supc_R = crate::BitReader;
#[doc = "Field `GCLK_` reader - GCLK APB Protect Enable"]
pub type Gclk_R = crate::BitReader;
#[doc = "Field `WDT_` reader - WDT APB Protect Enable"]
pub type Wdt_R = crate::BitReader;
#[doc = "Field `RTC_` reader - RTC APB Protect Enable"]
pub type Rtc_R = crate::BitReader;
#[doc = "Field `EIC_` reader - EIC APB Protect Enable"]
pub type Eic_R = crate::BitReader;
#[doc = "Field `FREQM_` reader - FREQM APB Protect Enable"]
pub type Freqm_R = crate::BitReader;
#[doc = "Field `SERCOM0_` reader - SERCOM0 APB Protect Enable"]
pub type Sercom0_R = crate::BitReader;
#[doc = "Field `SERCOM1_` reader - SERCOM1 APB Protect Enable"]
pub type Sercom1_R = crate::BitReader;
#[doc = "Field `TC0_` reader - TC0 APB Protect Enable"]
pub type Tc0_R = crate::BitReader;
#[doc = "Field `TC1_` reader - TC1 APB Protect Enable"]
pub type Tc1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PAC APB Protect Enable"]
    #[inline(always)]
    pub fn pac_(&self) -> Pac_R {
        Pac_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PM APB Protect Enable"]
    #[inline(always)]
    pub fn pm_(&self) -> Pm_R {
        Pm_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCLK APB Protect Enable"]
    #[inline(always)]
    pub fn mclk_(&self) -> Mclk_R {
        Mclk_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSTC APB Protect Enable"]
    #[inline(always)]
    pub fn rstc_(&self) -> Rstc_R {
        Rstc_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OSCCTRL APB Protect Enable"]
    #[inline(always)]
    pub fn oscctrl_(&self) -> Oscctrl_R {
        Oscctrl_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OSC32KCTRL APB Protect Enable"]
    #[inline(always)]
    pub fn osc32kctrl_(&self) -> Osc32kctrl_R {
        Osc32kctrl_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SUPC APB Protect Enable"]
    #[inline(always)]
    pub fn supc_(&self) -> Supc_R {
        Supc_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GCLK APB Protect Enable"]
    #[inline(always)]
    pub fn gclk_(&self) -> Gclk_R {
        Gclk_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WDT APB Protect Enable"]
    #[inline(always)]
    pub fn wdt_(&self) -> Wdt_R {
        Wdt_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RTC APB Protect Enable"]
    #[inline(always)]
    pub fn rtc_(&self) -> Rtc_R {
        Rtc_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EIC APB Protect Enable"]
    #[inline(always)]
    pub fn eic_(&self) -> Eic_R {
        Eic_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FREQM APB Protect Enable"]
    #[inline(always)]
    pub fn freqm_(&self) -> Freqm_R {
        Freqm_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SERCOM0 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> Sercom0_R {
        Sercom0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SERCOM1 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> Sercom1_R {
        Sercom1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> Tc0_R {
        Tc0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> Tc1_R {
        Tc1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge A\n\nYou can [`read`](crate::Reg::read) this register and get [`statusa::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusaSpec;
impl crate::RegisterSpec for StatusaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusa::R`](R) reader structure"]
impl crate::Readable for StatusaSpec {}
#[doc = "`reset()` method sets STATUSA to value 0x0001_0000"]
impl crate::Resettable for StatusaSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
