#[doc = "Register `ISI_IMR` reader"]
pub struct R(crate::R<ISI_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISI_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISI_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISI_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIS_DONE` reader - Module Disable Operation Completed"]
pub struct DIS_DONE_R(crate::FieldReader<bool, bool>);
impl DIS_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRST` reader - Software Reset Completed"]
pub struct SRST_R(crate::FieldReader<bool, bool>);
impl SRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSYNC` reader - Vertical Synchronization"]
pub struct VSYNC_R(crate::FieldReader<bool, bool>);
impl VSYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PXFR_DONE` reader - Preview DMA Transfer Completed"]
pub struct PXFR_DONE_R(crate::FieldReader<bool, bool>);
impl PXFR_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PXFR_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PXFR_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CXFR_DONE` reader - Codec DMA Transfer Completed"]
pub struct CXFR_DONE_R(crate::FieldReader<bool, bool>);
impl CXFR_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CXFR_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CXFR_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P_OVR` reader - Preview FIFO Overflow"]
pub struct P_OVR_R(crate::FieldReader<bool, bool>);
impl P_OVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P_OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P_OVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C_OVR` reader - Codec FIFO Overflow"]
pub struct C_OVR_R(crate::FieldReader<bool, bool>);
impl C_OVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        C_OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_OVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_ERR` reader - CRC Synchronization Error"]
pub struct CRC_ERR_R(crate::FieldReader<bool, bool>);
impl CRC_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FR_OVR` reader - Frame Rate Overrun"]
pub struct FR_OVR_R(crate::FieldReader<bool, bool>);
impl FR_OVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FR_OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FR_OVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Module Disable Operation Completed"]
    #[inline(always)]
    pub fn dis_done(&self) -> DIS_DONE_R {
        DIS_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software Reset Completed"]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Vertical Synchronization"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Preview DMA Transfer Completed"]
    #[inline(always)]
    pub fn pxfr_done(&self) -> PXFR_DONE_R {
        PXFR_DONE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Codec DMA Transfer Completed"]
    #[inline(always)]
    pub fn cxfr_done(&self) -> CXFR_DONE_R {
        CXFR_DONE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Preview FIFO Overflow"]
    #[inline(always)]
    pub fn p_ovr(&self) -> P_OVR_R {
        P_OVR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Codec FIFO Overflow"]
    #[inline(always)]
    pub fn c_ovr(&self) -> C_OVR_R {
        C_OVR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CRC Synchronization Error"]
    #[inline(always)]
    pub fn crc_err(&self) -> CRC_ERR_R {
        CRC_ERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Frame Rate Overrun"]
    #[inline(always)]
    pub fn fr_ovr(&self) -> FR_OVR_R {
        FR_OVR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
#[doc = "ISI Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_imr](index.html) module"]
pub struct ISI_IMR_SPEC;
impl crate::RegisterSpec for ISI_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isi_imr::R](R) reader structure"]
impl crate::Readable for ISI_IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISI_IMR to value 0"]
impl crate::Resettable for ISI_IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
