#[doc = "Register `STATUSD` reader"]
pub struct R(crate::R<STATUSD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SERCOM4_` reader - SERCOM4 APB Protect Enable"]
pub struct SERCOM4__R(crate::FieldReader<bool, bool>);
impl SERCOM4__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM4__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM4__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM5_` reader - SERCOM5 APB Protect Enable"]
pub struct SERCOM5__R(crate::FieldReader<bool, bool>);
impl SERCOM5__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM5__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM5__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC4_` reader - TCC4 APB Protect Enable"]
pub struct TCC4__R(crate::FieldReader<bool, bool>);
impl TCC4__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC4__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC4__R {
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
#[doc = "Field `ADC1_` reader - ADC1 APB Protect Enable"]
pub struct ADC1__R(crate::FieldReader<bool, bool>);
impl ADC1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_` reader - DAC APB Protect Enable"]
pub struct DAC__R(crate::FieldReader<bool, bool>);
impl DAC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_` reader - I2S APB Protect Enable"]
pub struct I2S__R(crate::FieldReader<bool, bool>);
impl I2S__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCC_` reader - PCC APB Protect Enable"]
pub struct PCC__R(crate::FieldReader<bool, bool>);
impl PCC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - SERCOM4 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom4_(&self) -> SERCOM4__R {
        SERCOM4__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SERCOM5 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom5_(&self) -> SERCOM5__R {
        SERCOM5__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCC4 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc4_(&self) -> TCC4__R {
        TCC4__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC0 APB Protect Enable"]
    #[inline(always)]
    pub fn adc0_(&self) -> ADC0__R {
        ADC0__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC1 APB Protect Enable"]
    #[inline(always)]
    pub fn adc1_(&self) -> ADC1__R {
        ADC1__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DAC APB Protect Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - I2S APB Protect Enable"]
    #[inline(always)]
    pub fn i2s_(&self) -> I2S__R {
        I2S__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PCC APB Protect Enable"]
    #[inline(always)]
    pub fn pcc_(&self) -> PCC__R {
        PCC__R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge D\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusd](index.html) module"]
pub struct STATUSD_SPEC;
impl crate::RegisterSpec for STATUSD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusd::R](R) reader structure"]
impl crate::Readable for STATUSD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSD to value 0"]
impl crate::Resettable for STATUSD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
