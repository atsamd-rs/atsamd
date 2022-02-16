#[doc = "Register `SPI_IMR` reader"]
pub struct R(crate::R<SPI_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDRF` reader - Receive Data Register Full Interrupt Mask"]
pub struct RDRF_R(crate::FieldReader<bool, bool>);
impl RDRF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDRE` reader - SPI Transmit Data Register Empty Interrupt Mask"]
pub struct TDRE_R(crate::FieldReader<bool, bool>);
impl TDRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODF` reader - Mode Fault Error Interrupt Mask"]
pub struct MODF_R(crate::FieldReader<bool, bool>);
impl MODF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRES` reader - Overrun Error Interrupt Mask"]
pub struct OVRES_R(crate::FieldReader<bool, bool>);
impl OVRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSSR` reader - NSS Rising Interrupt Mask"]
pub struct NSSR_R(crate::FieldReader<bool, bool>);
impl NSSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NSSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSSR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEMPTY` reader - Transmission Registers Empty Mask"]
pub struct TXEMPTY_R(crate::FieldReader<bool, bool>);
impl TXEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDES` reader - Underrun Error Interrupt Mask"]
pub struct UNDES_R(crate::FieldReader<bool, bool>);
impl UNDES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNDES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNDES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Mask"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPI Transmit Data Register Empty Interrupt Mask"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Mask"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovres(&self) -> OVRES_R {
        OVRES_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - NSS Rising Interrupt Mask"]
    #[inline(always)]
    pub fn nssr(&self) -> NSSR_R {
        NSSR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission Registers Empty Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn undes(&self) -> UNDES_R {
        UNDES_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_imr](index.html) module"]
pub struct SPI_IMR_SPEC;
impl crate::RegisterSpec for SPI_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_imr::R](R) reader structure"]
impl crate::Readable for SPI_IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_IMR to value 0"]
impl crate::Resettable for SPI_IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
