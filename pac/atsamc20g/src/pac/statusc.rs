#[doc = "Register `STATUSC` reader"]
pub struct R(crate::R<STATUSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EVSYS_` reader - EVSYS APB Protect Enable"]
pub struct EVSYS__R(crate::FieldReader<bool, bool>);
impl EVSYS__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVSYS__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVSYS__R {
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
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Protect Enable"]
pub struct SERCOM2__R(crate::FieldReader<bool, bool>);
impl SERCOM2__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM3_` reader - SERCOM3 APB Protect Enable"]
pub struct SERCOM3__R(crate::FieldReader<bool, bool>);
impl SERCOM3__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM3__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM3__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC0_` reader - TCC0 APB Protect Enable"]
pub struct TCC0__R(crate::FieldReader<bool, bool>);
impl TCC0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC1_` reader - TCC1 APB Protect Enable"]
pub struct TCC1__R(crate::FieldReader<bool, bool>);
impl TCC1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC2_` reader - TCC2 APB Protect Enable"]
pub struct TCC2__R(crate::FieldReader<bool, bool>);
impl TCC2__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC2__R {
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
#[doc = "Field `TC2_` reader - TC2 APB Protect Enable"]
pub struct TC2__R(crate::FieldReader<bool, bool>);
impl TC2__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC3_` reader - TC3 APB Protect Enable"]
pub struct TC3__R(crate::FieldReader<bool, bool>);
impl TC3__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC3__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC3__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC4_` reader - TC4 APB Protect Enable"]
pub struct TC4__R(crate::FieldReader<bool, bool>);
impl TC4__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC4__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC4__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0_` reader - ADC0 APB Protect Enable"]
pub struct ADC0__R(crate::FieldReader<bool, bool>);
impl ADC0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AC_` reader - AC APB Protect Enable"]
pub struct AC__R(crate::FieldReader<bool, bool>);
impl AC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTC_` reader - PTC APB Protect Enable"]
pub struct PTC__R(crate::FieldReader<bool, bool>);
impl PTC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PTC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCL_` reader - CCL APB Protect Enable"]
pub struct CCL__R(crate::FieldReader<bool, bool>);
impl CCL__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCL__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCL__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - EVSYS APB Protect Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SERCOM0 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SERCOM1 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SERCOM2 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SERCOM3 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> SERCOM3__R {
        SERCOM3__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TCC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> TCC0__R {
        TCC0__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TCC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc1_(&self) -> TCC1__R {
        TCC1__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TCC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc2_(&self) -> TCC2__R {
        TCC2__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TC3 APB Protect Enable"]
    #[inline(always)]
    pub fn tc3_(&self) -> TC3__R {
        TC3__R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TC4 APB Protect Enable"]
    #[inline(always)]
    pub fn tc4_(&self) -> TC4__R {
        TC4__R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC0 APB Protect Enable"]
    #[inline(always)]
    pub fn adc0_(&self) -> ADC0__R {
        ADC0__R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - AC APB Protect Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PTC APB Protect Enable"]
    #[inline(always)]
    pub fn ptc_(&self) -> PTC__R {
        PTC__R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CCL APB Protect Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusc](index.html) module"]
pub struct STATUSC_SPEC;
impl crate::RegisterSpec for STATUSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusc::R](R) reader structure"]
impl crate::Readable for STATUSC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSC to value 0x0200_0000"]
impl crate::Resettable for STATUSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0000
    }
}
