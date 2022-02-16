#[doc = "Register `QSPI_SR` reader"]
pub struct R(crate::R<QSPI_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_SR_SPEC>) -> Self {
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
#[doc = "Field `CSR` reader - Chip Select Rise (cleared on read)"]
pub struct CSR_R(crate::FieldReader<bool, bool>);
impl CSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSS` reader - Chip Select Status"]
pub struct CSS_R(crate::FieldReader<bool, bool>);
impl CSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSTRE` reader - Instruction End Status (cleared on read)"]
pub struct INSTRE_R(crate::FieldReader<bool, bool>);
impl INSTRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INSTRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSTRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QSPIENS` reader - QSPI Enable Status"]
pub struct QSPIENS_R(crate::FieldReader<bool, bool>);
impl QSPIENS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QSPIENS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QSPIENS_R {
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
    #[doc = "Bit 2 - Transmission Registers Empty (cleared by writing SPI_TDR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Status (cleared on read)"]
    #[inline(always)]
    pub fn ovres(&self) -> OVRES_R {
        OVRES_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Chip Select Rise (cleared on read)"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Chip Select Status"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Instruction End Status (cleared on read)"]
    #[inline(always)]
    pub fn instre(&self) -> INSTRE_R {
        INSTRE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 24 - QSPI Enable Status"]
    #[inline(always)]
    pub fn qspiens(&self) -> QSPIENS_R {
        QSPIENS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_sr](index.html) module"]
pub struct QSPI_SR_SPEC;
impl crate::RegisterSpec for QSPI_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_sr::R](R) reader structure"]
impl crate::Readable for QSPI_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QSPI_SR to value 0"]
impl crate::Resettable for QSPI_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
