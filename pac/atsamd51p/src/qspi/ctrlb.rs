#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Serial Memory Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: SPI operating mode"]
    SPI = 0,
    #[doc = "1: Serial Memory operating mode"]
    MEMORY = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Serial Memory Mode"]
pub struct MODE_R(crate::FieldReader<bool, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::SPI,
            true => MODE_A::MEMORY,
        }
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        **self == MODE_A::SPI
    }
    #[doc = "Checks if the value of the field is `MEMORY`"]
    #[inline(always)]
    pub fn is_memory(&self) -> bool {
        **self == MODE_A::MEMORY
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Serial Memory Mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI operating mode"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(MODE_A::SPI)
    }
    #[doc = "Serial Memory operating mode"]
    #[inline(always)]
    pub fn memory(self) -> &'a mut W {
        self.variant(MODE_A::MEMORY)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `LOOPEN` reader - Local Loopback Enable"]
pub struct LOOPEN_R(crate::FieldReader<bool, bool>);
impl LOOPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOPEN` writer - Local Loopback Enable"]
pub struct LOOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `WDRBT` reader - Wait Data Read Before Transfer"]
pub struct WDRBT_R(crate::FieldReader<bool, bool>);
impl WDRBT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDRBT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDRBT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDRBT` writer - Wait Data Read Before Transfer"]
pub struct WDRBT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDRBT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SMEMREG` reader - Serial Memory reg"]
pub struct SMEMREG_R(crate::FieldReader<bool, bool>);
impl SMEMREG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMEMREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMEMREG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMEMREG` writer - Serial Memory reg"]
pub struct SMEMREG_W<'a> {
    w: &'a mut W,
}
impl<'a> SMEMREG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Chip Select Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSMODE_A {
    #[doc = "0: The chip select is deasserted if TD has not been reloaded before the end of the current transfer."]
    NORELOAD = 0,
    #[doc = "1: The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in TD has been transferred."]
    LASTXFER = 1,
    #[doc = "2: The chip select is deasserted systematically after each transfer."]
    SYSTEMATICALLY = 2,
}
impl From<CSMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CSMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CSMODE` reader - Chip Select Mode"]
pub struct CSMODE_R(crate::FieldReader<u8, CSMODE_A>);
impl CSMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CSMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSMODE_A> {
        match self.bits {
            0 => Some(CSMODE_A::NORELOAD),
            1 => Some(CSMODE_A::LASTXFER),
            2 => Some(CSMODE_A::SYSTEMATICALLY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORELOAD`"]
    #[inline(always)]
    pub fn is_noreload(&self) -> bool {
        **self == CSMODE_A::NORELOAD
    }
    #[doc = "Checks if the value of the field is `LASTXFER`"]
    #[inline(always)]
    pub fn is_lastxfer(&self) -> bool {
        **self == CSMODE_A::LASTXFER
    }
    #[doc = "Checks if the value of the field is `SYSTEMATICALLY`"]
    #[inline(always)]
    pub fn is_systematically(&self) -> bool {
        **self == CSMODE_A::SYSTEMATICALLY
    }
}
impl core::ops::Deref for CSMODE_R {
    type Target = crate::FieldReader<u8, CSMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSMODE` writer - Chip Select Mode"]
pub struct CSMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The chip select is deasserted if TD has not been reloaded before the end of the current transfer."]
    #[inline(always)]
    pub fn noreload(self) -> &'a mut W {
        self.variant(CSMODE_A::NORELOAD)
    }
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in TD has been transferred."]
    #[inline(always)]
    pub fn lastxfer(self) -> &'a mut W {
        self.variant(CSMODE_A::LASTXFER)
    }
    #[doc = "The chip select is deasserted systematically after each transfer."]
    #[inline(always)]
    pub fn systematically(self) -> &'a mut W {
        self.variant(CSMODE_A::SYSTEMATICALLY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Data Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATALEN_A {
    #[doc = "0: 8-bits transfer"]
    _8BITS = 0,
    #[doc = "1: 9 bits transfer"]
    _9BITS = 1,
    #[doc = "2: 10-bits transfer"]
    _10BITS = 2,
    #[doc = "3: 11-bits transfer"]
    _11BITS = 3,
    #[doc = "4: 12-bits transfer"]
    _12BITS = 4,
    #[doc = "5: 13-bits transfer"]
    _13BITS = 5,
    #[doc = "6: 14-bits transfer"]
    _14BITS = 6,
    #[doc = "7: 15-bits transfer"]
    _15BITS = 7,
    #[doc = "8: 16-bits transfer"]
    _16BITS = 8,
}
impl From<DATALEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DATALEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATALEN` reader - Data Length"]
pub struct DATALEN_R(crate::FieldReader<u8, DATALEN_A>);
impl DATALEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATALEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATALEN_A> {
        match self.bits {
            0 => Some(DATALEN_A::_8BITS),
            1 => Some(DATALEN_A::_9BITS),
            2 => Some(DATALEN_A::_10BITS),
            3 => Some(DATALEN_A::_11BITS),
            4 => Some(DATALEN_A::_12BITS),
            5 => Some(DATALEN_A::_13BITS),
            6 => Some(DATALEN_A::_14BITS),
            7 => Some(DATALEN_A::_15BITS),
            8 => Some(DATALEN_A::_16BITS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8BITS`"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        **self == DATALEN_A::_8BITS
    }
    #[doc = "Checks if the value of the field is `_9BITS`"]
    #[inline(always)]
    pub fn is_9bits(&self) -> bool {
        **self == DATALEN_A::_9BITS
    }
    #[doc = "Checks if the value of the field is `_10BITS`"]
    #[inline(always)]
    pub fn is_10bits(&self) -> bool {
        **self == DATALEN_A::_10BITS
    }
    #[doc = "Checks if the value of the field is `_11BITS`"]
    #[inline(always)]
    pub fn is_11bits(&self) -> bool {
        **self == DATALEN_A::_11BITS
    }
    #[doc = "Checks if the value of the field is `_12BITS`"]
    #[inline(always)]
    pub fn is_12bits(&self) -> bool {
        **self == DATALEN_A::_12BITS
    }
    #[doc = "Checks if the value of the field is `_13BITS`"]
    #[inline(always)]
    pub fn is_13bits(&self) -> bool {
        **self == DATALEN_A::_13BITS
    }
    #[doc = "Checks if the value of the field is `_14BITS`"]
    #[inline(always)]
    pub fn is_14bits(&self) -> bool {
        **self == DATALEN_A::_14BITS
    }
    #[doc = "Checks if the value of the field is `_15BITS`"]
    #[inline(always)]
    pub fn is_15bits(&self) -> bool {
        **self == DATALEN_A::_15BITS
    }
    #[doc = "Checks if the value of the field is `_16BITS`"]
    #[inline(always)]
    pub fn is_16bits(&self) -> bool {
        **self == DATALEN_A::_16BITS
    }
}
impl core::ops::Deref for DATALEN_R {
    type Target = crate::FieldReader<u8, DATALEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATALEN` writer - Data Length"]
pub struct DATALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATALEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8-bits transfer"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut W {
        self.variant(DATALEN_A::_8BITS)
    }
    #[doc = "9 bits transfer"]
    #[inline(always)]
    pub fn _9bits(self) -> &'a mut W {
        self.variant(DATALEN_A::_9BITS)
    }
    #[doc = "10-bits transfer"]
    #[inline(always)]
    pub fn _10bits(self) -> &'a mut W {
        self.variant(DATALEN_A::_10BITS)
    }
    #[doc = "11-bits transfer"]
    #[inline(always)]
    pub fn _11bits(self) -> &'a mut W {
        self.variant(DATALEN_A::_11BITS)
    }
    #[doc = "12-bits transfer"]
    #[inline(always)]
    pub fn _12bits(self) -> &'a mut W {
        self.variant(DATALEN_A::_12BITS)
    }
    #[doc = "13-bits transfer"]
    #[inline(always)]
    pub fn _13bits(self) -> &'a mut W {
        self.variant(DATALEN_A::_13BITS)
    }
    #[doc = "14-bits transfer"]
    #[inline(always)]
    pub fn _14bits(self) -> &'a mut W {
        self.variant(DATALEN_A::_14BITS)
    }
    #[doc = "15-bits transfer"]
    #[inline(always)]
    pub fn _15bits(self) -> &'a mut W {
        self.variant(DATALEN_A::_15BITS)
    }
    #[doc = "16-bits transfer"]
    #[inline(always)]
    pub fn _16bits(self) -> &'a mut W {
        self.variant(DATALEN_A::_16BITS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `DLYBCT` reader - Delay Between Consecutive Transfers"]
pub struct DLYBCT_R(crate::FieldReader<u8, u8>);
impl DLYBCT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLYBCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYBCT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYBCT` writer - Delay Between Consecutive Transfers"]
pub struct DLYBCT_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYBCT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DLYCS` reader - Minimum Inactive CS Delay"]
pub struct DLYCS_R(crate::FieldReader<u8, u8>);
impl DLYCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLYCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYCS` writer - Minimum Inactive CS Delay"]
pub struct DLYCS_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    pub fn loopen(&self) -> LOOPEN_R {
        LOOPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WDRBT_R {
        WDRBT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Serial Memory reg"]
    #[inline(always)]
    pub fn smemreg(&self) -> SMEMREG_R {
        SMEMREG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    pub fn csmode(&self) -> CSMODE_R {
        CSMODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Data Length"]
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&self) -> DLYBCT_R {
        DLYBCT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Minimum Inactive CS Delay"]
    #[inline(always)]
    pub fn dlycs(&self) -> DLYCS_R {
        DLYCS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    pub fn loopen(&mut self) -> LOOPEN_W {
        LOOPEN_W { w: self }
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&mut self) -> WDRBT_W {
        WDRBT_W { w: self }
    }
    #[doc = "Bit 3 - Serial Memory reg"]
    #[inline(always)]
    pub fn smemreg(&mut self) -> SMEMREG_W {
        SMEMREG_W { w: self }
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    pub fn csmode(&mut self) -> CSMODE_W {
        CSMODE_W { w: self }
    }
    #[doc = "Bits 8:11 - Data Length"]
    #[inline(always)]
    pub fn datalen(&mut self) -> DATALEN_W {
        DATALEN_W { w: self }
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&mut self) -> DLYBCT_W {
        DLYBCT_W { w: self }
    }
    #[doc = "Bits 24:31 - Minimum Inactive CS Delay"]
    #[inline(always)]
    pub fn dlycs(&mut self) -> DLYCS_W {
        DLYCS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
