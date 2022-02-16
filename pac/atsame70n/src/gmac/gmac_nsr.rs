#[doc = "Register `GMAC_NSR` reader"]
pub struct R(crate::R<GMAC_NSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_NSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_NSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_NSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MDIO` reader - MDIO Input Status"]
pub struct MDIO_R(crate::FieldReader<bool, bool>);
impl MDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE` reader - PHY Management Logic Idle"]
pub struct IDLE_R(crate::FieldReader<bool, bool>);
impl IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXLPIS` reader - LPI Indication"]
pub struct RXLPIS_R(crate::FieldReader<bool, bool>);
impl RXLPIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXLPIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXLPIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - MDIO Input Status"]
    #[inline(always)]
    pub fn mdio(&self) -> MDIO_R {
        MDIO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PHY Management Logic Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPI Indication"]
    #[inline(always)]
    pub fn rxlpis(&self) -> RXLPIS_R {
        RXLPIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Network Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_nsr](index.html) module"]
pub struct GMAC_NSR_SPEC;
impl crate::RegisterSpec for GMAC_NSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_nsr::R](R) reader structure"]
impl crate::Readable for GMAC_NSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_NSR to value 0"]
impl crate::Resettable for GMAC_NSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
