#[doc = "Register `USBHS_HSTISR` reader"]
pub struct R(crate::R<USBHS_HSTISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_HSTISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_HSTISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_HSTISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DCONNI` reader - Device Connection Interrupt"]
pub struct DCONNI_R(crate::FieldReader<bool, bool>);
impl DCONNI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCONNI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCONNI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDISCI` reader - Device Disconnection Interrupt"]
pub struct DDISCI_R(crate::FieldReader<bool, bool>);
impl DDISCI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DDISCI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDISCI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTI` reader - USB Reset Sent Interrupt"]
pub struct RSTI_R(crate::FieldReader<bool, bool>);
impl RSTI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSMEDI` reader - Downstream Resume Sent Interrupt"]
pub struct RSMEDI_R(crate::FieldReader<bool, bool>);
impl RSMEDI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSMEDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSMEDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRSMI` reader - Upstream Resume Received Interrupt"]
pub struct RXRSMI_R(crate::FieldReader<bool, bool>);
impl RXRSMI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRSMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRSMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSOFI` reader - Host Start of Frame Interrupt"]
pub struct HSOFI_R(crate::FieldReader<bool, bool>);
impl HSOFI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSOFI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSOFI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HWUPI` reader - Host Wake-Up Interrupt"]
pub struct HWUPI_R(crate::FieldReader<bool, bool>);
impl HWUPI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HWUPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HWUPI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEP_0` reader - Pipe 0 Interrupt"]
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
#[doc = "Field `PEP_1` reader - Pipe 1 Interrupt"]
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
#[doc = "Field `PEP_2` reader - Pipe 2 Interrupt"]
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
#[doc = "Field `PEP_3` reader - Pipe 3 Interrupt"]
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
#[doc = "Field `PEP_4` reader - Pipe 4 Interrupt"]
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
#[doc = "Field `PEP_5` reader - Pipe 5 Interrupt"]
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
#[doc = "Field `PEP_6` reader - Pipe 6 Interrupt"]
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
#[doc = "Field `PEP_7` reader - Pipe 7 Interrupt"]
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
#[doc = "Field `PEP_8` reader - Pipe 8 Interrupt"]
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
#[doc = "Field `PEP_9` reader - Pipe 9 Interrupt"]
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
#[doc = "Field `DMA_0` reader - DMA Channel 0 Interrupt"]
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
#[doc = "Field `DMA_1` reader - DMA Channel 1 Interrupt"]
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
#[doc = "Field `DMA_2` reader - DMA Channel 2 Interrupt"]
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
#[doc = "Field `DMA_3` reader - DMA Channel 3 Interrupt"]
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
#[doc = "Field `DMA_4` reader - DMA Channel 4 Interrupt"]
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
#[doc = "Field `DMA_5` reader - DMA Channel 5 Interrupt"]
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
#[doc = "Field `DMA_6` reader - DMA Channel 6 Interrupt"]
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
    #[doc = "Bit 0 - Device Connection Interrupt"]
    #[inline(always)]
    pub fn dconni(&self) -> DCONNI_R {
        DCONNI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt"]
    #[inline(always)]
    pub fn ddisci(&self) -> DDISCI_R {
        DDISCI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt"]
    #[inline(always)]
    pub fn rsti(&self) -> RSTI_R {
        RSTI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt"]
    #[inline(always)]
    pub fn rsmedi(&self) -> RSMEDI_R {
        RSMEDI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt"]
    #[inline(always)]
    pub fn rxrsmi(&self) -> RXRSMI_R {
        RXRSMI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt"]
    #[inline(always)]
    pub fn hsofi(&self) -> HSOFI_R {
        HSOFI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt"]
    #[inline(always)]
    pub fn hwupi(&self) -> HWUPI_R {
        HWUPI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt"]
    #[inline(always)]
    pub fn pep_0(&self) -> PEP_0_R {
        PEP_0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt"]
    #[inline(always)]
    pub fn pep_1(&self) -> PEP_1_R {
        PEP_1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt"]
    #[inline(always)]
    pub fn pep_2(&self) -> PEP_2_R {
        PEP_2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt"]
    #[inline(always)]
    pub fn pep_3(&self) -> PEP_3_R {
        PEP_3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt"]
    #[inline(always)]
    pub fn pep_4(&self) -> PEP_4_R {
        PEP_4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt"]
    #[inline(always)]
    pub fn pep_5(&self) -> PEP_5_R {
        PEP_5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt"]
    #[inline(always)]
    pub fn pep_6(&self) -> PEP_6_R {
        PEP_6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pipe 7 Interrupt"]
    #[inline(always)]
    pub fn pep_7(&self) -> PEP_7_R {
        PEP_7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pipe 8 Interrupt"]
    #[inline(always)]
    pub fn pep_8(&self) -> PEP_8_R {
        PEP_8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pipe 9 Interrupt"]
    #[inline(always)]
    pub fn pep_9(&self) -> PEP_9_R {
        PEP_9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 0 Interrupt"]
    #[inline(always)]
    pub fn dma_0(&self) -> DMA_0_R {
        DMA_0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 1 Interrupt"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 2 Interrupt"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 3 Interrupt"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 4 Interrupt"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 5 Interrupt"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DMA Channel 6 Interrupt"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Host Global Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstisr](index.html) module"]
pub struct USBHS_HSTISR_SPEC;
impl crate::RegisterSpec for USBHS_HSTISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_hstisr::R](R) reader structure"]
impl crate::Readable for USBHS_HSTISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBHS_HSTISR to value 0"]
impl crate::Resettable for USBHS_HSTISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
