#[doc = "Register `GMAC_DCFGR` reader"]
pub struct R(crate::R<GMAC_DCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_DCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_DCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_DCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_DCFGR` writer"]
pub struct W(crate::W<GMAC_DCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_DCFGR_SPEC>;
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
impl From<crate::W<GMAC_DCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_DCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fixed Burst Length for DMA Data Operations:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FBLDO_A {
    #[doc = "1: 00001: Always use SINGLE AHB bursts"]
    SINGLE = 1,
    #[doc = "4: 001xx: Attempt to use INCR4 AHB bursts (Default)"]
    INCR4 = 4,
    #[doc = "8: 01xxx: Attempt to use INCR8 AHB bursts"]
    INCR8 = 8,
    #[doc = "16: 1xxxx: Attempt to use INCR16 AHB bursts"]
    INCR16 = 16,
}
impl From<FBLDO_A> for u8 {
    #[inline(always)]
    fn from(variant: FBLDO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FBLDO` reader - Fixed Burst Length for DMA Data Operations:"]
pub struct FBLDO_R(crate::FieldReader<u8, FBLDO_A>);
impl FBLDO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FBLDO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FBLDO_A> {
        match self.bits {
            1 => Some(FBLDO_A::SINGLE),
            4 => Some(FBLDO_A::INCR4),
            8 => Some(FBLDO_A::INCR8),
            16 => Some(FBLDO_A::INCR16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == FBLDO_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `INCR4`"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        **self == FBLDO_A::INCR4
    }
    #[doc = "Checks if the value of the field is `INCR8`"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        **self == FBLDO_A::INCR8
    }
    #[doc = "Checks if the value of the field is `INCR16`"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        **self == FBLDO_A::INCR16
    }
}
impl core::ops::Deref for FBLDO_R {
    type Target = crate::FieldReader<u8, FBLDO_A>;
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
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FBLDO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "00001: Always use SINGLE AHB bursts"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(FBLDO_A::SINGLE)
    }
    #[doc = "001xx: Attempt to use INCR4 AHB bursts (Default)"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut W {
        self.variant(FBLDO_A::INCR4)
    }
    #[doc = "01xxx: Attempt to use INCR8 AHB bursts"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut W {
        self.variant(FBLDO_A::INCR8)
    }
    #[doc = "1xxxx: Attempt to use INCR16 AHB bursts"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut W {
        self.variant(FBLDO_A::INCR16)
    }
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
#[doc = "Receiver Packet Buffer Memory Size Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXBMS_A {
    #[doc = "0: 4/8 Kbyte Memory Size"]
    EIGHTH = 0,
    #[doc = "1: 4/4 Kbytes Memory Size"]
    QUARTER = 1,
    #[doc = "2: 4/2 Kbytes Memory Size"]
    HALF = 2,
    #[doc = "3: 4 Kbytes Memory Size"]
    FULL = 3,
}
impl From<RXBMS_A> for u8 {
    #[inline(always)]
    fn from(variant: RXBMS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RXBMS` reader - Receiver Packet Buffer Memory Size Select"]
pub struct RXBMS_R(crate::FieldReader<u8, RXBMS_A>);
impl RXBMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXBMS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXBMS_A {
        match self.bits {
            0 => RXBMS_A::EIGHTH,
            1 => RXBMS_A::QUARTER,
            2 => RXBMS_A::HALF,
            3 => RXBMS_A::FULL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EIGHTH`"]
    #[inline(always)]
    pub fn is_eighth(&self) -> bool {
        **self == RXBMS_A::EIGHTH
    }
    #[doc = "Checks if the value of the field is `QUARTER`"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        **self == RXBMS_A::QUARTER
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        **self == RXBMS_A::HALF
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        **self == RXBMS_A::FULL
    }
}
impl core::ops::Deref for RXBMS_R {
    type Target = crate::FieldReader<u8, RXBMS_A>;
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
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXBMS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "4/8 Kbyte Memory Size"]
    #[inline(always)]
    pub fn eighth(self) -> &'a mut W {
        self.variant(RXBMS_A::EIGHTH)
    }
    #[doc = "4/4 Kbytes Memory Size"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut W {
        self.variant(RXBMS_A::QUARTER)
    }
    #[doc = "4/2 Kbytes Memory Size"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(RXBMS_A::HALF)
    }
    #[doc = "4 Kbytes Memory Size"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(RXBMS_A::FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
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
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_dcfgr](index.html) module"]
pub struct GMAC_DCFGR_SPEC;
impl crate::RegisterSpec for GMAC_DCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_dcfgr::R](R) reader structure"]
impl crate::Readable for GMAC_DCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_dcfgr::W](W) writer structure"]
impl crate::Writable for GMAC_DCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_DCFGR to value 0"]
impl crate::Resettable for GMAC_DCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
