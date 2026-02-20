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
#[doc = "Character Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHSIZE_A {
    #[doc = "0: 8 bits"]
    _8_BIT = 0,
    #[doc = "1: 9 bits"]
    _9_BIT = 1,
}
impl From<CHSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHSIZE` reader - Character Size"]
pub struct CHSIZE_R(crate::FieldReader<u8, CHSIZE_A>);
impl CHSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHSIZE_A> {
        match self.bits {
            0 => Some(CHSIZE_A::_8_BIT),
            1 => Some(CHSIZE_A::_9_BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        **self == CHSIZE_A::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_9_BIT`"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        **self == CHSIZE_A::_9_BIT
    }
}
impl core::ops::Deref for CHSIZE_R {
    type Target = crate::FieldReader<u8, CHSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSIZE` writer - Character Size"]
pub struct CHSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(CHSIZE_A::_8_BIT)
    }
    #[doc = "9 bits"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut W {
        self.variant(CHSIZE_A::_9_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `PLOADEN` reader - Data Preload Enable"]
pub struct PLOADEN_R(crate::FieldReader<bool, bool>);
impl PLOADEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLOADEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLOADEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLOADEN` writer - Data Preload Enable"]
pub struct PLOADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLOADEN_W<'a> {
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
#[doc = "Field `SSDE` reader - Slave Select Low Detect Enable"]
pub struct SSDE_R(crate::FieldReader<bool, bool>);
impl SSDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSDE` writer - Slave Select Low Detect Enable"]
pub struct SSDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `MSSEN` reader - Master Slave Select Enable"]
pub struct MSSEN_R(crate::FieldReader<bool, bool>);
impl MSSEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSSEN` writer - Master Slave Select Enable"]
pub struct MSSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Address Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AMODE_A {
    #[doc = "0: SPI Address mask"]
    MASK = 0,
    #[doc = "1: Two unique Addressess"]
    _2_ADDRESSES = 1,
    #[doc = "2: Address Range"]
    RANGE = 2,
}
impl From<AMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: AMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AMODE` reader - Address Mode"]
pub struct AMODE_R(crate::FieldReader<u8, AMODE_A>);
impl AMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AMODE_A> {
        match self.bits {
            0 => Some(AMODE_A::MASK),
            1 => Some(AMODE_A::_2_ADDRESSES),
            2 => Some(AMODE_A::RANGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == AMODE_A::MASK
    }
    #[doc = "Checks if the value of the field is `_2_ADDRESSES`"]
    #[inline(always)]
    pub fn is_2_addresses(&self) -> bool {
        **self == AMODE_A::_2_ADDRESSES
    }
    #[doc = "Checks if the value of the field is `RANGE`"]
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        **self == AMODE_A::RANGE
    }
}
impl core::ops::Deref for AMODE_R {
    type Target = crate::FieldReader<u8, AMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMODE` writer - Address Mode"]
pub struct AMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SPI Address mask"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(AMODE_A::MASK)
    }
    #[doc = "Two unique Addressess"]
    #[inline(always)]
    pub fn _2_addresses(self) -> &'a mut W {
        self.variant(AMODE_A::_2_ADDRESSES)
    }
    #[doc = "Address Range"]
    #[inline(always)]
    pub fn range(self) -> &'a mut W {
        self.variant(AMODE_A::RANGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `RXEN` reader - Receiver Enable"]
pub struct RXEN_R(crate::FieldReader<bool, bool>);
impl RXEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub struct RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&self) -> CHSIZE_R {
        CHSIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 6 - Data Preload Enable"]
    #[inline(always)]
    pub fn ploaden(&self) -> PLOADEN_R {
        PLOADEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Slave Select Low Detect Enable"]
    #[inline(always)]
    pub fn ssde(&self) -> SSDE_R {
        SSDE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Master Slave Select Enable"]
    #[inline(always)]
    pub fn mssen(&self) -> MSSEN_R {
        MSSEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&self) -> AMODE_R {
        AMODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&mut self) -> CHSIZE_W {
        CHSIZE_W { w: self }
    }
    #[doc = "Bit 6 - Data Preload Enable"]
    #[inline(always)]
    pub fn ploaden(&mut self) -> PLOADEN_W {
        PLOADEN_W { w: self }
    }
    #[doc = "Bit 9 - Slave Select Low Detect Enable"]
    #[inline(always)]
    pub fn ssde(&mut self) -> SSDE_W {
        SSDE_W { w: self }
    }
    #[doc = "Bit 13 - Master Slave Select Enable"]
    #[inline(always)]
    pub fn mssen(&mut self) -> MSSEN_W {
        MSSEN_W { w: self }
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&mut self) -> AMODE_W {
        AMODE_W { w: self }
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIM Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
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
