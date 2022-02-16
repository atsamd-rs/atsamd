#[doc = "Register `USBHS_HSTIMR` reader"]
pub struct R(crate::R<USBHS_HSTIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_HSTIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_HSTIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_HSTIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DCONNIE` reader - Device Connection Interrupt Enable"]
pub struct DCONNIE_R(crate::FieldReader<bool, bool>);
impl DCONNIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCONNIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCONNIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDISCIE` reader - Device Disconnection Interrupt Enable"]
pub struct DDISCIE_R(crate::FieldReader<bool, bool>);
impl DDISCIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DDISCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDISCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTIE` reader - USB Reset Sent Interrupt Enable"]
pub struct RSTIE_R(crate::FieldReader<bool, bool>);
impl RSTIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSMEDIE` reader - Downstream Resume Sent Interrupt Enable"]
pub struct RSMEDIE_R(crate::FieldReader<bool, bool>);
impl RSMEDIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSMEDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSMEDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRSMIE` reader - Upstream Resume Received Interrupt Enable"]
pub struct RXRSMIE_R(crate::FieldReader<bool, bool>);
impl RXRSMIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRSMIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRSMIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSOFIE` reader - Host Start of Frame Interrupt Enable"]
pub struct HSOFIE_R(crate::FieldReader<bool, bool>);
impl HSOFIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSOFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSOFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HWUPIE` reader - Host Wake-Up Interrupt Enable"]
pub struct HWUPIE_R(crate::FieldReader<bool, bool>);
impl HWUPIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HWUPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HWUPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_0` reader - Pipe 0 Interrupt Enable"]
pub struct PEP_0_R(crate::FieldReader<bool, bool>);
impl PEP_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEP_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_1` reader - Pipe 1 Interrupt Enable"]
pub struct PEP_1_R(crate::FieldReader<bool, bool>);
impl PEP_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEP_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_2` reader - Pipe 2 Interrupt Enable"]
pub struct PEP_2_R(crate::FieldReader<bool, bool>);
impl PEP_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEP_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_3` reader - Pipe 3 Interrupt Enable"]
pub struct PEP_3_R(crate::FieldReader<bool, bool>);
impl PEP_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEP_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_4` reader - Pipe 4 Interrupt Enable"]
pub struct PEP_4_R(crate::FieldReader<bool, bool>);
impl PEP_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEP_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_5` reader - Pipe 5 Interrupt Enable"]
pub struct PEP_5_R(crate::FieldReader<bool, bool>);
impl PEP_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEP_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_6` reader - Pipe 6 Interrupt Enable"]
pub struct PEP_6_R(crate::FieldReader<bool, bool>);
impl PEP_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEP_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_7` reader - Pipe 7 Interrupt Enable"]
pub struct PEP_7_R(crate::FieldReader<bool, bool>);
impl PEP_7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEP_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_8` reader - Pipe 8 Interrupt Enable"]
pub struct PEP_8_R(crate::FieldReader<bool, bool>);
impl PEP_8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEP_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_9` reader - Pipe 9 Interrupt Enable"]
pub struct PEP_9_R(crate::FieldReader<bool, bool>);
impl PEP_9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEP_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEP_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_0` reader - DMA Channel 0 Interrupt Enable"]
pub struct DMA_0_R(crate::FieldReader<bool, bool>);
impl DMA_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_1` reader - DMA Channel 1 Interrupt Enable"]
pub struct DMA_1_R(crate::FieldReader<bool, bool>);
impl DMA_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_2` reader - DMA Channel 2 Interrupt Enable"]
pub struct DMA_2_R(crate::FieldReader<bool, bool>);
impl DMA_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_3` reader - DMA Channel 3 Interrupt Enable"]
pub struct DMA_3_R(crate::FieldReader<bool, bool>);
impl DMA_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_4` reader - DMA Channel 4 Interrupt Enable"]
pub struct DMA_4_R(crate::FieldReader<bool, bool>);
impl DMA_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_5` reader - DMA Channel 5 Interrupt Enable"]
pub struct DMA_5_R(crate::FieldReader<bool, bool>);
impl DMA_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_6` reader - DMA Channel 6 Interrupt Enable"]
pub struct DMA_6_R(crate::FieldReader<bool, bool>);
impl DMA_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Device Connection Interrupt Enable"]
    #[inline(always)]
    pub fn dconnie(&self) -> DCONNIE_R {
        DCONNIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt Enable"]
    #[inline(always)]
    pub fn ddiscie(&self) -> DDISCIE_R {
        DDISCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt Enable"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt Enable"]
    #[inline(always)]
    pub fn rsmedie(&self) -> RSMEDIE_R {
        RSMEDIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt Enable"]
    #[inline(always)]
    pub fn rxrsmie(&self) -> RXRSMIE_R {
        RXRSMIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn hsofie(&self) -> HSOFIE_R {
        HSOFIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt Enable"]
    #[inline(always)]
    pub fn hwupie(&self) -> HWUPIE_R {
        HWUPIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_0(&self) -> PEP_0_R {
        PEP_0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_1(&self) -> PEP_1_R {
        PEP_1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_2(&self) -> PEP_2_R {
        PEP_2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_3(&self) -> PEP_3_R {
        PEP_3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_4(&self) -> PEP_4_R {
        PEP_4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_5(&self) -> PEP_5_R {
        PEP_5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_6(&self) -> PEP_6_R {
        PEP_6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pipe 7 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_7(&self) -> PEP_7_R {
        PEP_7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pipe 8 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_8(&self) -> PEP_8_R {
        PEP_8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pipe 9 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_9(&self) -> PEP_9_R {
        PEP_9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_0(&self) -> DMA_0_R {
        DMA_0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 4 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 5 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DMA Channel 6 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Host Global Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstimr](index.html) module"]
pub struct USBHS_HSTIMR_SPEC;
impl crate::RegisterSpec for USBHS_HSTIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_hstimr::R](R) reader structure"]
impl crate::Readable for USBHS_HSTIMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBHS_HSTIMR to value 0"]
impl crate::Resettable for USBHS_HSTIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
