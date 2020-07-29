#[doc = "Reader of register CTRLB"]
pub type R = crate::R<u32, super::CTRLB>;
#[doc = "Writer for register CTRLB"]
pub type W = crate::W<u32, super::CTRLB>;
#[doc = "Register CTRLB `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `CHSIZE`"]
pub type CHSIZE_R = crate::R<u8, CHSIZE_A>;
impl CHSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CHSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHSIZE_A::_8_BIT),
            1 => Val(CHSIZE_A::_9_BIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == CHSIZE_A::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_9_BIT`"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == CHSIZE_A::_9_BIT
    }
}
#[doc = "Write proxy for field `CHSIZE`"]
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
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `PLOADEN`"]
pub type PLOADEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLOADEN`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SSDE`"]
pub type SSDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSDE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `MSSEN`"]
pub type MSSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSSEN`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Address Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AMODE_A {
    #[doc = "0: SPI Address mask "]
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
#[doc = "Reader of field `AMODE`"]
pub type AMODE_R = crate::R<u8, AMODE_A>;
impl AMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AMODE_A::MASK),
            1 => Val(AMODE_A::_2_ADDRESSES),
            2 => Val(AMODE_A::RANGE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == AMODE_A::MASK
    }
    #[doc = "Checks if the value of the field is `_2_ADDRESSES`"]
    #[inline(always)]
    pub fn is_2_addresses(&self) -> bool {
        *self == AMODE_A::_2_ADDRESSES
    }
    #[doc = "Checks if the value of the field is `RANGE`"]
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        *self == AMODE_A::RANGE
    }
}
#[doc = "Write proxy for field `AMODE`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `RXEN`"]
pub type RXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXEN`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
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
}
