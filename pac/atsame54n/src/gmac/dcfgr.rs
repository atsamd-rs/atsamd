#[doc = "Register `DCFGR` reader"]
pub struct R(crate::R<DCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCFGR` writer"]
pub struct W(crate::W<DCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBLDO` reader - Fixed Burst Length for DMA Data Operations:"]
pub struct FBLDO_R(crate::FieldReader<u8, u8>);
impl FBLDO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FBLDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBLDO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBLDO` writer - Fixed Burst Length for DMA Data Operations:"]
pub struct FBLDO_W<'a> {
    w: &'a mut W,
}
impl<'a> FBLDO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `ESMA` reader - Endian Swap Mode Enable for Management Descriptor Accesses"]
pub struct ESMA_R(crate::FieldReader<bool, bool>);
impl ESMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESMA` writer - Endian Swap Mode Enable for Management Descriptor Accesses"]
pub struct ESMA_W<'a> {
    w: &'a mut W,
}
impl<'a> ESMA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `ESPA` reader - Endian Swap Mode Enable for Packet Data Accesses"]
pub struct ESPA_R(crate::FieldReader<bool, bool>);
impl ESPA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESPA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESPA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESPA` writer - Endian Swap Mode Enable for Packet Data Accesses"]
pub struct ESPA_W<'a> {
    w: &'a mut W,
}
impl<'a> ESPA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RXBMS` reader - Receiver Packet Buffer Memory Size Select"]
pub struct RXBMS_R(crate::FieldReader<u8, u8>);
impl RXBMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXBMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBMS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBMS` writer - Receiver Packet Buffer Memory Size Select"]
pub struct RXBMS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `TXPBMS` reader - Transmitter Packet Buffer Memory Size Select"]
pub struct TXPBMS_R(crate::FieldReader<bool, bool>);
impl TXPBMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXPBMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPBMS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPBMS` writer - Transmitter Packet Buffer Memory Size Select"]
pub struct TXPBMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPBMS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `TXCOEN` reader - Transmitter Checksum Generation Offload Enable"]
pub struct TXCOEN_R(crate::FieldReader<bool, bool>);
impl TXCOEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXCOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCOEN` writer - Transmitter Checksum Generation Offload Enable"]
pub struct TXCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCOEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `DRBS` reader - DMA Receive Buffer Size"]
pub struct DRBS_R(crate::FieldReader<u8, u8>);
impl DRBS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DRBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRBS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRBS` writer - DMA Receive Buffer Size"]
pub struct DRBS_W<'a> {
    w: &'a mut W,
}
impl<'a> DRBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DDRP` reader - DMA Discard Receive Packets"]
pub struct DDRP_R(crate::FieldReader<bool, bool>);
impl DDRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DDRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRP` writer - DMA Discard Receive Packets"]
pub struct DDRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Fixed Burst Length for DMA Data Operations:"]
    #[inline(always)]
    pub fn fbldo(&self) -> FBLDO_R {
        FBLDO_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Endian Swap Mode Enable for Management Descriptor Accesses"]
    #[inline(always)]
    pub fn esma(&self) -> ESMA_R {
        ESMA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endian Swap Mode Enable for Packet Data Accesses"]
    #[inline(always)]
    pub fn espa(&self) -> ESPA_R {
        ESPA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Receiver Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn rxbms(&self) -> RXBMS_R {
        RXBMS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Transmitter Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn txpbms(&self) -> TXPBMS_R {
        TXPBMS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmitter Checksum Generation Offload Enable"]
    #[inline(always)]
    pub fn txcoen(&self) -> TXCOEN_R {
        TXCOEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - DMA Receive Buffer Size"]
    #[inline(always)]
    pub fn drbs(&self) -> DRBS_R {
        DRBS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - DMA Discard Receive Packets"]
    #[inline(always)]
    pub fn ddrp(&self) -> DDRP_R {
        DDRP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Fixed Burst Length for DMA Data Operations:"]
    #[inline(always)]
    pub fn fbldo(&mut self) -> FBLDO_W {
        FBLDO_W { w: self }
    }
    #[doc = "Bit 6 - Endian Swap Mode Enable for Management Descriptor Accesses"]
    #[inline(always)]
    pub fn esma(&mut self) -> ESMA_W {
        ESMA_W { w: self }
    }
    #[doc = "Bit 7 - Endian Swap Mode Enable for Packet Data Accesses"]
    #[inline(always)]
    pub fn espa(&mut self) -> ESPA_W {
        ESPA_W { w: self }
    }
    #[doc = "Bits 8:9 - Receiver Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn rxbms(&mut self) -> RXBMS_W {
        RXBMS_W { w: self }
    }
    #[doc = "Bit 10 - Transmitter Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn txpbms(&mut self) -> TXPBMS_W {
        TXPBMS_W { w: self }
    }
    #[doc = "Bit 11 - Transmitter Checksum Generation Offload Enable"]
    #[inline(always)]
    pub fn txcoen(&mut self) -> TXCOEN_W {
        TXCOEN_W { w: self }
    }
    #[doc = "Bits 16:23 - DMA Receive Buffer Size"]
    #[inline(always)]
    pub fn drbs(&mut self) -> DRBS_W {
        DRBS_W { w: self }
    }
    #[doc = "Bit 24 - DMA Discard Receive Packets"]
    #[inline(always)]
    pub fn ddrp(&mut self) -> DDRP_W {
        DDRP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfgr](index.html) module"]
pub struct DCFGR_SPEC;
impl crate::RegisterSpec for DCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcfgr::R](R) reader structure"]
impl crate::Readable for DCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcfgr::W](W) writer structure"]
impl crate::Writable for DCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCFGR to value 0x0002_0704"]
impl crate::Resettable for DCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0704
    }
}
