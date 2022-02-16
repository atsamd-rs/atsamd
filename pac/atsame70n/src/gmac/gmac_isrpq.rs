#[doc = "Register `GMAC_ISRPQ[%s]` reader"]
pub struct R(crate::R<GMAC_ISRPQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_ISRPQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_ISRPQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_ISRPQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RCOMP` reader - Receive Complete"]
pub struct RCOMP_R(crate::FieldReader<bool, bool>);
impl RCOMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCOMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUBR` reader - RX Used Bit Read"]
pub struct RXUBR_R(crate::FieldReader<bool, bool>);
impl RXUBR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXUBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUBR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLEX` reader - Retry Limit Exceeded or Late Collision"]
pub struct RLEX_R(crate::FieldReader<bool, bool>);
impl RLEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RLEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLEX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFC` reader - Transmit Frame Corruption Due to AHB Error"]
pub struct TFC_R(crate::FieldReader<bool, bool>);
impl TFC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCOMP` reader - Transmit Complete"]
pub struct TCOMP_R(crate::FieldReader<bool, bool>);
impl TCOMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCOMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub struct ROVR_R(crate::FieldReader<bool, bool>);
impl ROVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRESP` reader - HRESP Not OK"]
pub struct HRESP_R(crate::FieldReader<bool, bool>);
impl HRESP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRESP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRESP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&self) -> RCOMP_R {
        RCOMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&self) -> RXUBR_R {
        RXUBR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded or Late Collision"]
    #[inline(always)]
    pub fn rlex(&self) -> RLEX_R {
        RLEX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    pub fn tfc(&self) -> TFC_R {
        TFC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TCOMP_R {
        TCOMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HRESP_R {
        HRESP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
#[doc = "Interrupt Status Register Priority Queue (1..5)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_isrpq](index.html) module"]
pub struct GMAC_ISRPQ_SPEC;
impl crate::RegisterSpec for GMAC_ISRPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_isrpq::R](R) reader structure"]
impl crate::Readable for GMAC_ISRPQ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_ISRPQ[%s]
to value 0"]
impl crate::Resettable for GMAC_ISRPQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
