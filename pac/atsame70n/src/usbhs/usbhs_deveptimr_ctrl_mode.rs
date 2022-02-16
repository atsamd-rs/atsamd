#[doc = "Register `USBHS_DEVEPTIMR_CTRL_MODE[%s]` reader"]
pub struct R(crate::R<USBHS_DEVEPTIMR_CTRL_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_DEVEPTIMR_CTRL_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_DEVEPTIMR_CTRL_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_DEVEPTIMR_CTRL_MODE_SPEC>) -> Self {
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
#[doc = "Field `RXSTPE` reader - Received SETUP Interrupt"]
pub struct RXSTPE_R(crate::FieldReader<bool, bool>);
impl RXSTPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXSTPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSTPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKOUTE` reader - NAKed OUT Interrupt"]
pub struct NAKOUTE_R(crate::FieldReader<bool, bool>);
impl NAKOUTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NAKOUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKOUTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKINE` reader - NAKed IN Interrupt"]
pub struct NAKINE_R(crate::FieldReader<bool, bool>);
impl NAKINE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NAKINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKINE_R {
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
#[doc = "Field `STALLEDE` reader - STALLed Interrupt"]
pub struct STALLEDE_R(crate::FieldReader<bool, bool>);
impl STALLEDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALLEDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALLEDE_R {
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
#[doc = "Field `NYETDIS` reader - NYET Token Disable"]
pub struct NYETDIS_R(crate::FieldReader<bool, bool>);
impl NYETDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NYETDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NYETDIS_R {
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
#[doc = "Field `STALLRQ` reader - STALL Request"]
pub struct STALLRQ_R(crate::FieldReader<bool, bool>);
impl STALLRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALLRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALLRQ_R {
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
    #[doc = "Bit 2 - Received SETUP Interrupt"]
    #[inline(always)]
    pub fn rxstpe(&self) -> RXSTPE_R {
        RXSTPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt"]
    #[inline(always)]
    pub fn nakoute(&self) -> NAKOUTE_R {
        NAKOUTE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt"]
    #[inline(always)]
    pub fn nakine(&self) -> NAKINE_R {
        NAKINE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfe(&self) -> OVERFE_R {
        OVERFE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - STALLed Interrupt"]
    #[inline(always)]
    pub fn stallede(&self) -> STALLEDE_R {
        STALLEDE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpackete(&self) -> SHORTPACKETE_R {
        SHORTPACKETE_R::new(((self.bits >> 7) & 0x01) != 0)
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
    #[doc = "Bit 17 - NYET Token Disable"]
    #[inline(always)]
    pub fn nyetdis(&self) -> NYETDIS_R {
        NYETDIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Reset Data Toggle"]
    #[inline(always)]
    pub fn rstdt(&self) -> RSTDT_R {
        RSTDT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - STALL Request"]
    #[inline(always)]
    pub fn stallrq(&self) -> STALLRQ_R {
        STALLRQ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
#[doc = "Device Endpoint Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptimr_ctrl_mode](index.html) module"]
pub struct USBHS_DEVEPTIMR_CTRL_MODE_SPEC;
impl crate::RegisterSpec for USBHS_DEVEPTIMR_CTRL_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_deveptimr_ctrl_mode::R](R) reader structure"]
impl crate::Readable for USBHS_DEVEPTIMR_CTRL_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBHS_DEVEPTIMR_CTRL_MODE[%s]
to value 0"]
impl crate::Resettable for USBHS_DEVEPTIMR_CTRL_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
