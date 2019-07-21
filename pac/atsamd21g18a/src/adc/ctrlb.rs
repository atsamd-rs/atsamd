#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRLB {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r" Reset value of the register"]
    #[inline]
    pub const fn reset_value() -> u16 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct DIFFMODER {
    bits: bool,
}
impl DIFFMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct LEFTADJR {
    bits: bool,
}
impl LEFTADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FREERUNR {
    bits: bool,
}
impl FREERUNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct CORRENR {
    bits: bool,
}
impl CORRENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `RESSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESSELR {
    #[doc = "12-bit result"]
    _12BIT,
    #[doc = "For averaging mode output"]
    _16BIT,
    #[doc = "10-bit result"]
    _10BIT,
    #[doc = "8-bit result"]
    _8BIT,
}
impl RESSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESSELR::_12BIT => 0,
            RESSELR::_16BIT => 0x01,
            RESSELR::_10BIT => 0x02,
            RESSELR::_8BIT => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESSELR {
        match value {
            0 => RESSELR::_12BIT,
            1 => RESSELR::_16BIT,
            2 => RESSELR::_10BIT,
            3 => RESSELR::_8BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline]
    pub fn is_12bit(&self) -> bool {
        *self == RESSELR::_12BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline]
    pub fn is_16bit(&self) -> bool {
        *self == RESSELR::_16BIT
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline]
    pub fn is_10bit(&self) -> bool {
        *self == RESSELR::_10BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline]
    pub fn is_8bit(&self) -> bool {
        *self == RESSELR::_8BIT
    }
}
#[doc = "Possible values of the field `PRESCALER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALERR {
    #[doc = "Peripheral clock divided by 4"]
    DIV4,
    #[doc = "Peripheral clock divided by 8"]
    DIV8,
    #[doc = "Peripheral clock divided by 16"]
    DIV16,
    #[doc = "Peripheral clock divided by 32"]
    DIV32,
    #[doc = "Peripheral clock divided by 64"]
    DIV64,
    #[doc = "Peripheral clock divided by 128"]
    DIV128,
    #[doc = "Peripheral clock divided by 256"]
    DIV256,
    #[doc = "Peripheral clock divided by 512"]
    DIV512,
}
impl PRESCALERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCALERR::DIV4 => 0,
            PRESCALERR::DIV8 => 0x01,
            PRESCALERR::DIV16 => 0x02,
            PRESCALERR::DIV32 => 0x03,
            PRESCALERR::DIV64 => 0x04,
            PRESCALERR::DIV128 => 0x05,
            PRESCALERR::DIV256 => 0x06,
            PRESCALERR::DIV512 => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCALERR {
        match value {
            0 => PRESCALERR::DIV4,
            1 => PRESCALERR::DIV8,
            2 => PRESCALERR::DIV16,
            3 => PRESCALERR::DIV32,
            4 => PRESCALERR::DIV64,
            5 => PRESCALERR::DIV128,
            6 => PRESCALERR::DIV256,
            7 => PRESCALERR::DIV512,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALERR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALERR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALERR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALERR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALERR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALERR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALERR::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline]
    pub fn is_div512(&self) -> bool {
        *self == PRESCALERR::DIV512
    }
}
#[doc = r" Proxy"]
pub struct _DIFFMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DIFFMODEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 0);
        self.w.bits |= ((value as u16) & 0x01) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LEFTADJW<'a> {
    w: &'a mut W,
}
impl<'a> _LEFTADJW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 1);
        self.w.bits |= ((value as u16) & 0x01) << 1;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FREERUNW<'a> {
    w: &'a mut W,
}
impl<'a> _FREERUNW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 2);
        self.w.bits |= ((value as u16) & 0x01) << 2;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CORRENW<'a> {
    w: &'a mut W,
}
impl<'a> _CORRENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 3);
        self.w.bits |= ((value as u16) & 0x01) << 3;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESSELW {
    #[doc = "12-bit result"]
    _12BIT,
    #[doc = "For averaging mode output"]
    _16BIT,
    #[doc = "10-bit result"]
    _10BIT,
    #[doc = "8-bit result"]
    _8BIT,
}
impl RESSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESSELW::_12BIT => 0,
            RESSELW::_16BIT => 1,
            RESSELW::_10BIT => 2,
            RESSELW::_8BIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RESSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "12-bit result"]
    #[inline]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(RESSELW::_12BIT)
    }
    #[doc = "For averaging mode output"]
    #[inline]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(RESSELW::_16BIT)
    }
    #[doc = "10-bit result"]
    #[inline]
    pub fn _10bit(self) -> &'a mut W {
        self.variant(RESSELW::_10BIT)
    }
    #[doc = "8-bit result"]
    #[inline]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(RESSELW::_8BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 4);
        self.w.bits |= ((value as u16) & 0x03) << 4;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESCALER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALERW {
    #[doc = "Peripheral clock divided by 4"]
    DIV4,
    #[doc = "Peripheral clock divided by 8"]
    DIV8,
    #[doc = "Peripheral clock divided by 16"]
    DIV16,
    #[doc = "Peripheral clock divided by 32"]
    DIV32,
    #[doc = "Peripheral clock divided by 64"]
    DIV64,
    #[doc = "Peripheral clock divided by 128"]
    DIV128,
    #[doc = "Peripheral clock divided by 256"]
    DIV256,
    #[doc = "Peripheral clock divided by 512"]
    DIV512,
}
impl PRESCALERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCALERW::DIV4 => 0,
            PRESCALERW::DIV8 => 1,
            PRESCALERW::DIV16 => 2,
            PRESCALERW::DIV32 => 3,
            PRESCALERW::DIV64 => 4,
            PRESCALERW::DIV128 => 5,
            PRESCALERW::DIV256 => 6,
            PRESCALERW::DIV512 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALERW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCALERW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Peripheral clock divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV4)
    }
    #[doc = "Peripheral clock divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV8)
    }
    #[doc = "Peripheral clock divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV16)
    }
    #[doc = "Peripheral clock divided by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV32)
    }
    #[doc = "Peripheral clock divided by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV64)
    }
    #[doc = "Peripheral clock divided by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV128)
    }
    #[doc = "Peripheral clock divided by 256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV256)
    }
    #[doc = "Peripheral clock divided by 512"]
    #[inline]
    pub fn div512(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV512)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 8);
        self.w.bits |= ((value as u16) & 0x07) << 8;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Differential Mode"]
    #[inline]
    pub fn diffmode(&self) -> DIFFMODER {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        DIFFMODER { bits }
    }
    #[doc = "Bit 1 - Left-Adjusted Result"]
    #[inline]
    pub fn leftadj(&self) -> LEFTADJR {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        LEFTADJR { bits }
    }
    #[doc = "Bit 2 - Free Running Mode"]
    #[inline]
    pub fn freerun(&self) -> FREERUNR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        FREERUNR { bits }
    }
    #[doc = "Bit 3 - Digital Correction Logic Enabled"]
    #[inline]
    pub fn corren(&self) -> CORRENR {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        CORRENR { bits }
    }
    #[doc = "Bits 4:5 - Conversion Result Resolution"]
    #[inline]
    pub fn ressel(&self) -> RESSELR {
        RESSELR::_from(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Prescaler Configuration"]
    #[inline]
    pub fn prescaler(&self) -> PRESCALERR {
        PRESCALERR::_from(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Differential Mode"]
    #[inline]
    pub fn diffmode(&mut self) -> _DIFFMODEW {
        _DIFFMODEW { w: self }
    }
    #[doc = "Bit 1 - Left-Adjusted Result"]
    #[inline]
    pub fn leftadj(&mut self) -> _LEFTADJW {
        _LEFTADJW { w: self }
    }
    #[doc = "Bit 2 - Free Running Mode"]
    #[inline]
    pub fn freerun(&mut self) -> _FREERUNW {
        _FREERUNW { w: self }
    }
    #[doc = "Bit 3 - Digital Correction Logic Enabled"]
    #[inline]
    pub fn corren(&mut self) -> _CORRENW {
        _CORRENW { w: self }
    }
    #[doc = "Bits 4:5 - Conversion Result Resolution"]
    #[inline]
    pub fn ressel(&mut self) -> _RESSELW {
        _RESSELW { w: self }
    }
    #[doc = "Bits 8:10 - Prescaler Configuration"]
    #[inline]
    pub fn prescaler(&mut self) -> _PRESCALERW {
        _PRESCALERW { w: self }
    }
}
