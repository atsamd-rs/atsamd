#[doc = "Register `PIO_PCIMR` reader"]
pub struct R(crate::R<PIO_PCIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO_PCIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO_PCIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO_PCIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DRDY` reader - Parallel Capture Mode Data Ready Interrupt Mask"]
pub struct DRDY_R(crate::FieldReader<bool, bool>);
impl DRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE` reader - Parallel Capture Mode Overrun Error Interrupt Mask"]
pub struct OVRE_R(crate::FieldReader<bool, bool>);
impl OVRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDRX` reader - End of Reception Transfer Interrupt Mask"]
pub struct ENDRX_R(crate::FieldReader<bool, bool>);
impl ENDRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBUFF` reader - Reception Buffer Full Interrupt Mask"]
pub struct RXBUFF_R(crate::FieldReader<bool, bool>);
impl RXBUFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXBUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBUFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Parallel Capture Mode Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parallel Capture Mode Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of Reception Transfer Interrupt Mask"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reception Buffer Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "Parallel Capture Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pcimr](index.html) module"]
pub struct PIO_PCIMR_SPEC;
impl crate::RegisterSpec for PIO_PCIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio_pcimr::R](R) reader structure"]
impl crate::Readable for PIO_PCIMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIO_PCIMR to value 0"]
impl crate::Resettable for PIO_PCIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
