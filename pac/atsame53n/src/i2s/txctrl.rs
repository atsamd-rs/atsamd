#[doc = "Register `TXCTRL` reader"]
pub struct R(crate::R<TXCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCTRL` writer"]
pub struct W(crate::W<TXCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCTRL_SPEC>;
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
impl From<crate::W<TXCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCTRL_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `SERMODE` reader - Serializer Mode"]
pub struct SERMODE_R(crate::FieldReader<u8, SERMODE_A>);
impl SERMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SERMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SERMODE_A> {
        match self.bits {
            0 => Some(SERMODE_A::RX),
            1 => Some(SERMODE_A::TX),
            2 => Some(SERMODE_A::PDM2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        **self == SERMODE_A::RX
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        **self == SERMODE_A::TX
    }
    #[doc = "Checks if the value of the field is `PDM2`"]
    #[inline(always)]
    pub fn is_pdm2(&self) -> bool {
        **self == SERMODE_A::PDM2
    }
}
impl core::ops::Deref for SERMODE_R {
    type Target = crate::FieldReader<u8, SERMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERMODE` writer - Serializer Mode"]
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
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
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
#[doc = "Field `TXDEFAULT` reader - Line Default Line when Slot Disabled"]
pub struct TXDEFAULT_R(crate::FieldReader<u8, TXDEFAULT_A>);
impl TXDEFAULT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXDEFAULT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXDEFAULT_A> {
        match self.bits {
            0 => Some(TXDEFAULT_A::ZERO),
            1 => Some(TXDEFAULT_A::ONE),
            3 => Some(TXDEFAULT_A::HIZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == TXDEFAULT_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == TXDEFAULT_A::ONE
    }
    #[doc = "Checks if the value of the field is `HIZ`"]
    #[inline(always)]
    pub fn is_hiz(&self) -> bool {
        **self == TXDEFAULT_A::HIZ
    }
}
impl core::ops::Deref for TXDEFAULT_R {
    type Target = crate::FieldReader<u8, TXDEFAULT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDEFAULT` writer - Line Default Line when Slot Disabled"]
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
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
#[doc = "Field `TXSAME` reader - Transmit Data when Underrun"]
pub struct TXSAME_R(crate::FieldReader<bool, TXSAME_A>);
impl TXSAME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXSAME_R(crate::FieldReader::new(bits))
    }
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
        **self == TXSAME_A::ZERO
    }
    #[doc = "Checks if the value of the field is `SAME`"]
    #[inline(always)]
    pub fn is_same(&self) -> bool {
        **self == TXSAME_A::SAME
    }
}
impl core::ops::Deref for TXSAME_R {
    type Target = crate::FieldReader<bool, TXSAME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSAME` writer - Transmit Data when Underrun"]
pub struct TXSAME_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSAME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSAME_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
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
#[doc = "Field `CLKSEL` reader - Clock Unit Selection"]
pub struct CLKSEL_R(crate::FieldReader<bool, CLKSEL_A>);
impl CLKSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKSEL_R(crate::FieldReader::new(bits))
    }
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
        **self == CLKSEL_A::CLK0
    }
    #[doc = "Checks if the value of the field is `CLK1`"]
    #[inline(always)]
    pub fn is_clk1(&self) -> bool {
        **self == CLKSEL_A::CLK1
    }
}
impl core::ops::Deref for CLKSEL_R {
    type Target = crate::FieldReader<bool, CLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSEL` writer - Clock Unit Selection"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
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
#[doc = "Field `SLOTADJ` reader - Data Slot Formatting Adjust"]
pub struct SLOTADJ_R(crate::FieldReader<bool, SLOTADJ_A>);
impl SLOTADJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLOTADJ_R(crate::FieldReader::new(bits))
    }
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
        **self == SLOTADJ_A::RIGHT
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        **self == SLOTADJ_A::LEFT
    }
}
impl core::ops::Deref for SLOTADJ_R {
    type Target = crate::FieldReader<bool, SLOTADJ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOTADJ` writer - Data Slot Formatting Adjust"]
pub struct SLOTADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTADJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOTADJ_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
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
#[doc = "Field `DATASIZE` reader - Data Word Size"]
pub struct DATASIZE_R(crate::FieldReader<u8, DATASIZE_A>);
impl DATASIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATASIZE_R(crate::FieldReader::new(bits))
    }
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
        **self == DATASIZE_A::_32
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        **self == DATASIZE_A::_24
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        **self == DATASIZE_A::_20
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        **self == DATASIZE_A::_18
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        **self == DATASIZE_A::_16
    }
    #[doc = "Checks if the value of the field is `_16C`"]
    #[inline(always)]
    pub fn is_16c(&self) -> bool {
        **self == DATASIZE_A::_16C
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == DATASIZE_A::_8
    }
    #[doc = "Checks if the value of the field is `_8C`"]
    #[inline(always)]
    pub fn is_8c(&self) -> bool {
        **self == DATASIZE_A::_8C
    }
}
impl core::ops::Deref for DATASIZE_R {
    type Target = crate::FieldReader<u8, DATASIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATASIZE` writer - Data Word Size"]
pub struct DATASIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATASIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATASIZE_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
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
#[doc = "Field `WORDADJ` reader - Data Word Formatting Adjust"]
pub struct WORDADJ_R(crate::FieldReader<bool, WORDADJ_A>);
impl WORDADJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WORDADJ_R(crate::FieldReader::new(bits))
    }
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
        **self == WORDADJ_A::RIGHT
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        **self == WORDADJ_A::LEFT
    }
}
impl core::ops::Deref for WORDADJ_R {
    type Target = crate::FieldReader<bool, WORDADJ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WORDADJ` writer - Data Word Formatting Adjust"]
pub struct WORDADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> WORDADJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WORDADJ_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
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
#[doc = "Field `EXTEND` reader - Data Formatting Bit Extension"]
pub struct EXTEND_R(crate::FieldReader<u8, EXTEND_A>);
impl EXTEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTEND_R(crate::FieldReader::new(bits))
    }
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
        **self == EXTEND_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == EXTEND_A::ONE
    }
    #[doc = "Checks if the value of the field is `MSBIT`"]
    #[inline(always)]
    pub fn is_msbit(&self) -> bool {
        **self == EXTEND_A::MSBIT
    }
    #[doc = "Checks if the value of the field is `LSBIT`"]
    #[inline(always)]
    pub fn is_lsbit(&self) -> bool {
        **self == EXTEND_A::LSBIT
    }
}
impl core::ops::Deref for EXTEND_R {
    type Target = crate::FieldReader<u8, EXTEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEND` writer - Data Formatting Bit Extension"]
pub struct EXTEND_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEND_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
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
#[doc = "Field `BITREV` reader - Data Formatting Bit Reverse"]
pub struct BITREV_R(crate::FieldReader<bool, BITREV_A>);
impl BITREV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BITREV_R(crate::FieldReader::new(bits))
    }
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
        **self == BITREV_A::MSBIT
    }
    #[doc = "Checks if the value of the field is `LSBIT`"]
    #[inline(always)]
    pub fn is_lsbit(&self) -> bool {
        **self == BITREV_A::LSBIT
    }
}
impl core::ops::Deref for BITREV_R {
    type Target = crate::FieldReader<bool, BITREV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BITREV` writer - Data Formatting Bit Reverse"]
pub struct BITREV_W<'a> {
    w: &'a mut W,
}
impl<'a> BITREV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BITREV_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `SLOTDIS0` reader - Slot 0 Disabled for this Serializer"]
pub struct SLOTDIS0_R(crate::FieldReader<bool, bool>);
impl SLOTDIS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLOTDIS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOTDIS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOTDIS0` writer - Slot 0 Disabled for this Serializer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SLOTDIS1` reader - Slot 1 Disabled for this Serializer"]
pub struct SLOTDIS1_R(crate::FieldReader<bool, bool>);
impl SLOTDIS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLOTDIS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOTDIS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOTDIS1` writer - Slot 1 Disabled for this Serializer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `SLOTDIS2` reader - Slot 2 Disabled for this Serializer"]
pub struct SLOTDIS2_R(crate::FieldReader<bool, bool>);
impl SLOTDIS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLOTDIS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOTDIS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOTDIS2` writer - Slot 2 Disabled for this Serializer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `SLOTDIS3` reader - Slot 3 Disabled for this Serializer"]
pub struct SLOTDIS3_R(crate::FieldReader<bool, bool>);
impl SLOTDIS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLOTDIS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOTDIS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOTDIS3` writer - Slot 3 Disabled for this Serializer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `SLOTDIS4` reader - Slot 4 Disabled for this Serializer"]
pub struct SLOTDIS4_R(crate::FieldReader<bool, bool>);
impl SLOTDIS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLOTDIS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOTDIS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOTDIS4` writer - Slot 4 Disabled for this Serializer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `SLOTDIS5` reader - Slot 5 Disabled for this Serializer"]
pub struct SLOTDIS5_R(crate::FieldReader<bool, bool>);
impl SLOTDIS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLOTDIS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOTDIS5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOTDIS5` writer - Slot 5 Disabled for this Serializer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `SLOTDIS6` reader - Slot 6 Disabled for this Serializer"]
pub struct SLOTDIS6_R(crate::FieldReader<bool, bool>);
impl SLOTDIS6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLOTDIS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOTDIS6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOTDIS6` writer - Slot 6 Disabled for this Serializer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `SLOTDIS7` reader - Slot 7 Disabled for this Serializer"]
pub struct SLOTDIS7_R(crate::FieldReader<bool, bool>);
impl SLOTDIS7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLOTDIS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOTDIS7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOTDIS7` writer - Slot 7 Disabled for this Serializer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
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
#[doc = "Field `MONO` reader - Mono Mode"]
pub struct MONO_R(crate::FieldReader<bool, MONO_A>);
impl MONO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MONO_R(crate::FieldReader::new(bits))
    }
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
        **self == MONO_A::STEREO
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        **self == MONO_A::MONO
    }
}
impl core::ops::Deref for MONO_R {
    type Target = crate::FieldReader<bool, MONO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONO` writer - Mono Mode"]
pub struct MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONO_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
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
#[doc = "Field `DMA` reader - Single or Multiple DMA Channels"]
pub struct DMA_R(crate::FieldReader<bool, DMA_A>);
impl DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
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
        **self == DMA_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        **self == DMA_A::MULTIPLE
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<bool, DMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA` writer - Single or Multiple DMA Channels"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Serializer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txctrl](index.html) module"]
pub struct TXCTRL_SPEC;
impl crate::RegisterSpec for TXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txctrl::R](R) reader structure"]
impl crate::Readable for TXCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txctrl::W](W) writer structure"]
impl crate::Writable for TXCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXCTRL to value 0"]
impl crate::Resettable for TXCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
