#[doc = "Register `GMAC_TXLPI` reader"]
pub struct R(crate::R<GMAC_TXLPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_TXLPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_TXLPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_TXLPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Count of LPI transitions (cleared on read)"]
pub struct COUNT_R(crate::FieldReader<u16, u16>);
impl COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Count of LPI transitions (cleared on read)"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Transmit LPI Transitions\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_txlpi](index.html) module"]
pub struct GMAC_TXLPI_SPEC;
impl crate::RegisterSpec for GMAC_TXLPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_txlpi::R](R) reader structure"]
impl crate::Readable for GMAC_TXLPI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_TXLPI to value 0"]
impl crate::Resettable for GMAC_TXLPI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
