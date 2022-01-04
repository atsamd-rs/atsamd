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
pub struct PAC__R(crate::FieldReader<bool, bool>);
impl PAC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM_` reader - PM APB Protect Enable"]
pub struct PM__R(crate::FieldReader<bool, bool>);
impl PM__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PM__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PM__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCLK_` reader - MCLK APB Protect Enable"]
pub struct MCLK__R(crate::FieldReader<bool, bool>);
impl MCLK__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCLK__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCLK__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTC_` reader - RSTC APB Protect Enable"]
pub struct RSTC__R(crate::FieldReader<bool, bool>);
impl RSTC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSTC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCCTRL_` reader - OSCCTRL APB Protect Enable"]
pub struct OSCCTRL__R(crate::FieldReader<bool, bool>);
impl OSCCTRL__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSCCTRL__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSCCTRL__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC32KCTRL_` reader - OSC32KCTRL APB Protect Enable"]
pub struct OSC32KCTRL__R(crate::FieldReader<bool, bool>);
impl OSC32KCTRL__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC32KCTRL__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC32KCTRL__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUPC_` reader - SUPC APB Protect Enable"]
pub struct SUPC__R(crate::FieldReader<bool, bool>);
impl SUPC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUPC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUPC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCLK_` reader - GCLK APB Protect Enable"]
pub struct GCLK__R(crate::FieldReader<bool, bool>);
impl GCLK__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GCLK__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCLK__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_` reader - WDT APB Protect Enable"]
pub struct WDT__R(crate::FieldReader<bool, bool>);
impl WDT__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_` reader - RTC APB Protect Enable"]
pub struct RTC__R(crate::FieldReader<bool, bool>);
impl RTC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIC_` reader - EIC APB Protect Enable"]
pub struct EIC__R(crate::FieldReader<bool, bool>);
impl EIC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EIC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQM_` reader - FREQM APB Protect Enable"]
pub struct FREQM__R(crate::FieldReader<bool, bool>);
impl FREQM__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FREQM__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQM__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM0_` reader - SERCOM0 APB Protect Enable"]
pub struct SERCOM0__R(crate::FieldReader<bool, bool>);
impl SERCOM0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM1_` reader - SERCOM1 APB Protect Enable"]
pub struct SERCOM1__R(crate::FieldReader<bool, bool>);
impl SERCOM1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC0_` reader - TC0 APB Protect Enable"]
pub struct TC0__R(crate::FieldReader<bool, bool>);
impl TC0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC1_` reader - TC1 APB Protect Enable"]
pub struct TC1__R(crate::FieldReader<bool, bool>);
impl TC1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Peripheral write protection status - Bridge A\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusa](index.html) module"]
pub struct STATUSA_SPEC;
impl crate::RegisterSpec for STATUSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusa::R](R) reader structure"]
impl crate::Readable for STATUSA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSA to value 0x0001_0000"]
impl crate::Resettable for STATUSA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
