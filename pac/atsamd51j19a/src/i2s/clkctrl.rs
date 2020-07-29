#[doc = "Reader of register CLKCTRL[%s]"]
pub type R = crate::R<u32, super::CLKCTRL>;
#[doc = "Writer for register CLKCTRL[%s]"]
pub type W = crate::W<u32, super::CLKCTRL>;
#[doc = "Register CLKCTRL[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::CLKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Slot Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLOTSIZE_A {
    #[doc = "0: 8-bit Slot for Clock Unit n"]
    _8 = 0,
    #[doc = "1: 16-bit Slot for Clock Unit n"]
    _16 = 1,
    #[doc = "2: 24-bit Slot for Clock Unit n"]
    _24 = 2,
    #[doc = "3: 32-bit Slot for Clock Unit n"]
    _32 = 3,
}
impl From<SLOTSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SLOTSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SLOTSIZE`"]
pub type SLOTSIZE_R = crate::R<u8, SLOTSIZE_A>;
impl SLOTSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOTSIZE_A {
        match self.bits {
            0 => SLOTSIZE_A::_8,
            1 => SLOTSIZE_A::_16,
            2 => SLOTSIZE_A::_24,
            3 => SLOTSIZE_A::_32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SLOTSIZE_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == SLOTSIZE_A::_16
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == SLOTSIZE_A::_24
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == SLOTSIZE_A::_32
    }
}
#[doc = "Write proxy for field `SLOTSIZE`"]
pub struct SLOTSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOTSIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "8-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(SLOTSIZE_A::_8)
    }
    #[doc = "16-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(SLOTSIZE_A::_16)
    }
    #[doc = "24-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(SLOTSIZE_A::_24)
    }
    #[doc = "32-bit Slot for Clock Unit n"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(SLOTSIZE_A::_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `NBSLOTS`"]
pub type NBSLOTS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBSLOTS`"]
pub struct NBSLOTS_W<'a> {
    w: &'a mut W,
}
impl<'a> NBSLOTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Frame Sync Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FSWIDTH_A {
    #[doc = "0: Frame Sync Pulse is 1 Slot wide (default for I2S protocol)"]
    SLOT = 0,
    #[doc = "1: Frame Sync Pulse is half a Frame wide"]
    HALF = 1,
    #[doc = "2: Frame Sync Pulse is 1 Bit wide"]
    BIT = 2,
    #[doc = "3: Clock Unit n operates in Burst mode, with a 1-bit wide Frame Sync pulse per Data sample, only when Data transfer is requested"]
    BURST = 3,
}
impl From<FSWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: FSWIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FSWIDTH`"]
pub type FSWIDTH_R = crate::R<u8, FSWIDTH_A>;
impl FSWIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSWIDTH_A {
        match self.bits {
            0 => FSWIDTH_A::SLOT,
            1 => FSWIDTH_A::HALF,
            2 => FSWIDTH_A::BIT,
            3 => FSWIDTH_A::BURST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLOT`"]
    #[inline(always)]
    pub fn is_slot(&self) -> bool {
        *self == FSWIDTH_A::SLOT
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FSWIDTH_A::HALF
    }
    #[doc = "Checks if the value of the field is `BIT`"]
    #[inline(always)]
    pub fn is_bit_(&self) -> bool {
        *self == FSWIDTH_A::BIT
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == FSWIDTH_A::BURST
    }
}
#[doc = "Write proxy for field `FSWIDTH`"]
pub struct FSWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FSWIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSWIDTH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Frame Sync Pulse is 1 Slot wide (default for I2S protocol)"]
    #[inline(always)]
    pub fn slot(self) -> &'a mut W {
        self.variant(FSWIDTH_A::SLOT)
    }
    #[doc = "Frame Sync Pulse is half a Frame wide"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(FSWIDTH_A::HALF)
    }
    #[doc = "Frame Sync Pulse is 1 Bit wide"]
    #[inline(always)]
    pub fn bit_(self) -> &'a mut W {
        self.variant(FSWIDTH_A::BIT)
    }
    #[doc = "Clock Unit n operates in Burst mode, with a 1-bit wide Frame Sync pulse per Data sample, only when Data transfer is requested"]
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(FSWIDTH_A::BURST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Data Delay from Frame Sync\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITDELAY_A {
    #[doc = "0: Left Justified (0 Bit Delay)"]
    LJ = 0,
    #[doc = "1: I2S (1 Bit Delay)"]
    I2S = 1,
}
impl From<BITDELAY_A> for bool {
    #[inline(always)]
    fn from(variant: BITDELAY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BITDELAY`"]
pub type BITDELAY_R = crate::R<bool, BITDELAY_A>;
impl BITDELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BITDELAY_A {
        match self.bits {
            false => BITDELAY_A::LJ,
            true => BITDELAY_A::I2S,
        }
    }
    #[doc = "Checks if the value of the field is `LJ`"]
    #[inline(always)]
    pub fn is_lj(&self) -> bool {
        *self == BITDELAY_A::LJ
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == BITDELAY_A::I2S
    }
}
#[doc = "Write proxy for field `BITDELAY`"]
pub struct BITDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> BITDELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BITDELAY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Left Justified (0 Bit Delay)"]
    #[inline(always)]
    pub fn lj(self) -> &'a mut W {
        self.variant(BITDELAY_A::LJ)
    }
    #[doc = "I2S (1 Bit Delay)"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(BITDELAY_A::I2S)
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
#[doc = "Frame Sync Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSSEL_A {
    #[doc = "0: Divided Serial Clock n is used as Frame Sync n source"]
    SCKDIV = 0,
    #[doc = "1: FSn input pin is used as Frame Sync n source"]
    FSPIN = 1,
}
impl From<FSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FSSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FSSEL`"]
pub type FSSEL_R = crate::R<bool, FSSEL_A>;
impl FSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSSEL_A {
        match self.bits {
            false => FSSEL_A::SCKDIV,
            true => FSSEL_A::FSPIN,
        }
    }
    #[doc = "Checks if the value of the field is `SCKDIV`"]
    #[inline(always)]
    pub fn is_sckdiv(&self) -> bool {
        *self == FSSEL_A::SCKDIV
    }
    #[doc = "Checks if the value of the field is `FSPIN`"]
    #[inline(always)]
    pub fn is_fspin(&self) -> bool {
        *self == FSSEL_A::FSPIN
    }
}
#[doc = "Write proxy for field `FSSEL`"]
pub struct FSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Divided Serial Clock n is used as Frame Sync n source"]
    #[inline(always)]
    pub fn sckdiv(self) -> &'a mut W {
        self.variant(FSSEL_A::SCKDIV)
    }
    #[doc = "FSn input pin is used as Frame Sync n source"]
    #[inline(always)]
    pub fn fspin(self) -> &'a mut W {
        self.variant(FSSEL_A::FSPIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `FSINV`"]
pub type FSINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSINV`"]
pub struct FSINV_W<'a> {
    w: &'a mut W,
}
impl<'a> FSINV_W<'a> {
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
#[doc = "Reader of field `FSOUTINV`"]
pub type FSOUTINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSOUTINV`"]
pub struct FSOUTINV_W<'a> {
    w: &'a mut W,
}
impl<'a> FSOUTINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Serial Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCKSEL_A {
    #[doc = "0: Divided Master Clock n is used as Serial Clock n source"]
    MCKDIV = 0,
    #[doc = "1: SCKn input pin is used as Serial Clock n source"]
    SCKPIN = 1,
}
impl From<SCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCKSEL`"]
pub type SCKSEL_R = crate::R<bool, SCKSEL_A>;
impl SCKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKSEL_A {
        match self.bits {
            false => SCKSEL_A::MCKDIV,
            true => SCKSEL_A::SCKPIN,
        }
    }
    #[doc = "Checks if the value of the field is `MCKDIV`"]
    #[inline(always)]
    pub fn is_mckdiv(&self) -> bool {
        *self == SCKSEL_A::MCKDIV
    }
    #[doc = "Checks if the value of the field is `SCKPIN`"]
    #[inline(always)]
    pub fn is_sckpin(&self) -> bool {
        *self == SCKSEL_A::SCKPIN
    }
}
#[doc = "Write proxy for field `SCKSEL`"]
pub struct SCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Divided Master Clock n is used as Serial Clock n source"]
    #[inline(always)]
    pub fn mckdiv(self) -> &'a mut W {
        self.variant(SCKSEL_A::MCKDIV)
    }
    #[doc = "SCKn input pin is used as Serial Clock n source"]
    #[inline(always)]
    pub fn sckpin(self) -> &'a mut W {
        self.variant(SCKSEL_A::SCKPIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SCKOUTINV`"]
pub type SCKOUTINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCKOUTINV`"]
pub struct SCKOUTINV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKOUTINV_W<'a> {
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
#[doc = "Master Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCKSEL_A {
    #[doc = "0: GCLK_I2S_n is used as Master Clock n source"]
    GCLK = 0,
    #[doc = "1: MCKn input pin is used as Master Clock n source"]
    MCKPIN = 1,
}
impl From<MCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCKSEL`"]
pub type MCKSEL_R = crate::R<bool, MCKSEL_A>;
impl MCKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCKSEL_A {
        match self.bits {
            false => MCKSEL_A::GCLK,
            true => MCKSEL_A::MCKPIN,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == MCKSEL_A::GCLK
    }
    #[doc = "Checks if the value of the field is `MCKPIN`"]
    #[inline(always)]
    pub fn is_mckpin(&self) -> bool {
        *self == MCKSEL_A::MCKPIN
    }
}
#[doc = "Write proxy for field `MCKSEL`"]
pub struct MCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "GCLK_I2S_n is used as Master Clock n source"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut W {
        self.variant(MCKSEL_A::GCLK)
    }
    #[doc = "MCKn input pin is used as Master Clock n source"]
    #[inline(always)]
    pub fn mckpin(self) -> &'a mut W {
        self.variant(MCKSEL_A::MCKPIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `MCKEN`"]
pub type MCKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCKEN`"]
pub struct MCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `MCKOUTINV`"]
pub type MCKOUTINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCKOUTINV`"]
pub struct MCKOUTINV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKOUTINV_W<'a> {
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
#[doc = "Reader of field `MCKDIV`"]
pub type MCKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCKDIV`"]
pub struct MCKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MCKOUTDIV`"]
pub type MCKOUTDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCKOUTDIV`"]
pub struct MCKOUTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKOUTDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Slot Size"]
    #[inline(always)]
    pub fn slotsize(&self) -> SLOTSIZE_R {
        SLOTSIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - Number of Slots in Frame"]
    #[inline(always)]
    pub fn nbslots(&self) -> NBSLOTS_R {
        NBSLOTS_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:6 - Frame Sync Width"]
    #[inline(always)]
    pub fn fswidth(&self) -> FSWIDTH_R {
        FSWIDTH_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Data Delay from Frame Sync"]
    #[inline(always)]
    pub fn bitdelay(&self) -> BITDELAY_R {
        BITDELAY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Frame Sync Select"]
    #[inline(always)]
    pub fn fssel(&self) -> FSSEL_R {
        FSSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Frame Sync Invert"]
    #[inline(always)]
    pub fn fsinv(&self) -> FSINV_R {
        FSINV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Frame Sync Output Invert"]
    #[inline(always)]
    pub fn fsoutinv(&self) -> FSOUTINV_R {
        FSOUTINV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Serial Clock Select"]
    #[inline(always)]
    pub fn scksel(&self) -> SCKSEL_R {
        SCKSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Serial Clock Output Invert"]
    #[inline(always)]
    pub fn sckoutinv(&self) -> SCKOUTINV_R {
        SCKOUTINV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Master Clock Select"]
    #[inline(always)]
    pub fn mcksel(&self) -> MCKSEL_R {
        MCKSEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Master Clock Enable"]
    #[inline(always)]
    pub fn mcken(&self) -> MCKEN_R {
        MCKEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Master Clock Output Invert"]
    #[inline(always)]
    pub fn mckoutinv(&self) -> MCKOUTINV_R {
        MCKOUTINV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Master Clock Division Factor"]
    #[inline(always)]
    pub fn mckdiv(&self) -> MCKDIV_R {
        MCKDIV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Master Clock Output Division Factor"]
    #[inline(always)]
    pub fn mckoutdiv(&self) -> MCKOUTDIV_R {
        MCKOUTDIV_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Slot Size"]
    #[inline(always)]
    pub fn slotsize(&mut self) -> SLOTSIZE_W {
        SLOTSIZE_W { w: self }
    }
    #[doc = "Bits 2:4 - Number of Slots in Frame"]
    #[inline(always)]
    pub fn nbslots(&mut self) -> NBSLOTS_W {
        NBSLOTS_W { w: self }
    }
    #[doc = "Bits 5:6 - Frame Sync Width"]
    #[inline(always)]
    pub fn fswidth(&mut self) -> FSWIDTH_W {
        FSWIDTH_W { w: self }
    }
    #[doc = "Bit 7 - Data Delay from Frame Sync"]
    #[inline(always)]
    pub fn bitdelay(&mut self) -> BITDELAY_W {
        BITDELAY_W { w: self }
    }
    #[doc = "Bit 8 - Frame Sync Select"]
    #[inline(always)]
    pub fn fssel(&mut self) -> FSSEL_W {
        FSSEL_W { w: self }
    }
    #[doc = "Bit 9 - Frame Sync Invert"]
    #[inline(always)]
    pub fn fsinv(&mut self) -> FSINV_W {
        FSINV_W { w: self }
    }
    #[doc = "Bit 10 - Frame Sync Output Invert"]
    #[inline(always)]
    pub fn fsoutinv(&mut self) -> FSOUTINV_W {
        FSOUTINV_W { w: self }
    }
    #[doc = "Bit 11 - Serial Clock Select"]
    #[inline(always)]
    pub fn scksel(&mut self) -> SCKSEL_W {
        SCKSEL_W { w: self }
    }
    #[doc = "Bit 12 - Serial Clock Output Invert"]
    #[inline(always)]
    pub fn sckoutinv(&mut self) -> SCKOUTINV_W {
        SCKOUTINV_W { w: self }
    }
    #[doc = "Bit 13 - Master Clock Select"]
    #[inline(always)]
    pub fn mcksel(&mut self) -> MCKSEL_W {
        MCKSEL_W { w: self }
    }
    #[doc = "Bit 14 - Master Clock Enable"]
    #[inline(always)]
    pub fn mcken(&mut self) -> MCKEN_W {
        MCKEN_W { w: self }
    }
    #[doc = "Bit 15 - Master Clock Output Invert"]
    #[inline(always)]
    pub fn mckoutinv(&mut self) -> MCKOUTINV_W {
        MCKOUTINV_W { w: self }
    }
    #[doc = "Bits 16:21 - Master Clock Division Factor"]
    #[inline(always)]
    pub fn mckdiv(&mut self) -> MCKDIV_W {
        MCKDIV_W { w: self }
    }
    #[doc = "Bits 24:29 - Master Clock Output Division Factor"]
    #[inline(always)]
    pub fn mckoutdiv(&mut self) -> MCKOUTDIV_W {
        MCKOUTDIV_W { w: self }
    }
}
