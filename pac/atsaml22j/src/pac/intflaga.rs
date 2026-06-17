#[doc = "Register `INTFLAGA` reader"]
pub struct R(crate::R<INTFLAGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGA` writer"]
pub struct W(crate::W<INTFLAGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGA_SPEC>;
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
impl From<crate::W<INTFLAGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAC_` reader - PAC"]
pub type PAC__R = crate::BitReader<bool>;
#[doc = "Field `PAC_` writer - PAC"]
pub type PAC__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, O>;
#[doc = "Field `PM_` reader - PM"]
pub type PM__R = crate::BitReader<bool>;
#[doc = "Field `PM_` writer - PM"]
pub type PM__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, O>;
#[doc = "Field `MCLK_` reader - MCLK"]
pub type MCLK__R = crate::BitReader<bool>;
#[doc = "Field `MCLK_` writer - MCLK"]
pub type MCLK__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, O>;
#[doc = "Field `RSTC_` reader - RSTC"]
pub type RSTC__R = crate::BitReader<bool>;
#[doc = "Field `RSTC_` writer - RSTC"]
pub type RSTC__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, O>;
#[doc = "Field `OSCCTRL_` reader - OSCCTRL"]
pub type OSCCTRL__R = crate::BitReader<bool>;
#[doc = "Field `OSCCTRL_` writer - OSCCTRL"]
pub type OSCCTRL__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, O>;
#[doc = "Field `OSC32KCTRL_` reader - OSC32KCTRL"]
pub type OSC32KCTRL__R = crate::BitReader<bool>;
#[doc = "Field `OSC32KCTRL_` writer - OSC32KCTRL"]
pub type OSC32KCTRL__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, O>;
#[doc = "Field `SUPC_` reader - SUPC"]
pub type SUPC__R = crate::BitReader<bool>;
#[doc = "Field `SUPC_` writer - SUPC"]
pub type SUPC__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, O>;
#[doc = "Field `GCLK_` reader - GCLK"]
pub type GCLK__R = crate::BitReader<bool>;
#[doc = "Field `GCLK_` writer - GCLK"]
pub type GCLK__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, O>;
#[doc = "Field `WDT_` reader - WDT"]
pub type WDT__R = crate::BitReader<bool>;
#[doc = "Field `WDT_` writer - WDT"]
pub type WDT__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, O>;
#[doc = "Field `RTC_` reader - RTC"]
pub type RTC__R = crate::BitReader<bool>;
#[doc = "Field `RTC_` writer - RTC"]
pub type RTC__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, O>;
#[doc = "Field `EIC_` reader - EIC"]
pub type EIC__R = crate::BitReader<bool>;
#[doc = "Field `EIC_` writer - EIC"]
pub type EIC__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, O>;
#[doc = "Field `FREQM_` reader - FREQM"]
pub type FREQM__R = crate::BitReader<bool>;
#[doc = "Field `FREQM_` writer - FREQM"]
pub type FREQM__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGA_SPEC, bool, O>;
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
}
impl W {
    #[doc = "Bit 0 - PAC"]
    #[inline(always)]
    #[must_use]
    pub fn pac_(&mut self) -> PAC__W<0> {
        PAC__W::new(self)
    }
    #[doc = "Bit 1 - PM"]
    #[inline(always)]
    #[must_use]
    pub fn pm_(&mut self) -> PM__W<1> {
        PM__W::new(self)
    }
    #[doc = "Bit 2 - MCLK"]
    #[inline(always)]
    #[must_use]
    pub fn mclk_(&mut self) -> MCLK__W<2> {
        MCLK__W::new(self)
    }
    #[doc = "Bit 3 - RSTC"]
    #[inline(always)]
    #[must_use]
    pub fn rstc_(&mut self) -> RSTC__W<3> {
        RSTC__W::new(self)
    }
    #[doc = "Bit 4 - OSCCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn oscctrl_(&mut self) -> OSCCTRL__W<4> {
        OSCCTRL__W::new(self)
    }
    #[doc = "Bit 5 - OSC32KCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn osc32kctrl_(&mut self) -> OSC32KCTRL__W<5> {
        OSC32KCTRL__W::new(self)
    }
    #[doc = "Bit 6 - SUPC"]
    #[inline(always)]
    #[must_use]
    pub fn supc_(&mut self) -> SUPC__W<6> {
        SUPC__W::new(self)
    }
    #[doc = "Bit 7 - GCLK"]
    #[inline(always)]
    #[must_use]
    pub fn gclk_(&mut self) -> GCLK__W<7> {
        GCLK__W::new(self)
    }
    #[doc = "Bit 8 - WDT"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_(&mut self) -> WDT__W<8> {
        WDT__W::new(self)
    }
    #[doc = "Bit 9 - RTC"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_(&mut self) -> RTC__W<9> {
        RTC__W::new(self)
    }
    #[doc = "Bit 10 - EIC"]
    #[inline(always)]
    #[must_use]
    pub fn eic_(&mut self) -> EIC__W<10> {
        EIC__W::new(self)
    }
    #[doc = "Bit 11 - FREQM"]
    #[inline(always)]
    #[must_use]
    pub fn freqm_(&mut self) -> FREQM__W<11> {
        FREQM__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral interrupt flag status - Bridge A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflaga](index.html) module"]
pub struct INTFLAGA_SPEC;
impl crate::RegisterSpec for INTFLAGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflaga::R](R) reader structure"]
impl crate::Readable for INTFLAGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflaga::W](W) writer structure"]
impl crate::Writable for INTFLAGA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGA to value 0"]
impl crate::Resettable for INTFLAGA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
