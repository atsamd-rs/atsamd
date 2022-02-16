#[doc = "Register `SPI_SR` reader"]
pub struct R(crate::R<SPI_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDRF` reader - Receive Data Register Full (cleared by reading SPI_RDR)"]
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
#[doc = "Field `TDRE` reader - Transmit Data Register Empty (cleared by writing SPI_TDR)"]
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
#[doc = "Field `MODF` reader - Mode Fault Error (cleared on read)"]
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
#[doc = "Field `OVRES` reader - Overrun Error Status (cleared on read)"]
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
#[doc = "Field `NSSR` reader - NSS Rising (cleared on read)"]
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
#[doc = "Field `TXEMPTY` reader - Transmission Registers Empty (cleared by writing SPI_TDR)"]
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
#[doc = "Field `UNDES` reader - Underrun Error Status (Slave mode only) (cleared on read)"]
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
#[doc = "Field `SPIENS` reader - SPI Enable Status"]
pub struct SPIENS_R(crate::FieldReader<bool, bool>);
impl SPIENS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIENS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIENS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receive Data Register Full (cleared by reading SPI_RDR)"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty (cleared by writing SPI_TDR)"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error (cleared on read)"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Status (cleared on read)"]
    #[inline(always)]
    pub fn ovres(&self) -> OVRES_R {
        OVRES_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - NSS Rising (cleared on read)"]
    #[inline(always)]
    pub fn nssr(&self) -> NSSR_R {
        NSSR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission Registers Empty (cleared by writing SPI_TDR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Underrun Error Status (Slave mode only) (cleared on read)"]
    #[inline(always)]
    pub fn undes(&self) -> UNDES_R {
        UNDES_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SPI Enable Status"]
    #[inline(always)]
    pub fn spiens(&self) -> SPIENS_R {
        SPIENS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_sr](index.html) module"]
pub struct SPI_SR_SPEC;
impl crate::RegisterSpec for SPI_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_sr::R](R) reader structure"]
impl crate::Readable for SPI_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_SR to value 0"]
impl crate::Resettable for SPI_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
