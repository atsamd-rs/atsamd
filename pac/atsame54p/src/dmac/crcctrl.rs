#[doc = "Register `CRCCTRL` reader"]
pub struct R(crate::R<CRCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCCTRL` writer"]
pub struct W(crate::W<CRCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCCTRL_SPEC>;
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
impl From<crate::W<CRCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CRC Beat Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRCBEATSIZE_A {
    #[doc = "0: 8-bit bus transfer"]
    BYTE = 0,
    #[doc = "1: 16-bit bus transfer"]
    HWORD = 1,
    #[doc = "2: 32-bit bus transfer"]
    WORD = 2,
}
impl From<CRCBEATSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCBEATSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRCBEATSIZE` reader - CRC Beat Size"]
pub struct CRCBEATSIZE_R(crate::FieldReader<u8, CRCBEATSIZE_A>);
impl CRCBEATSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CRCBEATSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRCBEATSIZE_A> {
        match self.bits {
            0 => Some(CRCBEATSIZE_A::BYTE),
            1 => Some(CRCBEATSIZE_A::HWORD),
            2 => Some(CRCBEATSIZE_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        **self == CRCBEATSIZE_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HWORD`"]
    #[inline(always)]
    pub fn is_hword(&self) -> bool {
        **self == CRCBEATSIZE_A::HWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        **self == CRCBEATSIZE_A::WORD
    }
}
impl core::ops::Deref for CRCBEATSIZE_R {
    type Target = crate::FieldReader<u8, CRCBEATSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCBEATSIZE` writer - CRC Beat Size"]
pub struct CRCBEATSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCBEATSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCBEATSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8-bit bus transfer"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(CRCBEATSIZE_A::BYTE)
    }
    #[doc = "16-bit bus transfer"]
    #[inline(always)]
    pub fn hword(self) -> &'a mut W {
        self.variant(CRCBEATSIZE_A::HWORD)
    }
    #[doc = "32-bit bus transfer"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(CRCBEATSIZE_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u16 & 0x03);
        self.w
    }
}
#[doc = "CRC Polynomial Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRCPOLY_A {
    #[doc = "0: CRC-16 (CRC-CCITT)"]
    CRC16 = 0,
    #[doc = "1: CRC32 (IEEE 802.3)"]
    CRC32 = 1,
}
impl From<CRCPOLY_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCPOLY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRCPOLY` reader - CRC Polynomial Type"]
pub struct CRCPOLY_R(crate::FieldReader<u8, CRCPOLY_A>);
impl CRCPOLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CRCPOLY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRCPOLY_A> {
        match self.bits {
            0 => Some(CRCPOLY_A::CRC16),
            1 => Some(CRCPOLY_A::CRC32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CRC16`"]
    #[inline(always)]
    pub fn is_crc16(&self) -> bool {
        **self == CRCPOLY_A::CRC16
    }
    #[doc = "Checks if the value of the field is `CRC32`"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        **self == CRCPOLY_A::CRC32
    }
}
impl core::ops::Deref for CRCPOLY_R {
    type Target = crate::FieldReader<u8, CRCPOLY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCPOLY` writer - CRC Polynomial Type"]
pub struct CRCPOLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCPOLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCPOLY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CRC-16 (CRC-CCITT)"]
    #[inline(always)]
    pub fn crc16(self) -> &'a mut W {
        self.variant(CRCPOLY_A::CRC16)
    }
    #[doc = "CRC32 (IEEE 802.3)"]
    #[inline(always)]
    pub fn crc32(self) -> &'a mut W {
        self.variant(CRCPOLY_A::CRC32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u16 & 0x03) << 2);
        self.w
    }
}
#[doc = "CRC Input Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRCSRC_A {
    #[doc = "0: CRC Disabled"]
    DISABLE = 0,
    #[doc = "1: I/O interface"]
    IO = 1,
}
impl From<CRCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRCSRC` reader - CRC Input Source"]
pub struct CRCSRC_R(crate::FieldReader<u8, CRCSRC_A>);
impl CRCSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CRCSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRCSRC_A> {
        match self.bits {
            0 => Some(CRCSRC_A::DISABLE),
            1 => Some(CRCSRC_A::IO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CRCSRC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        **self == CRCSRC_A::IO
    }
}
impl core::ops::Deref for CRCSRC_R {
    type Target = crate::FieldReader<u8, CRCSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCSRC` writer - CRC Input Source"]
pub struct CRCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CRC Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRCSRC_A::DISABLE)
    }
    #[doc = "I/O interface"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(CRCSRC_A::IO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u16 & 0x3f) << 8);
        self.w
    }
}
#[doc = "CRC Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRCMODE_A {
    #[doc = "0: Default operating mode"]
    DEFAULT = 0,
    #[doc = "2: Memory CRC monitor operating mode"]
    CRCMON = 2,
    #[doc = "3: Memory CRC generation operating mode"]
    CRCGEN = 3,
}
impl From<CRCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRCMODE` reader - CRC Operating Mode"]
pub struct CRCMODE_R(crate::FieldReader<u8, CRCMODE_A>);
impl CRCMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CRCMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRCMODE_A> {
        match self.bits {
            0 => Some(CRCMODE_A::DEFAULT),
            2 => Some(CRCMODE_A::CRCMON),
            3 => Some(CRCMODE_A::CRCGEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        **self == CRCMODE_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `CRCMON`"]
    #[inline(always)]
    pub fn is_crcmon(&self) -> bool {
        **self == CRCMODE_A::CRCMON
    }
    #[doc = "Checks if the value of the field is `CRCGEN`"]
    #[inline(always)]
    pub fn is_crcgen(&self) -> bool {
        **self == CRCMODE_A::CRCGEN
    }
}
impl core::ops::Deref for CRCMODE_R {
    type Target = crate::FieldReader<u8, CRCMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCMODE` writer - CRC Operating Mode"]
pub struct CRCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Default operating mode"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(CRCMODE_A::DEFAULT)
    }
    #[doc = "Memory CRC monitor operating mode"]
    #[inline(always)]
    pub fn crcmon(self) -> &'a mut W {
        self.variant(CRCMODE_A::CRCMON)
    }
    #[doc = "Memory CRC generation operating mode"]
    #[inline(always)]
    pub fn crcgen(self) -> &'a mut W {
        self.variant(CRCMODE_A::CRCGEN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u16 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CRC Beat Size"]
    #[inline(always)]
    pub fn crcbeatsize(&self) -> CRCBEATSIZE_R {
        CRCBEATSIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - CRC Polynomial Type"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 8:13 - CRC Input Source"]
    #[inline(always)]
    pub fn crcsrc(&self) -> CRCSRC_R {
        CRCSRC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - CRC Operating Mode"]
    #[inline(always)]
    pub fn crcmode(&self) -> CRCMODE_R {
        CRCMODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC Beat Size"]
    #[inline(always)]
    pub fn crcbeatsize(&mut self) -> CRCBEATSIZE_W {
        CRCBEATSIZE_W { w: self }
    }
    #[doc = "Bits 2:3 - CRC Polynomial Type"]
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CRCPOLY_W {
        CRCPOLY_W { w: self }
    }
    #[doc = "Bits 8:13 - CRC Input Source"]
    #[inline(always)]
    pub fn crcsrc(&mut self) -> CRCSRC_W {
        CRCSRC_W { w: self }
    }
    #[doc = "Bits 14:15 - CRC Operating Mode"]
    #[inline(always)]
    pub fn crcmode(&mut self) -> CRCMODE_W {
        CRCMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcctrl](index.html) module"]
pub struct CRCCTRL_SPEC;
impl crate::RegisterSpec for CRCCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crcctrl::R](R) reader structure"]
impl crate::Readable for CRCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcctrl::W](W) writer structure"]
impl crate::Writable for CRCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCCTRL to value 0"]
impl crate::Resettable for CRCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
