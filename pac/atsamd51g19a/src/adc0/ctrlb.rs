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
#[doc = "Possible values of the field `WINMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINMODER {
    #[doc = "No window mode (default)"]
    DISABLE,
    #[doc = "RESULT > WINLT"]
    MODE1,
    #[doc = "RESULT < WINUT"]
    MODE2,
    #[doc = "WINLT < RESULT < WINUT"]
    MODE3,
    #[doc = "!(WINLT < RESULT < WINUT)"]
    MODE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WINMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WINMODER::DISABLE => 0,
            WINMODER::MODE1 => 1,
            WINMODER::MODE2 => 2,
            WINMODER::MODE3 => 3,
            WINMODER::MODE4 => 4,
            WINMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WINMODER {
        match value {
            0 => WINMODER::DISABLE,
            1 => WINMODER::MODE1,
            2 => WINMODER::MODE2,
            3 => WINMODER::MODE3,
            4 => WINMODER::MODE4,
            i => WINMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WINMODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline]
    pub fn is_mode1(&self) -> bool {
        *self == WINMODER::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline]
    pub fn is_mode2(&self) -> bool {
        *self == WINMODER::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline]
    pub fn is_mode3(&self) -> bool {
        *self == WINMODER::MODE3
    }
    #[doc = "Checks if the value of the field is `MODE4`"]
    #[inline]
    pub fn is_mode4(&self) -> bool {
        *self == WINMODER::MODE4
    }
}
#[doc = r" Value of the field"]
pub struct WINSSR {
    bits: bool,
}
impl WINSSR {
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
        const OFFSET: u8 = 0;
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
        const OFFSET: u8 = 1;
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
        const OFFSET: u8 = 2;
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WINMODE`"]
pub enum WINMODEW {
    #[doc = "No window mode (default)"]
    DISABLE,
    #[doc = "RESULT > WINLT"]
    MODE1,
    #[doc = "RESULT < WINUT"]
    MODE2,
    #[doc = "WINLT < RESULT < WINUT"]
    MODE3,
    #[doc = "!(WINLT < RESULT < WINUT)"]
    MODE4,
}
impl WINMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WINMODEW::DISABLE => 0,
            WINMODEW::MODE1 => 1,
            WINMODEW::MODE2 => 2,
            WINMODEW::MODE3 => 3,
            WINMODEW::MODE4 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WINMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WINMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WINMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No window mode (default)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WINMODEW::DISABLE)
    }
    #[doc = "RESULT > WINLT"]
    #[inline]
    pub fn mode1(self) -> &'a mut W {
        self.variant(WINMODEW::MODE1)
    }
    #[doc = "RESULT < WINUT"]
    #[inline]
    pub fn mode2(self) -> &'a mut W {
        self.variant(WINMODEW::MODE2)
    }
    #[doc = "WINLT < RESULT < WINUT"]
    #[inline]
    pub fn mode3(self) -> &'a mut W {
        self.variant(WINMODEW::MODE3)
    }
    #[doc = "!(WINLT < RESULT < WINUT)"]
    #[inline]
    pub fn mode4(self) -> &'a mut W {
        self.variant(WINMODEW::MODE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WINSSW<'a> {
    w: &'a mut W,
}
impl<'a> _WINSSW<'a> {
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
        const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Left-Adjusted Result"]
    #[inline]
    pub fn leftadj(&self) -> LEFTADJR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        LEFTADJR { bits }
    }
    #[doc = "Bit 1 - Free Running Mode"]
    #[inline]
    pub fn freerun(&self) -> FREERUNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        FREERUNR { bits }
    }
    #[doc = "Bit 2 - Digital Correction Logic Enable"]
    #[inline]
    pub fn corren(&self) -> CORRENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CORRENR { bits }
    }
    #[doc = "Bits 3:4 - Conversion Result Resolution"]
    #[inline]
    pub fn ressel(&self) -> RESSELR {
        RESSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline]
    pub fn winmode(&self) -> WINMODER {
        WINMODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 11 - Window Single Sample"]
    #[inline]
    pub fn winss(&self) -> WINSSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        WINSSR { bits }
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
    #[doc = "Bit 0 - Left-Adjusted Result"]
    #[inline]
    pub fn leftadj(&mut self) -> _LEFTADJW {
        _LEFTADJW { w: self }
    }
    #[doc = "Bit 1 - Free Running Mode"]
    #[inline]
    pub fn freerun(&mut self) -> _FREERUNW {
        _FREERUNW { w: self }
    }
    #[doc = "Bit 2 - Digital Correction Logic Enable"]
    #[inline]
    pub fn corren(&mut self) -> _CORRENW {
        _CORRENW { w: self }
    }
    #[doc = "Bits 3:4 - Conversion Result Resolution"]
    #[inline]
    pub fn ressel(&mut self) -> _RESSELW {
        _RESSELW { w: self }
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline]
    pub fn winmode(&mut self) -> _WINMODEW {
        _WINMODEW { w: self }
    }
    #[doc = "Bit 11 - Window Single Sample"]
    #[inline]
    pub fn winss(&mut self) -> _WINSSW {
        _WINSSW { w: self }
    }
}
