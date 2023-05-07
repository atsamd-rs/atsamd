#[doc = "Register `APBAMASK` reader"]
pub struct R(crate::R<APBAMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBAMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBAMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBAMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBAMASK` writer"]
pub struct W(crate::W<APBAMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBAMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<APBAMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBAMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAC0_` reader - PAC0 APB Clock Enable"]
pub type PAC0__R = crate::BitReader<bool>;
#[doc = "Field `PAC0_` writer - PAC0 APB Clock Enable"]
pub type PAC0__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, O>;
#[doc = "Field `PM_` reader - PM APB Clock Enable"]
pub type PM__R = crate::BitReader<bool>;
#[doc = "Field `PM_` writer - PM APB Clock Enable"]
pub type PM__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, O>;
#[doc = "Field `SYSCTRL_` reader - SYSCTRL APB Clock Enable"]
pub type SYSCTRL__R = crate::BitReader<bool>;
#[doc = "Field `SYSCTRL_` writer - SYSCTRL APB Clock Enable"]
pub type SYSCTRL__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, O>;
#[doc = "Field `GCLK_` reader - GCLK APB Clock Enable"]
pub type GCLK__R = crate::BitReader<bool>;
#[doc = "Field `GCLK_` writer - GCLK APB Clock Enable"]
pub type GCLK__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, O>;
#[doc = "Field `WDT_` reader - WDT APB Clock Enable"]
pub type WDT__R = crate::BitReader<bool>;
#[doc = "Field `WDT_` writer - WDT APB Clock Enable"]
pub type WDT__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, O>;
#[doc = "Field `RTC_` reader - RTC APB Clock Enable"]
pub type RTC__R = crate::BitReader<bool>;
#[doc = "Field `RTC_` writer - RTC APB Clock Enable"]
pub type RTC__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, O>;
#[doc = "Field `EIC_` reader - EIC APB Clock Enable"]
pub type EIC__R = crate::BitReader<bool>;
#[doc = "Field `EIC_` writer - EIC APB Clock Enable"]
pub type EIC__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBAMASK_SPEC, bool, O>;
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
    pub fn pac0_(&mut self) -> PAC0__W<0> {
        PAC0__W::new(self)
    }
    #[doc = "Bit 1 - PM APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pm_(&mut self) -> PM__W<1> {
        PM__W::new(self)
    }
    #[doc = "Bit 2 - SYSCTRL APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sysctrl_(&mut self) -> SYSCTRL__W<2> {
        SYSCTRL__W::new(self)
    }
    #[doc = "Bit 3 - GCLK APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gclk_(&mut self) -> GCLK__W<3> {
        GCLK__W::new(self)
    }
    #[doc = "Bit 4 - WDT APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_(&mut self) -> WDT__W<4> {
        WDT__W::new(self)
    }
    #[doc = "Bit 5 - RTC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_(&mut self) -> RTC__W<5> {
        RTC__W::new(self)
    }
    #[doc = "Bit 6 - EIC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eic_(&mut self) -> EIC__W<6> {
        EIC__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBA Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbamask](index.html) module"]
pub struct APBAMASK_SPEC;
impl crate::RegisterSpec for APBAMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbamask::R](R) reader structure"]
impl crate::Readable for APBAMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbamask::W](W) writer structure"]
impl crate::Writable for APBAMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBAMASK to value 0x7f"]
impl crate::Resettable for APBAMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
