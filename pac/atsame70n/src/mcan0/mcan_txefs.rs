#[doc = "Register `MCAN_TXEFS` reader"]
pub struct R(crate::R<MCAN_TXEFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_TXEFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_TXEFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_TXEFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EFFL` reader - Event FIFO Fill Level"]
pub struct EFFL_R(crate::FieldReader<u8, u8>);
impl EFFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EFFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFGI` reader - Event FIFO Get Index"]
pub struct EFGI_R(crate::FieldReader<u8, u8>);
impl EFGI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EFGI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFGI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFPI` reader - Event FIFO Put Index"]
pub struct EFPI_R(crate::FieldReader<u8, u8>);
impl EFPI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EFPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFPI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFF` reader - Event FIFO Full"]
pub struct EFF_R(crate::FieldReader<bool, bool>);
impl EFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEFL` reader - Tx Event FIFO Element Lost"]
pub struct TEFL_R(crate::FieldReader<bool, bool>);
impl TEFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Event FIFO Fill Level"]
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Event FIFO Get Index"]
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Event FIFO Put Index"]
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Event FIFO Full"]
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
#[doc = "Transmit Event FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txefs](index.html) module"]
pub struct MCAN_TXEFS_SPEC;
impl crate::RegisterSpec for MCAN_TXEFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_txefs::R](R) reader structure"]
impl crate::Readable for MCAN_TXEFS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MCAN_TXEFS to value 0"]
impl crate::Resettable for MCAN_TXEFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
