#[doc = "Register `DACC_ISR` reader"]
pub struct R(crate::R<DACC_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACC_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACC_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACC_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXRDY0` reader - Transmit Ready Interrupt Flag of channel 0"]
pub struct TXRDY0_R(crate::FieldReader<bool, bool>);
impl TXRDY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDY1` reader - Transmit Ready Interrupt Flag of channel 1"]
pub struct TXRDY1_R(crate::FieldReader<bool, bool>);
impl TXRDY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC0` reader - End of Conversion Interrupt Flag of channel 0"]
pub struct EOC0_R(crate::FieldReader<bool, bool>);
impl EOC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC1` reader - End of Conversion Interrupt Flag of channel 1"]
pub struct EOC1_R(crate::FieldReader<bool, bool>);
impl EOC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Ready Interrupt Flag of channel 0"]
    #[inline(always)]
    pub fn txrdy0(&self) -> TXRDY0_R {
        TXRDY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Ready Interrupt Flag of channel 1"]
    #[inline(always)]
    pub fn txrdy1(&self) -> TXRDY1_R {
        TXRDY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Flag of channel 0"]
    #[inline(always)]
    pub fn eoc0(&self) -> EOC0_R {
        EOC0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Flag of channel 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_isr](index.html) module"]
pub struct DACC_ISR_SPEC;
impl crate::RegisterSpec for DACC_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dacc_isr::R](R) reader structure"]
impl crate::Readable for DACC_ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DACC_ISR to value 0"]
impl crate::Resettable for DACC_ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
