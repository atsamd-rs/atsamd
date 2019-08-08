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
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
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
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
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
            RESSELR::_16BIT => 1,
            RESSELR::_10BIT => 2,
            RESSELR::_8BIT => 3,
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
            PRESCALERR::DIV8 => 1,
            PRESCALERR::DIV16 => 2,
            PRESCALERR::DIV32 => 3,
            PRESCALERR::DIV64 => 4,
            PRESCALERR::DIV128 => 5,
            PRESCALERR::DIV256 => 6,
            PRESCALERR::DIV512 => 7,
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
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
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
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
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
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
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
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESSEL`"]
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
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESCALER`"]
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
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
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
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        DIFFMODER { bits }
    }
    #[doc = "Bit 1 - Left-Adjusted Result"]
    #[inline]
    pub fn leftadj(&self) -> LEFTADJR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        LEFTADJR { bits }
    }
    #[doc = "Bit 2 - Free Running Mode"]
    #[inline]
    pub fn freerun(&self) -> FREERUNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        FREERUNR { bits }
    }
    #[doc = "Bit 3 - Digital Correction Logic Enabled"]
    #[inline]
    pub fn corren(&self) -> CORRENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CORRENR { bits }
    }
    #[doc = "Bits 4:5 - Conversion Result Resolution"]
    #[inline]
    pub fn ressel(&self) -> RESSELR {
        RESSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:10 - Prescaler Configuration"]
    #[inline]
    pub fn prescaler(&self) -> PRESCALERR {
        PRESCALERR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
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
