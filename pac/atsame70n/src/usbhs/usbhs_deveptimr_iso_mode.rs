#[doc = "Register `USBHS_DEVEPTIMR_ISO_MODE[%s]` reader"]
pub struct R(crate::R<USBHS_DEVEPTIMR_ISO_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_DEVEPTIMR_ISO_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_DEVEPTIMR_ISO_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_DEVEPTIMR_ISO_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXINE` reader - Transmitted IN Data Interrupt"]
pub struct TXINE_R(crate::FieldReader<bool, bool>);
impl TXINE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXINE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOUTE` reader - Received OUT Data Interrupt"]
pub struct RXOUTE_R(crate::FieldReader<bool, bool>);
impl RXOUTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXOUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOUTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDERFE` reader - Underflow Interrupt"]
pub struct UNDERFE_R(crate::FieldReader<bool, bool>);
impl UNDERFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNDERFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNDERFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HBISOINERRE` reader - High Bandwidth Isochronous IN Underflow Error Interrupt"]
pub struct HBISOINERRE_R(crate::FieldReader<bool, bool>);
impl HBISOINERRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HBISOINERRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBISOINERRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HBISOFLUSHE` reader - High Bandwidth Isochronous IN Flush Interrupt"]
pub struct HBISOFLUSHE_R(crate::FieldReader<bool, bool>);
impl HBISOFLUSHE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HBISOFLUSHE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBISOFLUSHE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERFE` reader - Overflow Interrupt"]
pub struct OVERFE_R(crate::FieldReader<bool, bool>);
impl OVERFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCERRE` reader - CRC Error Interrupt"]
pub struct CRCERRE_R(crate::FieldReader<bool, bool>);
impl CRCERRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRCERRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCERRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHORTPACKETE` reader - Short Packet Interrupt"]
pub struct SHORTPACKETE_R(crate::FieldReader<bool, bool>);
impl SHORTPACKETE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHORTPACKETE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHORTPACKETE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDATAE` reader - MData Interrupt"]
pub struct MDATAE_R(crate::FieldReader<bool, bool>);
impl MDATAE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MDATAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDATAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAXE` reader - DataX Interrupt"]
pub struct DATAXE_R(crate::FieldReader<bool, bool>);
impl DATAXE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATAXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRORTRANSE` reader - Transaction Error Interrupt"]
pub struct ERRORTRANSE_R(crate::FieldReader<bool, bool>);
impl ERRORTRANSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRORTRANSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRORTRANSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBUSYBKE` reader - Number of Busy Banks Interrupt"]
pub struct NBUSYBKE_R(crate::FieldReader<bool, bool>);
impl NBUSYBKE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NBUSYBKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBUSYBKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KILLBK` reader - Kill IN Bank"]
pub struct KILLBK_R(crate::FieldReader<bool, bool>);
impl KILLBK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KILLBK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KILLBK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOCON` reader - FIFO Control"]
pub struct FIFOCON_R(crate::FieldReader<bool, bool>);
impl FIFOCON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFOCON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOCON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPDISHDMA` reader - Endpoint Interrupts Disable HDMA Request"]
pub struct EPDISHDMA_R(crate::FieldReader<bool, bool>);
impl EPDISHDMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPDISHDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPDISHDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTDT` reader - Reset Data Toggle"]
pub struct RSTDT_R(crate::FieldReader<bool, bool>);
impl RSTDT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSTDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline(always)]
    pub fn txine(&self) -> TXINE_R {
        TXINE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline(always)]
    pub fn rxoute(&self) -> RXOUTE_R {
        RXOUTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Underflow Interrupt"]
    #[inline(always)]
    pub fn underfe(&self) -> UNDERFE_R {
        UNDERFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Underflow Error Interrupt"]
    #[inline(always)]
    pub fn hbisoinerre(&self) -> HBISOINERRE_R {
        HBISOINERRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt"]
    #[inline(always)]
    pub fn hbisoflushe(&self) -> HBISOFLUSHE_R {
        HBISOFLUSHE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfe(&self) -> OVERFE_R {
        OVERFE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CRC Error Interrupt"]
    #[inline(always)]
    pub fn crcerre(&self) -> CRCERRE_R {
        CRCERRE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpackete(&self) -> SHORTPACKETE_R {
        SHORTPACKETE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MData Interrupt"]
    #[inline(always)]
    pub fn mdatae(&self) -> MDATAE_R {
        MDATAE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DataX Interrupt"]
    #[inline(always)]
    pub fn dataxe(&self) -> DATAXE_R {
        DATAXE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transaction Error Interrupt"]
    #[inline(always)]
    pub fn errortranse(&self) -> ERRORTRANSE_R {
        ERRORTRANSE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt"]
    #[inline(always)]
    pub fn nbusybke(&self) -> NBUSYBKE_R {
        NBUSYBKE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    pub fn killbk(&self) -> KILLBK_R {
        KILLBK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocon(&self) -> FIFOCON_R {
        FIFOCON_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request"]
    #[inline(always)]
    pub fn epdishdma(&self) -> EPDISHDMA_R {
        EPDISHDMA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Reset Data Toggle"]
    #[inline(always)]
    pub fn rstdt(&self) -> RSTDT_R {
        RSTDT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
#[doc = "Device Endpoint Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptimr_iso_mode](index.html) module"]
pub struct USBHS_DEVEPTIMR_ISO_MODE_SPEC;
impl crate::RegisterSpec for USBHS_DEVEPTIMR_ISO_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_deveptimr_iso_mode::R](R) reader structure"]
impl crate::Readable for USBHS_DEVEPTIMR_ISO_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBHS_DEVEPTIMR_ISO_MODE[%s]
to value 0"]
impl crate::Resettable for USBHS_DEVEPTIMR_ISO_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
