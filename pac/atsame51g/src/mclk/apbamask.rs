#[doc = "Register `APBAMASK` reader"]
pub type R = crate::R<ApbamaskSpec>;
#[doc = "Register `APBAMASK` writer"]
pub type W = crate::W<ApbamaskSpec>;
#[doc = "Field `PAC_` reader - PAC APB Clock Enable"]
pub type Pac_R = crate::BitReader;
#[doc = "Field `PAC_` writer - PAC APB Clock Enable"]
pub type Pac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PM_` reader - PM APB Clock Enable"]
pub type Pm_R = crate::BitReader;
#[doc = "Field `PM_` writer - PM APB Clock Enable"]
pub type Pm_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCLK_` reader - MCLK APB Clock Enable"]
pub type Mclk_R = crate::BitReader;
#[doc = "Field `MCLK_` writer - MCLK APB Clock Enable"]
pub type Mclk_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTC_` reader - RSTC APB Clock Enable"]
pub type Rstc_R = crate::BitReader;
#[doc = "Field `RSTC_` writer - RSTC APB Clock Enable"]
pub type Rstc_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCCTRL_` reader - OSCCTRL APB Clock Enable"]
pub type Oscctrl_R = crate::BitReader;
#[doc = "Field `OSCCTRL_` writer - OSCCTRL APB Clock Enable"]
pub type Oscctrl_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSC32KCTRL_` reader - OSC32KCTRL APB Clock Enable"]
pub type Osc32kctrl_R = crate::BitReader;
#[doc = "Field `OSC32KCTRL_` writer - OSC32KCTRL APB Clock Enable"]
pub type Osc32kctrl_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPC_` reader - SUPC APB Clock Enable"]
pub type Supc_R = crate::BitReader;
#[doc = "Field `SUPC_` writer - SUPC APB Clock Enable"]
pub type Supc_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCLK_` reader - GCLK APB Clock Enable"]
pub type Gclk_R = crate::BitReader;
#[doc = "Field `GCLK_` writer - GCLK APB Clock Enable"]
pub type Gclk_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_` reader - WDT APB Clock Enable"]
pub type Wdt_R = crate::BitReader;
#[doc = "Field `WDT_` writer - WDT APB Clock Enable"]
pub type Wdt_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_` reader - RTC APB Clock Enable"]
pub type Rtc_R = crate::BitReader;
#[doc = "Field `RTC_` writer - RTC APB Clock Enable"]
pub type Rtc_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIC_` reader - EIC APB Clock Enable"]
pub type Eic_R = crate::BitReader;
#[doc = "Field `EIC_` writer - EIC APB Clock Enable"]
pub type Eic_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREQM_` reader - FREQM APB Clock Enable"]
pub type Freqm_R = crate::BitReader;
#[doc = "Field `FREQM_` writer - FREQM APB Clock Enable"]
pub type Freqm_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM0_` reader - SERCOM0 APB Clock Enable"]
pub type Sercom0_R = crate::BitReader;
#[doc = "Field `SERCOM0_` writer - SERCOM0 APB Clock Enable"]
pub type Sercom0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM1_` reader - SERCOM1 APB Clock Enable"]
pub type Sercom1_R = crate::BitReader;
#[doc = "Field `SERCOM1_` writer - SERCOM1 APB Clock Enable"]
pub type Sercom1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC0_` reader - TC0 APB Clock Enable"]
pub type Tc0_R = crate::BitReader;
#[doc = "Field `TC0_` writer - TC0 APB Clock Enable"]
pub type Tc0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1_` reader - TC1 APB Clock Enable"]
pub type Tc1_R = crate::BitReader;
#[doc = "Field `TC1_` writer - TC1 APB Clock Enable"]
pub type Tc1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PAC APB Clock Enable"]
    #[inline(always)]
    pub fn pac_(&self) -> Pac_R {
        Pac_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PM APB Clock Enable"]
    #[inline(always)]
    pub fn pm_(&self) -> Pm_R {
        Pm_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCLK APB Clock Enable"]
    #[inline(always)]
    pub fn mclk_(&self) -> Mclk_R {
        Mclk_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSTC APB Clock Enable"]
    #[inline(always)]
    pub fn rstc_(&self) -> Rstc_R {
        Rstc_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OSCCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn oscctrl_(&self) -> Oscctrl_R {
        Oscctrl_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OSC32KCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn osc32kctrl_(&self) -> Osc32kctrl_R {
        Osc32kctrl_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SUPC APB Clock Enable"]
    #[inline(always)]
    pub fn supc_(&self) -> Supc_R {
        Supc_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GCLK APB Clock Enable"]
    #[inline(always)]
    pub fn gclk_(&self) -> Gclk_R {
        Gclk_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WDT APB Clock Enable"]
    #[inline(always)]
    pub fn wdt_(&self) -> Wdt_R {
        Wdt_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RTC APB Clock Enable"]
    #[inline(always)]
    pub fn rtc_(&self) -> Rtc_R {
        Rtc_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EIC APB Clock Enable"]
    #[inline(always)]
    pub fn eic_(&self) -> Eic_R {
        Eic_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FREQM APB Clock Enable"]
    #[inline(always)]
    pub fn freqm_(&self) -> Freqm_R {
        Freqm_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> Sercom0_R {
        Sercom0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> Sercom1_R {
        Sercom1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> Tc0_R {
        Tc0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> Tc1_R {
        Tc1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC APB Clock Enable"]
    #[inline(always)]
    pub fn pac_(&mut self) -> Pac_W<ApbamaskSpec> {
        Pac_W::new(self, 0)
    }
    #[doc = "Bit 1 - PM APB Clock Enable"]
    #[inline(always)]
    pub fn pm_(&mut self) -> Pm_W<ApbamaskSpec> {
        Pm_W::new(self, 1)
    }
    #[doc = "Bit 2 - MCLK APB Clock Enable"]
    #[inline(always)]
    pub fn mclk_(&mut self) -> Mclk_W<ApbamaskSpec> {
        Mclk_W::new(self, 2)
    }
    #[doc = "Bit 3 - RSTC APB Clock Enable"]
    #[inline(always)]
    pub fn rstc_(&mut self) -> Rstc_W<ApbamaskSpec> {
        Rstc_W::new(self, 3)
    }
    #[doc = "Bit 4 - OSCCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn oscctrl_(&mut self) -> Oscctrl_W<ApbamaskSpec> {
        Oscctrl_W::new(self, 4)
    }
    #[doc = "Bit 5 - OSC32KCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn osc32kctrl_(&mut self) -> Osc32kctrl_W<ApbamaskSpec> {
        Osc32kctrl_W::new(self, 5)
    }
    #[doc = "Bit 6 - SUPC APB Clock Enable"]
    #[inline(always)]
    pub fn supc_(&mut self) -> Supc_W<ApbamaskSpec> {
        Supc_W::new(self, 6)
    }
    #[doc = "Bit 7 - GCLK APB Clock Enable"]
    #[inline(always)]
    pub fn gclk_(&mut self) -> Gclk_W<ApbamaskSpec> {
        Gclk_W::new(self, 7)
    }
    #[doc = "Bit 8 - WDT APB Clock Enable"]
    #[inline(always)]
    pub fn wdt_(&mut self) -> Wdt_W<ApbamaskSpec> {
        Wdt_W::new(self, 8)
    }
    #[doc = "Bit 9 - RTC APB Clock Enable"]
    #[inline(always)]
    pub fn rtc_(&mut self) -> Rtc_W<ApbamaskSpec> {
        Rtc_W::new(self, 9)
    }
    #[doc = "Bit 10 - EIC APB Clock Enable"]
    #[inline(always)]
    pub fn eic_(&mut self) -> Eic_W<ApbamaskSpec> {
        Eic_W::new(self, 10)
    }
    #[doc = "Bit 11 - FREQM APB Clock Enable"]
    #[inline(always)]
    pub fn freqm_(&mut self) -> Freqm_W<ApbamaskSpec> {
        Freqm_W::new(self, 11)
    }
    #[doc = "Bit 12 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom0_(&mut self) -> Sercom0_W<ApbamaskSpec> {
        Sercom0_W::new(self, 12)
    }
    #[doc = "Bit 13 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom1_(&mut self) -> Sercom1_W<ApbamaskSpec> {
        Sercom1_W::new(self, 13)
    }
    #[doc = "Bit 14 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&mut self) -> Tc0_W<ApbamaskSpec> {
        Tc0_W::new(self, 14)
    }
    #[doc = "Bit 15 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&mut self) -> Tc1_W<ApbamaskSpec> {
        Tc1_W::new(self, 15)
    }
}
#[doc = "APBA Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbamask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbamask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbamaskSpec;
impl crate::RegisterSpec for ApbamaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbamask::R`](R) reader structure"]
impl crate::Readable for ApbamaskSpec {}
#[doc = "`write(|w| ..)` method takes [`apbamask::W`](W) writer structure"]
impl crate::Writable for ApbamaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBAMASK to value 0x07ff"]
impl crate::Resettable for ApbamaskSpec {
    const RESET_VALUE: u32 = 0x07ff;
}
