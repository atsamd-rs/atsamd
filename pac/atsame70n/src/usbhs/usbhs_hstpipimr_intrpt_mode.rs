#[doc = "Register `USBHS_HSTPIPIMR_INTRPT_MODE[%s]` reader"]
pub struct R(crate::R<USBHS_HSTPIPIMR_INTRPT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_HSTPIPIMR_INTRPT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_HSTPIPIMR_INTRPT_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_HSTPIPIMR_INTRPT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXINE` reader - Received IN Data Interrupt Enable"]
pub struct RXINE_R(crate::FieldReader<bool, bool>);
impl RXINE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXINE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOUTE` reader - Transmitted OUT Data Interrupt Enable"]
pub struct TXOUTE_R(crate::FieldReader<bool, bool>);
impl TXOUTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXOUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOUTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDERFIE` reader - Underflow Interrupt Enable"]
pub struct UNDERFIE_R(crate::FieldReader<bool, bool>);
impl UNDERFIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNDERFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNDERFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERRE` reader - Pipe Error Interrupt Enable"]
pub struct PERRE_R(crate::FieldReader<bool, bool>);
impl PERRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PERRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKEDE` reader - NAKed Interrupt Enable"]
pub struct NAKEDE_R(crate::FieldReader<bool, bool>);
impl NAKEDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NAKEDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKEDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERFIE` reader - Overflow Interrupt Enable"]
pub struct OVERFIE_R(crate::FieldReader<bool, bool>);
impl OVERFIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTALLDE` reader - Received STALLed Interrupt Enable"]
pub struct RXSTALLDE_R(crate::FieldReader<bool, bool>);
impl RXSTALLDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXSTALLDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSTALLDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHORTPACKETIE` reader - Short Packet Interrupt Enable"]
pub struct SHORTPACKETIE_R(crate::FieldReader<bool, bool>);
impl SHORTPACKETIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHORTPACKETIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHORTPACKETIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBUSYBKE` reader - Number of Busy Banks Interrupt Enable"]
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
#[doc = "Field `PDISHDMA` reader - Pipe Interrupts Disable HDMA Request Enable"]
pub struct PDISHDMA_R(crate::FieldReader<bool, bool>);
impl PDISHDMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDISHDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDISHDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PFREEZE` reader - Pipe Freeze"]
pub struct PFREEZE_R(crate::FieldReader<bool, bool>);
impl PFREEZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PFREEZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PFREEZE_R {
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
    #[doc = "Bit 0 - Received IN Data Interrupt Enable"]
    #[inline(always)]
    pub fn rxine(&self) -> RXINE_R {
        RXINE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Enable"]
    #[inline(always)]
    pub fn txoute(&self) -> TXOUTE_R {
        TXOUTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn underfie(&self) -> UNDERFIE_R {
        UNDERFIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Enable"]
    #[inline(always)]
    pub fn perre(&self) -> PERRE_R {
        PERRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAKed Interrupt Enable"]
    #[inline(always)]
    pub fn nakede(&self) -> NAKEDE_R {
        NAKEDE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overfie(&self) -> OVERFIE_R {
        OVERFIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Enable"]
    #[inline(always)]
    pub fn rxstallde(&self) -> RXSTALLDE_R {
        RXSTALLDE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    pub fn shortpacketie(&self) -> SHORTPACKETIE_R {
        SHORTPACKETIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Enable"]
    #[inline(always)]
    pub fn nbusybke(&self) -> NBUSYBKE_R {
        NBUSYBKE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocon(&self) -> FIFOCON_R {
        FIFOCON_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    pub fn pdishdma(&self) -> PDISHDMA_R {
        PDISHDMA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pipe Freeze"]
    #[inline(always)]
    pub fn pfreeze(&self) -> PFREEZE_R {
        PFREEZE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Reset Data Toggle"]
    #[inline(always)]
    pub fn rstdt(&self) -> RSTDT_R {
        RSTDT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
#[doc = "Host Pipe Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipimr_intrpt_mode](index.html) module"]
pub struct USBHS_HSTPIPIMR_INTRPT_MODE_SPEC;
impl crate::RegisterSpec for USBHS_HSTPIPIMR_INTRPT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_hstpipimr_intrpt_mode::R](R) reader structure"]
impl crate::Readable for USBHS_HSTPIPIMR_INTRPT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBHS_HSTPIPIMR_INTRPT_MODE[%s]
to value 0"]
impl crate::Resettable for USBHS_HSTPIPIMR_INTRPT_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
