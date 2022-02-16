#[doc = "Register `XDMAC_GTYPE` reader"]
pub struct R(crate::R<XDMAC_GTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDMAC_GTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDMAC_GTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDMAC_GTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NB_CH` reader - Number of Channels Minus One"]
pub struct NB_CH_R(crate::FieldReader<u8, u8>);
impl NB_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NB_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NB_CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_SZ` reader - Number of Bytes"]
pub struct FIFO_SZ_R(crate::FieldReader<u16, u16>);
impl FIFO_SZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FIFO_SZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_SZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NB_REQ` reader - Number of Peripheral Requests Minus One"]
pub struct NB_REQ_R(crate::FieldReader<u8, u8>);
impl NB_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NB_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NB_REQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of Channels Minus One"]
    #[inline(always)]
    pub fn nb_ch(&self) -> NB_CH_R {
        NB_CH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:15 - Number of Bytes"]
    #[inline(always)]
    pub fn fifo_sz(&self) -> FIFO_SZ_R {
        FIFO_SZ_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:22 - Number of Peripheral Requests Minus One"]
    #[inline(always)]
    pub fn nb_req(&self) -> NB_REQ_R {
        NB_REQ_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Global Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gtype](index.html) module"]
pub struct XDMAC_GTYPE_SPEC;
impl crate::RegisterSpec for XDMAC_GTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xdmac_gtype::R](R) reader structure"]
impl crate::Readable for XDMAC_GTYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets XDMAC_GTYPE to value 0"]
impl crate::Resettable for XDMAC_GTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
