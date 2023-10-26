#[doc = "Register `INTFLAGA` reader"]
pub type R = crate::R<INTFLAGA_SPEC>;
#[doc = "Register `INTFLAGA` writer"]
pub type W = crate::W<INTFLAGA_SPEC>;
#[doc = "Field `PAC_` reader - PAC"]
pub type PAC__R = crate::BitReader;
#[doc = "Field `PAC_` writer - PAC"]
pub type PAC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PM_` reader - PM"]
pub type PM__R = crate::BitReader;
#[doc = "Field `PM_` writer - PM"]
pub type PM__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MCLK_` reader - MCLK"]
pub type MCLK__R = crate::BitReader;
#[doc = "Field `MCLK_` writer - MCLK"]
pub type MCLK__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTC_` reader - RSTC"]
pub type RSTC__R = crate::BitReader;
#[doc = "Field `RSTC_` writer - RSTC"]
pub type RSTC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCCTRL_` reader - OSCCTRL"]
pub type OSCCTRL__R = crate::BitReader;
#[doc = "Field `OSCCTRL_` writer - OSCCTRL"]
pub type OSCCTRL__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSC32KCTRL_` reader - OSC32KCTRL"]
pub type OSC32KCTRL__R = crate::BitReader;
#[doc = "Field `OSC32KCTRL_` writer - OSC32KCTRL"]
pub type OSC32KCTRL__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SUPC_` reader - SUPC"]
pub type SUPC__R = crate::BitReader;
#[doc = "Field `SUPC_` writer - SUPC"]
pub type SUPC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GCLK_` reader - GCLK"]
pub type GCLK__R = crate::BitReader;
#[doc = "Field `GCLK_` writer - GCLK"]
pub type GCLK__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDT_` reader - WDT"]
pub type WDT__R = crate::BitReader;
#[doc = "Field `WDT_` writer - WDT"]
pub type WDT__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTC_` reader - RTC"]
pub type RTC__R = crate::BitReader;
#[doc = "Field `RTC_` writer - RTC"]
pub type RTC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EIC_` reader - EIC"]
pub type EIC__R = crate::BitReader;
#[doc = "Field `EIC_` writer - EIC"]
pub type EIC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FREQM_` reader - FREQM"]
pub type FREQM__R = crate::BitReader;
#[doc = "Field `FREQM_` writer - FREQM"]
pub type FREQM__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SERCOM0_` reader - SERCOM0"]
pub type SERCOM0__R = crate::BitReader;
#[doc = "Field `SERCOM0_` writer - SERCOM0"]
pub type SERCOM0__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SERCOM1_` reader - SERCOM1"]
pub type SERCOM1__R = crate::BitReader;
#[doc = "Field `SERCOM1_` writer - SERCOM1"]
pub type SERCOM1__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC0_` reader - TC0"]
pub type TC0__R = crate::BitReader;
#[doc = "Field `TC0_` writer - TC0"]
pub type TC0__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC1_` reader - TC1"]
pub type TC1__R = crate::BitReader;
#[doc = "Field `TC1_` writer - TC1"]
pub type TC1__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PAC"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PM"]
    #[inline(always)]
    pub fn pm_(&self) -> PM__R {
        PM__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCLK"]
    #[inline(always)]
    pub fn mclk_(&self) -> MCLK__R {
        MCLK__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSTC"]
    #[inline(always)]
    pub fn rstc_(&self) -> RSTC__R {
        RSTC__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OSCCTRL"]
    #[inline(always)]
    pub fn oscctrl_(&self) -> OSCCTRL__R {
        OSCCTRL__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OSC32KCTRL"]
    #[inline(always)]
    pub fn osc32kctrl_(&self) -> OSC32KCTRL__R {
        OSC32KCTRL__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SUPC"]
    #[inline(always)]
    pub fn supc_(&self) -> SUPC__R {
        SUPC__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GCLK"]
    #[inline(always)]
    pub fn gclk_(&self) -> GCLK__R {
        GCLK__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WDT"]
    #[inline(always)]
    pub fn wdt_(&self) -> WDT__R {
        WDT__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RTC"]
    #[inline(always)]
    pub fn rtc_(&self) -> RTC__R {
        RTC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EIC"]
    #[inline(always)]
    pub fn eic_(&self) -> EIC__R {
        EIC__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FREQM"]
    #[inline(always)]
    pub fn freqm_(&self) -> FREQM__R {
        FREQM__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SERCOM0"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SERCOM1"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TC0"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TC1"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC"]
    #[inline(always)]
    #[must_use]
    pub fn pac_(&mut self) -> PAC__W<INTFLAGA_SPEC, 0> {
        PAC__W::new(self)
    }
    #[doc = "Bit 1 - PM"]
    #[inline(always)]
    #[must_use]
    pub fn pm_(&mut self) -> PM__W<INTFLAGA_SPEC, 1> {
        PM__W::new(self)
    }
    #[doc = "Bit 2 - MCLK"]
    #[inline(always)]
    #[must_use]
    pub fn mclk_(&mut self) -> MCLK__W<INTFLAGA_SPEC, 2> {
        MCLK__W::new(self)
    }
    #[doc = "Bit 3 - RSTC"]
    #[inline(always)]
    #[must_use]
    pub fn rstc_(&mut self) -> RSTC__W<INTFLAGA_SPEC, 3> {
        RSTC__W::new(self)
    }
    #[doc = "Bit 4 - OSCCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn oscctrl_(&mut self) -> OSCCTRL__W<INTFLAGA_SPEC, 4> {
        OSCCTRL__W::new(self)
    }
    #[doc = "Bit 5 - OSC32KCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn osc32kctrl_(&mut self) -> OSC32KCTRL__W<INTFLAGA_SPEC, 5> {
        OSC32KCTRL__W::new(self)
    }
    #[doc = "Bit 6 - SUPC"]
    #[inline(always)]
    #[must_use]
    pub fn supc_(&mut self) -> SUPC__W<INTFLAGA_SPEC, 6> {
        SUPC__W::new(self)
    }
    #[doc = "Bit 7 - GCLK"]
    #[inline(always)]
    #[must_use]
    pub fn gclk_(&mut self) -> GCLK__W<INTFLAGA_SPEC, 7> {
        GCLK__W::new(self)
    }
    #[doc = "Bit 8 - WDT"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_(&mut self) -> WDT__W<INTFLAGA_SPEC, 8> {
        WDT__W::new(self)
    }
    #[doc = "Bit 9 - RTC"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_(&mut self) -> RTC__W<INTFLAGA_SPEC, 9> {
        RTC__W::new(self)
    }
    #[doc = "Bit 10 - EIC"]
    #[inline(always)]
    #[must_use]
    pub fn eic_(&mut self) -> EIC__W<INTFLAGA_SPEC, 10> {
        EIC__W::new(self)
    }
    #[doc = "Bit 11 - FREQM"]
    #[inline(always)]
    #[must_use]
    pub fn freqm_(&mut self) -> FREQM__W<INTFLAGA_SPEC, 11> {
        FREQM__W::new(self)
    }
    #[doc = "Bit 12 - SERCOM0"]
    #[inline(always)]
    #[must_use]
    pub fn sercom0_(&mut self) -> SERCOM0__W<INTFLAGA_SPEC, 12> {
        SERCOM0__W::new(self)
    }
    #[doc = "Bit 13 - SERCOM1"]
    #[inline(always)]
    #[must_use]
    pub fn sercom1_(&mut self) -> SERCOM1__W<INTFLAGA_SPEC, 13> {
        SERCOM1__W::new(self)
    }
    #[doc = "Bit 14 - TC0"]
    #[inline(always)]
    #[must_use]
    pub fn tc0_(&mut self) -> TC0__W<INTFLAGA_SPEC, 14> {
        TC0__W::new(self)
    }
    #[doc = "Bit 15 - TC1"]
    #[inline(always)]
    #[must_use]
    pub fn tc1_(&mut self) -> TC1__W<INTFLAGA_SPEC, 15> {
        TC1__W::new(self)
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
#[doc = "Peripheral interrupt flag status - Bridge A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflaga::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflaga::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAGA_SPEC;
impl crate::RegisterSpec for INTFLAGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflaga::R`](R) reader structure"]
impl crate::Readable for INTFLAGA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intflaga::W`](W) writer structure"]
impl crate::Writable for INTFLAGA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGA to value 0"]
impl crate::Resettable for INTFLAGA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
