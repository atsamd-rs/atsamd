#[doc = "Register `ISI_SR` reader"]
pub struct R(crate::R<ISI_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISI_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISI_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISI_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENABLE` reader - Module Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_DONE` reader - Module Disable Request has Terminated (cleared on read)"]
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
#[doc = "Field `SRST` reader - Module Software Reset Request has Terminated (cleared on read)"]
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
#[doc = "Field `CDC_PND` reader - Pending Codec Request"]
pub struct CDC_PND_R(crate::FieldReader<bool, bool>);
impl CDC_PND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDC_PND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDC_PND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSYNC` reader - Vertical Synchronization (cleared on read)"]
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
#[doc = "Field `PXFR_DONE` reader - Preview DMA Transfer has Terminated (cleared on read)"]
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
#[doc = "Field `CXFR_DONE` reader - Codec DMA Transfer has Terminated (cleared on read)"]
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
#[doc = "Field `SIP` reader - Synchronization in Progress"]
pub struct SIP_R(crate::FieldReader<bool, bool>);
impl SIP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P_OVR` reader - Preview Datapath Overflow (cleared on read)"]
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
#[doc = "Field `C_OVR` reader - Codec Datapath Overflow (cleared on read)"]
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
#[doc = "Field `CRC_ERR` reader - CRC Synchronization Error (cleared on read)"]
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
#[doc = "Field `FR_OVR` reader - Frame Rate Overrun (cleared on read)"]
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
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Module Disable Request has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn dis_done(&self) -> DIS_DONE_R {
        DIS_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Module Software Reset Request has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pending Codec Request"]
    #[inline(always)]
    pub fn cdc_pnd(&self) -> CDC_PND_R {
        CDC_PND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Vertical Synchronization (cleared on read)"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Preview DMA Transfer has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn pxfr_done(&self) -> PXFR_DONE_R {
        PXFR_DONE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Codec DMA Transfer has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn cxfr_done(&self) -> CXFR_DONE_R {
        CXFR_DONE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Synchronization in Progress"]
    #[inline(always)]
    pub fn sip(&self) -> SIP_R {
        SIP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Preview Datapath Overflow (cleared on read)"]
    #[inline(always)]
    pub fn p_ovr(&self) -> P_OVR_R {
        P_OVR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Codec Datapath Overflow (cleared on read)"]
    #[inline(always)]
    pub fn c_ovr(&self) -> C_OVR_R {
        C_OVR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CRC Synchronization Error (cleared on read)"]
    #[inline(always)]
    pub fn crc_err(&self) -> CRC_ERR_R {
        CRC_ERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Frame Rate Overrun (cleared on read)"]
    #[inline(always)]
    pub fn fr_ovr(&self) -> FR_OVR_R {
        FR_OVR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
#[doc = "ISI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_sr](index.html) module"]
pub struct ISI_SR_SPEC;
impl crate::RegisterSpec for ISI_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isi_sr::R](R) reader structure"]
impl crate::Readable for ISI_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISI_SR to value 0"]
impl crate::Resettable for ISI_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
