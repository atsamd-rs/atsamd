#[doc = "Register `US_CSR_LON_SPI_MODE` reader"]
pub struct R(crate::R<US_CSR_LON_SPI_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_CSR_LON_SPI_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_CSR_LON_SPI_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_CSR_LON_SPI_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - Receiver Ready (cleared by reading US_RHR)"]
pub struct RXRDY_R(crate::FieldReader<bool, bool>);
impl RXRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDY` reader - Transmitter Ready (cleared by writing US_THR)"]
pub struct TXRDY_R(crate::FieldReader<bool, bool>);
impl TXRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE` reader - Overrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
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
#[doc = "Field `TXEMPTY` reader - Transmitter Empty (cleared by writing US_THR)"]
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
#[doc = "Field `RIIC` reader - Ring Indicator Input Change Flag (cleared on read)"]
pub struct RIIC_R(crate::FieldReader<bool, bool>);
impl RIIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RIIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRIC` reader - Data Set Ready Input Change Flag (cleared on read)"]
pub struct DSRIC_R(crate::FieldReader<bool, bool>);
impl DSRIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSRIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDIC` reader - Data Carrier Detect Input Change Flag (cleared on read)"]
pub struct DCDIC_R(crate::FieldReader<bool, bool>);
impl DCDIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNRE` reader - SPI Underrun Error"]
pub struct UNRE_R(crate::FieldReader<bool, bool>);
impl UNRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Ready (cleared by reading US_RHR)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitter Ready (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Flag (cleared on read)"]
    #[inline(always)]
    pub fn riic(&self) -> RIIC_R {
        RIIC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Flag (cleared on read)"]
    #[inline(always)]
    pub fn dsric(&self) -> DSRIC_R {
        DSRIC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Flag (cleared on read)"]
    #[inline(always)]
    pub fn dcdic(&self) -> DCDIC_R {
        DCDIC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPI Underrun Error"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_csr_lon_spi_mode](index.html) module"]
pub struct US_CSR_LON_SPI_MODE_SPEC;
impl crate::RegisterSpec for US_CSR_LON_SPI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_csr_lon_spi_mode::R](R) reader structure"]
impl crate::Readable for US_CSR_LON_SPI_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets US_CSR_LON_SPI_MODE to value 0"]
impl crate::Resettable for US_CSR_LON_SPI_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
