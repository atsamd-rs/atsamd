#[doc = "Register `APBAMASK` reader"]
pub type R = crate::R<ApbamaskSpec>;
#[doc = "Register `APBAMASK` writer"]
pub type W = crate::W<ApbamaskSpec>;
#[doc = "Field `PAC0_` reader - PAC0 APB Clock Enable"]
pub type Pac0_R = crate::BitReader;
#[doc = "Field `PAC0_` writer - PAC0 APB Clock Enable"]
pub type Pac0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PM_` reader - PM APB Clock Enable"]
pub type Pm_R = crate::BitReader;
#[doc = "Field `PM_` writer - PM APB Clock Enable"]
pub type Pm_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCTRL_` reader - SYSCTRL APB Clock Enable"]
pub type Sysctrl_R = crate::BitReader;
#[doc = "Field `SYSCTRL_` writer - SYSCTRL APB Clock Enable"]
pub type Sysctrl_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl R {
    #[doc = "Bit 0 - PAC0 APB Clock Enable"]
    #[inline(always)]
    pub fn pac0_(&self) -> Pac0_R {
        Pac0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PM APB Clock Enable"]
    #[inline(always)]
    pub fn pm_(&self) -> Pm_R {
        Pm_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SYSCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn sysctrl_(&self) -> Sysctrl_R {
        Sysctrl_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GCLK APB Clock Enable"]
    #[inline(always)]
    pub fn gclk_(&self) -> Gclk_R {
        Gclk_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WDT APB Clock Enable"]
    #[inline(always)]
    pub fn wdt_(&self) -> Wdt_R {
        Wdt_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC APB Clock Enable"]
    #[inline(always)]
    pub fn rtc_(&self) -> Rtc_R {
        Rtc_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EIC APB Clock Enable"]
    #[inline(always)]
    pub fn eic_(&self) -> Eic_R {
        Eic_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC0 APB Clock Enable"]
    #[inline(always)]
    pub fn pac0_(&mut self) -> Pac0_W<ApbamaskSpec> {
        Pac0_W::new(self, 0)
    }
    #[doc = "Bit 1 - PM APB Clock Enable"]
    #[inline(always)]
    pub fn pm_(&mut self) -> Pm_W<ApbamaskSpec> {
        Pm_W::new(self, 1)
    }
    #[doc = "Bit 2 - SYSCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn sysctrl_(&mut self) -> Sysctrl_W<ApbamaskSpec> {
        Sysctrl_W::new(self, 2)
    }
    #[doc = "Bit 3 - GCLK APB Clock Enable"]
    #[inline(always)]
    pub fn gclk_(&mut self) -> Gclk_W<ApbamaskSpec> {
        Gclk_W::new(self, 3)
    }
    #[doc = "Bit 4 - WDT APB Clock Enable"]
    #[inline(always)]
    pub fn wdt_(&mut self) -> Wdt_W<ApbamaskSpec> {
        Wdt_W::new(self, 4)
    }
    #[doc = "Bit 5 - RTC APB Clock Enable"]
    #[inline(always)]
    pub fn rtc_(&mut self) -> Rtc_W<ApbamaskSpec> {
        Rtc_W::new(self, 5)
    }
    #[doc = "Bit 6 - EIC APB Clock Enable"]
    #[inline(always)]
    pub fn eic_(&mut self) -> Eic_W<ApbamaskSpec> {
        Eic_W::new(self, 6)
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
#[doc = "`reset()` method sets APBAMASK to value 0x7f"]
impl crate::Resettable for ApbamaskSpec {
    const RESET_VALUE: u32 = 0x7f;
}
