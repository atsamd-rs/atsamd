#[doc = "Reader of register TXCTRL"]
pub type R = crate::R<u32, super::TXCTRL>;
#[doc = "Writer for register TXCTRL"]
pub type W = crate::W<u32, super::TXCTRL>;
#[doc = "Register TXCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TXCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Serializer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SERMODE_A {
    #[doc = "0: Receive"]
    RX = 0,
    #[doc = "1: Transmit"]
    TX = 1,
    #[doc = "2: Receive one PDM data on each serial clock edge"]
    PDM2 = 2,
}
impl From<SERMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SERMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SERMODE`"]
pub type SERMODE_R = crate::R<u8, SERMODE_A>;
impl SERMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SERMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SERMODE_A::RX),
            1 => Val(SERMODE_A::TX),
            2 => Val(SERMODE_A::PDM2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == SERMODE_A::RX
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == SERMODE_A::TX
    }
    #[doc = "Checks if the value of the field is `PDM2`"]
    #[inline(always)]
    pub fn is_pdm2(&self) -> bool {
        *self == SERMODE_A::PDM2
    }
}
#[doc = "Write proxy for field `SERMODE`"]
pub struct SERMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SERMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SERMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Receive"]
    #[inline(always)]
    pub fn rx(self) -> &'a mut W {
        self.variant(SERMODE_A::RX)
    }
    #[doc = "Transmit"]
    #[inline(always)]
    pub fn tx(self) -> &'a mut W {
        self.variant(SERMODE_A::TX)
    }
    #[doc = "Receive one PDM data on each serial clock edge"]
    #[inline(always)]
    pub fn pdm2(self) -> &'a mut W {
        self.variant(SERMODE_A::PDM2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Line Default Line when Slot Disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXDEFAULT_A {
    #[doc = "0: Output Default Value is 0"]
    ZERO = 0,
    #[doc = "1: Output Default Value is 1"]
    ONE = 1,
    #[doc = "3: Output Default Value is high impedance"]
    HIZ = 3,
}
impl From<TXDEFAULT_A> for u8 {
    #[inline(always)]
    fn from(variant: TXDEFAULT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXDEFAULT`"]
pub type TXDEFAULT_R = crate::R<u8, TXDEFAULT_A>;
impl TXDEFAULT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXDEFAULT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXDEFAULT_A::ZERO),
            1 => Val(TXDEFAULT_A::ONE),
            3 => Val(TXDEFAULT_A::HIZ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == TXDEFAULT_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == TXDEFAULT_A::ONE
    }
    #[doc = "Checks if the value of the field is `HIZ`"]
    #[inline(always)]
    pub fn is_hiz(&self) -> bool {
        *self == TXDEFAULT_A::HIZ
    }
}
#[doc = "Write proxy for field `TXDEFAULT`"]
pub struct TXDEFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDEFAULT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDEFAULT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Output Default Value is 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(TXDEFAULT_A::ZERO)
    }
    #[doc = "Output Default Value is 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(TXDEFAULT_A::ONE)
    }
    #[doc = "Output Default Value is high impedance"]
    #[inline(always)]
    pub fn hiz(self) -> &'a mut W {
        self.variant(TXDEFAULT_A::HIZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Transmit Data when Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSAME_A {
    #[doc = "0: Zero data transmitted in case of underrun"]
    ZERO = 0,
    #[doc = "1: Last data transmitted in case of underrun"]
    SAME = 1,
}
impl From<TXSAME_A> for bool {
    #[inline(always)]
    fn from(variant: TXSAME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXSAME`"]
pub type TXSAME_R = crate::R<bool, TXSAME_A>;
impl TXSAME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSAME_A {
        match self.bits {
            false => TXSAME_A::ZERO,
            true => TXSAME_A::SAME,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == TXSAME_A::ZERO
    }
    #[doc = "Checks if the value of the field is `SAME`"]
    #[inline(always)]
    pub fn is_same(&self) -> bool {
        *self == TXSAME_A::SAME
    }
}
#[doc = "Write proxy for field `TXSAME`"]
pub struct TXSAME_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSAME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSAME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Zero data transmitted in case of underrun"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(TXSAME_A::ZERO)
    }
    #[doc = "Last data transmitted in case of underrun"]
    #[inline(always)]
    pub fn same(self) -> &'a mut W {
        self.variant(TXSAME_A::SAME)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Clock Unit Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSEL_A {
    #[doc = "0: Use Clock Unit 0"]
    CLK0 = 0,
    #[doc = "1: Use Clock Unit 1"]
    CLK1 = 1,
}
impl From<CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<bool, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            false => CLKSEL_A::CLK0,
            true => CLKSEL_A::CLK1,
        }
    }
    #[doc = "Checks if the value of the field is `CLK0`"]
    #[inline(always)]
    pub fn is_clk0(&self) -> bool {
        *self == CLKSEL_A::CLK0
    }
    #[doc = "Checks if the value of the field is `CLK1`"]
    #[inline(always)]
    pub fn is_clk1(&self) -> bool {
        *self == CLKSEL_A::CLK1
    }
}
#[doc = "Write proxy for field `CLKSEL`"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use Clock Unit 0"]
    #[inline(always)]
    pub fn clk0(self) -> &'a mut W {
        self.variant(CLKSEL_A::CLK0)
    }
    #[doc = "Use Clock Unit 1"]
    #[inline(always)]
    pub fn clk1(self) -> &'a mut W {
        self.variant(CLKSEL_A::CLK1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Data Slot Formatting Adjust\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOTADJ_A {
    #[doc = "0: Data is right adjusted in slot"]
    RIGHT = 0,
    #[doc = "1: Data is left adjusted in slot"]
    LEFT = 1,
}
impl From<SLOTADJ_A> for bool {
    #[inline(always)]
    fn from(variant: SLOTADJ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLOTADJ`"]
pub type SLOTADJ_R = crate::R<bool, SLOTADJ_A>;
impl SLOTADJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOTADJ_A {
        match self.bits {
            false => SLOTADJ_A::RIGHT,
            true => SLOTADJ_A::LEFT,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == SLOTADJ_A::RIGHT
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == SLOTADJ_A::LEFT
    }
}
#[doc = "Write proxy for field `SLOTADJ`"]
pub struct SLOTADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTADJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOTADJ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data is right adjusted in slot"]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(SLOTADJ_A::RIGHT)
    }
    #[doc = "Data is left adjusted in slot"]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(SLOTADJ_A::LEFT)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Data Word Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATASIZE_A {
    #[doc = "0: 32 bits"]
    _32 = 0,
    #[doc = "1: 24 bits"]
    _24 = 1,
    #[doc = "2: 20 bits"]
    _20 = 2,
    #[doc = "3: 18 bits"]
    _18 = 3,
    #[doc = "4: 16 bits"]
    _16 = 4,
    #[doc = "5: 16 bits compact stereo"]
    _16C = 5,
    #[doc = "6: 8 bits"]
    _8 = 6,
    #[doc = "7: 8 bits compact stereo"]
    _8C = 7,
}
impl From<DATASIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DATASIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DATASIZE`"]
pub type DATASIZE_R = crate::R<u8, DATASIZE_A>;
impl DATASIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATASIZE_A {
        match self.bits {
            0 => DATASIZE_A::_32,
            1 => DATASIZE_A::_24,
            2 => DATASIZE_A::_20,
            3 => DATASIZE_A::_18,
            4 => DATASIZE_A::_16,
            5 => DATASIZE_A::_16C,
            6 => DATASIZE_A::_8,
            7 => DATASIZE_A::_8C,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == DATASIZE_A::_32
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == DATASIZE_A::_24
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == DATASIZE_A::_20
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        *self == DATASIZE_A::_18
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == DATASIZE_A::_16
    }
    #[doc = "Checks if the value of the field is `_16C`"]
    #[inline(always)]
    pub fn is_16c(&self) -> bool {
        *self == DATASIZE_A::_16C
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == DATASIZE_A::_8
    }
    #[doc = "Checks if the value of the field is `_8C`"]
    #[inline(always)]
    pub fn is_8c(&self) -> bool {
        *self == DATASIZE_A::_8C
    }
}
#[doc = "Write proxy for field `DATASIZE`"]
pub struct DATASIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATASIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATASIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(DATASIZE_A::_32)
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(DATASIZE_A::_24)
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut W {
        self.variant(DATASIZE_A::_20)
    }
    #[doc = "18 bits"]
    #[inline(always)]
    pub fn _18(self) -> &'a mut W {
        self.variant(DATASIZE_A::_18)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(DATASIZE_A::_16)
    }
    #[doc = "16 bits compact stereo"]
    #[inline(always)]
    pub fn _16c(self) -> &'a mut W {
        self.variant(DATASIZE_A::_16C)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(DATASIZE_A::_8)
    }
    #[doc = "8 bits compact stereo"]
    #[inline(always)]
    pub fn _8c(self) -> &'a mut W {
        self.variant(DATASIZE_A::_8C)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Data Word Formatting Adjust\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORDADJ_A {
    #[doc = "0: Data is right adjusted in word"]
    RIGHT = 0,
    #[doc = "1: Data is left adjusted in word"]
    LEFT = 1,
}
impl From<WORDADJ_A> for bool {
    #[inline(always)]
    fn from(variant: WORDADJ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WORDADJ`"]
pub type WORDADJ_R = crate::R<bool, WORDADJ_A>;
impl WORDADJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WORDADJ_A {
        match self.bits {
            false => WORDADJ_A::RIGHT,
            true => WORDADJ_A::LEFT,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == WORDADJ_A::RIGHT
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == WORDADJ_A::LEFT
    }
}
#[doc = "Write proxy for field `WORDADJ`"]
pub struct WORDADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> WORDADJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WORDADJ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data is right adjusted in word"]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(WORDADJ_A::RIGHT)
    }
    #[doc = "Data is left adjusted in word"]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(WORDADJ_A::LEFT)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Data Formatting Bit Extension\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTEND_A {
    #[doc = "0: Extend with zeroes"]
    ZERO = 0,
    #[doc = "1: Extend with ones"]
    ONE = 1,
    #[doc = "2: Extend with Most Significant Bit"]
    MSBIT = 2,
    #[doc = "3: Extend with Least Significant Bit"]
    LSBIT = 3,
}
impl From<EXTEND_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTEND_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTEND`"]
pub type EXTEND_R = crate::R<u8, EXTEND_A>;
impl EXTEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTEND_A {
        match self.bits {
            0 => EXTEND_A::ZERO,
            1 => EXTEND_A::ONE,
            2 => EXTEND_A::MSBIT,
            3 => EXTEND_A::LSBIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == EXTEND_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == EXTEND_A::ONE
    }
    #[doc = "Checks if the value of the field is `MSBIT`"]
    #[inline(always)]
    pub fn is_msbit(&self) -> bool {
        *self == EXTEND_A::MSBIT
    }
    #[doc = "Checks if the value of the field is `LSBIT`"]
    #[inline(always)]
    pub fn is_lsbit(&self) -> bool {
        *self == EXTEND_A::LSBIT
    }
}
#[doc = "Write proxy for field `EXTEND`"]
pub struct EXTEND_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEND_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Extend with zeroes"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(EXTEND_A::ZERO)
    }
    #[doc = "Extend with ones"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(EXTEND_A::ONE)
    }
    #[doc = "Extend with Most Significant Bit"]
    #[inline(always)]
    pub fn msbit(self) -> &'a mut W {
        self.variant(EXTEND_A::MSBIT)
    }
    #[doc = "Extend with Least Significant Bit"]
    #[inline(always)]
    pub fn lsbit(self) -> &'a mut W {
        self.variant(EXTEND_A::LSBIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Data Formatting Bit Reverse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITREV_A {
    #[doc = "0: Transfer Data Most Significant Bit (MSB) first (default for I2S protocol)"]
    MSBIT = 0,
    #[doc = "1: Transfer Data Least Significant Bit (LSB) first"]
    LSBIT = 1,
}
impl From<BITREV_A> for bool {
    #[inline(always)]
    fn from(variant: BITREV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BITREV`"]
pub type BITREV_R = crate::R<bool, BITREV_A>;
impl BITREV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BITREV_A {
        match self.bits {
            false => BITREV_A::MSBIT,
            true => BITREV_A::LSBIT,
        }
    }
    #[doc = "Checks if the value of the field is `MSBIT`"]
    #[inline(always)]
    pub fn is_msbit(&self) -> bool {
        *self == BITREV_A::MSBIT
    }
    #[doc = "Checks if the value of the field is `LSBIT`"]
    #[inline(always)]
    pub fn is_lsbit(&self) -> bool {
        *self == BITREV_A::LSBIT
    }
}
#[doc = "Write proxy for field `BITREV`"]
pub struct BITREV_W<'a> {
    w: &'a mut W,
}
impl<'a> BITREV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BITREV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transfer Data Most Significant Bit (MSB) first (default for I2S protocol)"]
    #[inline(always)]
    pub fn msbit(self) -> &'a mut W {
        self.variant(BITREV_A::MSBIT)
    }
    #[doc = "Transfer Data Least Significant Bit (LSB) first"]
    #[inline(always)]
    pub fn lsbit(self) -> &'a mut W {
        self.variant(BITREV_A::LSBIT)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SLOTDIS0`"]
pub type SLOTDIS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOTDIS0`"]
pub struct SLOTDIS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTDIS0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SLOTDIS1`"]
pub type SLOTDIS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOTDIS1`"]
pub struct SLOTDIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTDIS1_W<'a> {
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
#[doc = "Reader of field `SLOTDIS2`"]
pub type SLOTDIS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOTDIS2`"]
pub struct SLOTDIS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTDIS2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `SLOTDIS3`"]
pub type SLOTDIS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOTDIS3`"]
pub struct SLOTDIS3_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTDIS3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `SLOTDIS4`"]
pub type SLOTDIS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOTDIS4`"]
pub struct SLOTDIS4_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTDIS4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SLOTDIS5`"]
pub type SLOTDIS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOTDIS5`"]
pub struct SLOTDIS5_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTDIS5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SLOTDIS6`"]
pub type SLOTDIS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOTDIS6`"]
pub struct SLOTDIS6_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTDIS6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SLOTDIS7`"]
pub type SLOTDIS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOTDIS7`"]
pub struct SLOTDIS7_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTDIS7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Mono Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONO_A {
    #[doc = "0: Normal mode"]
    STEREO = 0,
    #[doc = "1: Left channel data is duplicated to right channel"]
    MONO = 1,
}
impl From<MONO_A> for bool {
    #[inline(always)]
    fn from(variant: MONO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MONO`"]
pub type MONO_R = crate::R<bool, MONO_A>;
impl MONO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONO_A {
        match self.bits {
            false => MONO_A::STEREO,
            true => MONO_A::MONO,
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == MONO_A::STEREO
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == MONO_A::MONO
    }
}
#[doc = "Write proxy for field `MONO`"]
pub struct MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(MONO_A::STEREO)
    }
    #[doc = "Left channel data is duplicated to right channel"]
    #[inline(always)]
    pub fn mono(self) -> &'a mut W {
        self.variant(MONO_A::MONO)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Single or Multiple DMA Channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: Single DMA channel"]
    SINGLE = 0,
    #[doc = "1: One DMA channel per data channel"]
    MULTIPLE = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, DMA_A>;
impl DMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::SINGLE,
            true => DMA_A::MULTIPLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DMA_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == DMA_A::MULTIPLE
    }
}
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single DMA channel"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DMA_A::SINGLE)
    }
    #[doc = "One DMA channel per data channel"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut W {
        self.variant(DMA_A::MULTIPLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Serializer Mode"]
    #[inline(always)]
    pub fn sermode(&self) -> SERMODE_R {
        SERMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Line Default Line when Slot Disabled"]
    #[inline(always)]
    pub fn txdefault(&self) -> TXDEFAULT_R {
        TXDEFAULT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Transmit Data when Underrun"]
    #[inline(always)]
    pub fn txsame(&self) -> TXSAME_R {
        TXSAME_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Unit Selection"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Data Slot Formatting Adjust"]
    #[inline(always)]
    pub fn slotadj(&self) -> SLOTADJ_R {
        SLOTADJ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Data Word Size"]
    #[inline(always)]
    pub fn datasize(&self) -> DATASIZE_R {
        DATASIZE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Data Word Formatting Adjust"]
    #[inline(always)]
    pub fn wordadj(&self) -> WORDADJ_R {
        WORDADJ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Data Formatting Bit Extension"]
    #[inline(always)]
    pub fn extend(&self) -> EXTEND_R {
        EXTEND_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Data Formatting Bit Reverse"]
    #[inline(always)]
    pub fn bitrev(&self) -> BITREV_R {
        BITREV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Slot 0 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis0(&self) -> SLOTDIS0_R {
        SLOTDIS0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Slot 1 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis1(&self) -> SLOTDIS1_R {
        SLOTDIS1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Slot 2 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis2(&self) -> SLOTDIS2_R {
        SLOTDIS2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Slot 3 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis3(&self) -> SLOTDIS3_R {
        SLOTDIS3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Slot 4 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis4(&self) -> SLOTDIS4_R {
        SLOTDIS4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Slot 5 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis5(&self) -> SLOTDIS5_R {
        SLOTDIS5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Slot 6 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis6(&self) -> SLOTDIS6_R {
        SLOTDIS6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Slot 7 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis7(&self) -> SLOTDIS7_R {
        SLOTDIS7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Mono Mode"]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Single or Multiple DMA Channels"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Serializer Mode"]
    #[inline(always)]
    pub fn sermode(&mut self) -> SERMODE_W {
        SERMODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Line Default Line when Slot Disabled"]
    #[inline(always)]
    pub fn txdefault(&mut self) -> TXDEFAULT_W {
        TXDEFAULT_W { w: self }
    }
    #[doc = "Bit 4 - Transmit Data when Underrun"]
    #[inline(always)]
    pub fn txsame(&mut self) -> TXSAME_W {
        TXSAME_W { w: self }
    }
    #[doc = "Bit 5 - Clock Unit Selection"]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bit 7 - Data Slot Formatting Adjust"]
    #[inline(always)]
    pub fn slotadj(&mut self) -> SLOTADJ_W {
        SLOTADJ_W { w: self }
    }
    #[doc = "Bits 8:10 - Data Word Size"]
    #[inline(always)]
    pub fn datasize(&mut self) -> DATASIZE_W {
        DATASIZE_W { w: self }
    }
    #[doc = "Bit 12 - Data Word Formatting Adjust"]
    #[inline(always)]
    pub fn wordadj(&mut self) -> WORDADJ_W {
        WORDADJ_W { w: self }
    }
    #[doc = "Bits 13:14 - Data Formatting Bit Extension"]
    #[inline(always)]
    pub fn extend(&mut self) -> EXTEND_W {
        EXTEND_W { w: self }
    }
    #[doc = "Bit 15 - Data Formatting Bit Reverse"]
    #[inline(always)]
    pub fn bitrev(&mut self) -> BITREV_W {
        BITREV_W { w: self }
    }
    #[doc = "Bit 16 - Slot 0 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis0(&mut self) -> SLOTDIS0_W {
        SLOTDIS0_W { w: self }
    }
    #[doc = "Bit 17 - Slot 1 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis1(&mut self) -> SLOTDIS1_W {
        SLOTDIS1_W { w: self }
    }
    #[doc = "Bit 18 - Slot 2 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis2(&mut self) -> SLOTDIS2_W {
        SLOTDIS2_W { w: self }
    }
    #[doc = "Bit 19 - Slot 3 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis3(&mut self) -> SLOTDIS3_W {
        SLOTDIS3_W { w: self }
    }
    #[doc = "Bit 20 - Slot 4 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis4(&mut self) -> SLOTDIS4_W {
        SLOTDIS4_W { w: self }
    }
    #[doc = "Bit 21 - Slot 5 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis5(&mut self) -> SLOTDIS5_W {
        SLOTDIS5_W { w: self }
    }
    #[doc = "Bit 22 - Slot 6 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis6(&mut self) -> SLOTDIS6_W {
        SLOTDIS6_W { w: self }
    }
    #[doc = "Bit 23 - Slot 7 Disabled for this Serializer"]
    #[inline(always)]
    pub fn slotdis7(&mut self) -> SLOTDIS7_W {
        SLOTDIS7_W { w: self }
    }
    #[doc = "Bit 24 - Mono Mode"]
    #[inline(always)]
    pub fn mono(&mut self) -> MONO_W {
        MONO_W { w: self }
    }
    #[doc = "Bit 25 - Single or Multiple DMA Channels"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
}
