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
pub struct PAC0__R(crate::FieldReader<bool, bool>);
impl PAC0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAC0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAC0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAC0_` writer - PAC0 APB Clock Enable"]
pub struct PAC0__W<'a> {
    w: &'a mut W,
}
impl<'a> PAC0__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `PM_` reader - PM APB Clock Enable"]
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
#[doc = "Field `PM_` writer - PM APB Clock Enable"]
pub struct PM__W<'a> {
    w: &'a mut W,
}
impl<'a> PM__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SYSCTRL_` reader - SYSCTRL APB Clock Enable"]
pub struct SYSCTRL__R(crate::FieldReader<bool, bool>);
impl SYSCTRL__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSCTRL__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCTRL__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCTRL_` writer - SYSCTRL APB Clock Enable"]
pub struct SYSCTRL__W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTRL__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `GCLK_` reader - GCLK APB Clock Enable"]
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
#[doc = "Field `GCLK_` writer - GCLK APB Clock Enable"]
pub struct GCLK__W<'a> {
    w: &'a mut W,
}
impl<'a> GCLK__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `WDT_` reader - WDT APB Clock Enable"]
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
#[doc = "Field `WDT_` writer - WDT APB Clock Enable"]
pub struct WDT__W<'a> {
    w: &'a mut W,
}
impl<'a> WDT__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RTC_` reader - RTC APB Clock Enable"]
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
#[doc = "Field `RTC_` writer - RTC APB Clock Enable"]
pub struct RTC__W<'a> {
    w: &'a mut W,
}
impl<'a> RTC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `EIC_` reader - EIC APB Clock Enable"]
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
#[doc = "Field `EIC_` writer - EIC APB Clock Enable"]
pub struct EIC__W<'a> {
    w: &'a mut W,
}
impl<'a> EIC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PAC0 APB Clock Enable"]
    #[inline(always)]
    pub fn pac0_(&self) -> PAC0__R {
        PAC0__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PM APB Clock Enable"]
    #[inline(always)]
    pub fn pm_(&self) -> PM__R {
        PM__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SYSCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn sysctrl_(&self) -> SYSCTRL__R {
        SYSCTRL__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GCLK APB Clock Enable"]
    #[inline(always)]
    pub fn gclk_(&self) -> GCLK__R {
        GCLK__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WDT APB Clock Enable"]
    #[inline(always)]
    pub fn wdt_(&self) -> WDT__R {
        WDT__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC APB Clock Enable"]
    #[inline(always)]
    pub fn rtc_(&self) -> RTC__R {
        RTC__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EIC APB Clock Enable"]
    #[inline(always)]
    pub fn eic_(&self) -> EIC__R {
        EIC__R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC0 APB Clock Enable"]
    #[inline(always)]
    pub fn pac0_(&mut self) -> PAC0__W {
        PAC0__W { w: self }
    }
    #[doc = "Bit 1 - PM APB Clock Enable"]
    #[inline(always)]
    pub fn pm_(&mut self) -> PM__W {
        PM__W { w: self }
    }
    #[doc = "Bit 2 - SYSCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn sysctrl_(&mut self) -> SYSCTRL__W {
        SYSCTRL__W { w: self }
    }
    #[doc = "Bit 3 - GCLK APB Clock Enable"]
    #[inline(always)]
    pub fn gclk_(&mut self) -> GCLK__W {
        GCLK__W { w: self }
    }
    #[doc = "Bit 4 - WDT APB Clock Enable"]
    #[inline(always)]
    pub fn wdt_(&mut self) -> WDT__W {
        WDT__W { w: self }
    }
    #[doc = "Bit 5 - RTC APB Clock Enable"]
    #[inline(always)]
    pub fn rtc_(&mut self) -> RTC__W {
        RTC__W { w: self }
    }
    #[doc = "Bit 6 - EIC APB Clock Enable"]
    #[inline(always)]
    pub fn eic_(&mut self) -> EIC__W {
        EIC__W { w: self }
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
}
#[doc = "`reset()` method sets APBAMASK to value 0x7f"]
impl crate::Resettable for APBAMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}
