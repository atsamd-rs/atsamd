#[doc = "Register `APBAMASK` reader"]
pub type R = crate::R<APBAMASK_SPEC>;
#[doc = "Register `APBAMASK` writer"]
pub type W = crate::W<APBAMASK_SPEC>;
#[doc = "Field `PAC0_` reader - PAC0 APB Clock Enable"]
pub type PAC0__R = crate::BitReader;
#[doc = "Field `PAC0_` writer - PAC0 APB Clock Enable"]
pub type PAC0__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PM_` reader - PM APB Clock Enable"]
pub type PM__R = crate::BitReader;
#[doc = "Field `PM_` writer - PM APB Clock Enable"]
pub type PM__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTRL_` reader - SYSCTRL APB Clock Enable"]
pub type SYSCTRL__R = crate::BitReader;
#[doc = "Field `SYSCTRL_` writer - SYSCTRL APB Clock Enable"]
pub type SYSCTRL__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GCLK_` reader - GCLK APB Clock Enable"]
pub type GCLK__R = crate::BitReader;
#[doc = "Field `GCLK_` writer - GCLK APB Clock Enable"]
pub type GCLK__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDT_` reader - WDT APB Clock Enable"]
pub type WDT__R = crate::BitReader;
#[doc = "Field `WDT_` writer - WDT APB Clock Enable"]
pub type WDT__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTC_` reader - RTC APB Clock Enable"]
pub type RTC__R = crate::BitReader;
#[doc = "Field `RTC_` writer - RTC APB Clock Enable"]
pub type RTC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EIC_` reader - EIC APB Clock Enable"]
pub type EIC__R = crate::BitReader;
#[doc = "Field `EIC_` writer - EIC APB Clock Enable"]
pub type EIC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PAC0 APB Clock Enable"]
    #[inline(always)]
    pub fn pac0_(&self) -> PAC0__R {
        PAC0__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PM APB Clock Enable"]
    #[inline(always)]
    pub fn pm_(&self) -> PM__R {
        PM__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SYSCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn sysctrl_(&self) -> SYSCTRL__R {
        SYSCTRL__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GCLK APB Clock Enable"]
    #[inline(always)]
    pub fn gclk_(&self) -> GCLK__R {
        GCLK__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WDT APB Clock Enable"]
    #[inline(always)]
    pub fn wdt_(&self) -> WDT__R {
        WDT__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC APB Clock Enable"]
    #[inline(always)]
    pub fn rtc_(&self) -> RTC__R {
        RTC__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EIC APB Clock Enable"]
    #[inline(always)]
    pub fn eic_(&self) -> EIC__R {
        EIC__R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pac0_(&mut self) -> PAC0__W<APBAMASK_SPEC, 0> {
        PAC0__W::new(self)
    }
    #[doc = "Bit 1 - PM APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pm_(&mut self) -> PM__W<APBAMASK_SPEC, 1> {
        PM__W::new(self)
    }
    #[doc = "Bit 2 - SYSCTRL APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sysctrl_(&mut self) -> SYSCTRL__W<APBAMASK_SPEC, 2> {
        SYSCTRL__W::new(self)
    }
    #[doc = "Bit 3 - GCLK APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gclk_(&mut self) -> GCLK__W<APBAMASK_SPEC, 3> {
        GCLK__W::new(self)
    }
    #[doc = "Bit 4 - WDT APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_(&mut self) -> WDT__W<APBAMASK_SPEC, 4> {
        WDT__W::new(self)
    }
    #[doc = "Bit 5 - RTC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_(&mut self) -> RTC__W<APBAMASK_SPEC, 5> {
        RTC__W::new(self)
    }
    #[doc = "Bit 6 - EIC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eic_(&mut self) -> EIC__W<APBAMASK_SPEC, 6> {
        EIC__W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "APBA Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbamask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbamask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBAMASK_SPEC;
impl crate::RegisterSpec for APBAMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbamask::R`](R) reader structure"]
impl crate::Readable for APBAMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbamask::W`](W) writer structure"]
impl crate::Writable for APBAMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBAMASK to value 0x7f"]
impl crate::Resettable for APBAMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
