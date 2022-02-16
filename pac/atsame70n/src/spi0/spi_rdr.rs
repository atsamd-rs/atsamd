#[doc = "Register `SPI_RDR` reader"]
pub struct R(crate::R<SPI_RDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_RDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_RDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_RDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD` reader - Receive Data"]
pub struct RD_R(crate::FieldReader<u16, u16>);
impl RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCS` reader - Peripheral Chip Select"]
pub struct PCS_R(crate::FieldReader<u8, u8>);
impl PCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Receive Data"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_rdr](index.html) module"]
pub struct SPI_RDR_SPEC;
impl crate::RegisterSpec for SPI_RDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_rdr::R](R) reader structure"]
impl crate::Readable for SPI_RDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_RDR to value 0"]
impl crate::Resettable for SPI_RDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
