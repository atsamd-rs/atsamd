#[doc = "Register `MCAN_TXFQS` reader"]
pub struct R(crate::R<MCAN_TXFQS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_TXFQS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_TXFQS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_TXFQS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TFFL` reader - Tx FIFO Free Level"]
pub struct TFFL_R(crate::FieldReader<u8, u8>);
impl TFFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TFFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFGI` reader - Tx FIFO Get Index"]
pub struct TFGI_R(crate::FieldReader<u8, u8>);
impl TFGI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TFGI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFGI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFQPI` reader - Tx FIFO/Queue Put Index"]
pub struct TFQPI_R(crate::FieldReader<u8, u8>);
impl TFQPI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TFQPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFQPI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFQF` reader - Tx FIFO/Queue Full"]
pub struct TFQF_R(crate::FieldReader<bool, bool>);
impl TFQF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFQF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFQF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Tx FIFO Free Level"]
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Tx FIFO Get Index"]
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Tx FIFO/Queue Put Index"]
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/Queue Full"]
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
#[doc = "Transmit FIFO/Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txfqs](index.html) module"]
pub struct MCAN_TXFQS_SPEC;
impl crate::RegisterSpec for MCAN_TXFQS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_txfqs::R](R) reader structure"]
impl crate::Readable for MCAN_TXFQS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MCAN_TXFQS to value 0"]
impl crate::Resettable for MCAN_TXFQS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
