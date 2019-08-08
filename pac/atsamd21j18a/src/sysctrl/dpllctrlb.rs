#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DPLLCTRLB {
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
#[doc = "Possible values of the field `FILTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTERR {
    #[doc = "Default filter mode"]
    DEFAULT,
    #[doc = "Low bandwidth filter"]
    LBFILT,
    #[doc = "High bandwidth filter"]
    HBFILT,
    #[doc = "High damping filter"]
    HDFILT,
}
impl FILTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FILTERR::DEFAULT => 0,
            FILTERR::LBFILT => 1,
            FILTERR::HBFILT => 2,
            FILTERR::HDFILT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FILTERR {
        match value {
            0 => FILTERR::DEFAULT,
            1 => FILTERR::LBFILT,
            2 => FILTERR::HBFILT,
            3 => FILTERR::HDFILT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == FILTERR::DEFAULT
    }
    #[doc = "Checks if the value of the field is `LBFILT`"]
    #[inline]
    pub fn is_lbfilt(&self) -> bool {
        *self == FILTERR::LBFILT
    }
    #[doc = "Checks if the value of the field is `HBFILT`"]
    #[inline]
    pub fn is_hbfilt(&self) -> bool {
        *self == FILTERR::HBFILT
    }
    #[doc = "Checks if the value of the field is `HDFILT`"]
    #[inline]
    pub fn is_hdfilt(&self) -> bool {
        *self == FILTERR::HDFILT
    }
}
#[doc = r" Value of the field"]
pub struct LPENR {
    bits: bool,
}
impl LPENR {
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
pub struct WUFR {
    bits: bool,
}
impl WUFR {
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
#[doc = "Possible values of the field `REFCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFCLKR {
    #[doc = "CLK_DPLL_REF0 clock reference"]
    REF0,
    #[doc = "CLK_DPLL_REF1 clock reference"]
    REF1,
    #[doc = "GCLK_DPLL clock reference"]
    GCLK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REFCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFCLKR::REF0 => 0,
            REFCLKR::REF1 => 1,
            REFCLKR::GCLK => 2,
            REFCLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFCLKR {
        match value {
            0 => REFCLKR::REF0,
            1 => REFCLKR::REF1,
            2 => REFCLKR::GCLK,
            i => REFCLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REF0`"]
    #[inline]
    pub fn is_ref0(&self) -> bool {
        *self == REFCLKR::REF0
    }
    #[doc = "Checks if the value of the field is `REF1`"]
    #[inline]
    pub fn is_ref1(&self) -> bool {
        *self == REFCLKR::REF1
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline]
    pub fn is_gclk(&self) -> bool {
        *self == REFCLKR::GCLK
    }
}
#[doc = "Possible values of the field `LTIME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTIMER {
    #[doc = "No time-out"]
    DEFAULT,
    #[doc = "Time-out if no lock within 8 ms"]
    _8MS,
    #[doc = "Time-out if no lock within 9 ms"]
    _9MS,
    #[doc = "Time-out if no lock within 10 ms"]
    _10MS,
    #[doc = "Time-out if no lock within 11 ms"]
    _11MS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LTIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LTIMER::DEFAULT => 0,
            LTIMER::_8MS => 4,
            LTIMER::_9MS => 5,
            LTIMER::_10MS => 6,
            LTIMER::_11MS => 7,
            LTIMER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LTIMER {
        match value {
            0 => LTIMER::DEFAULT,
            4 => LTIMER::_8MS,
            5 => LTIMER::_9MS,
            6 => LTIMER::_10MS,
            7 => LTIMER::_11MS,
            i => LTIMER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == LTIMER::DEFAULT
    }
    #[doc = "Checks if the value of the field is `_8MS`"]
    #[inline]
    pub fn is_8ms(&self) -> bool {
        *self == LTIMER::_8MS
    }
    #[doc = "Checks if the value of the field is `_9MS`"]
    #[inline]
    pub fn is_9ms(&self) -> bool {
        *self == LTIMER::_9MS
    }
    #[doc = "Checks if the value of the field is `_10MS`"]
    #[inline]
    pub fn is_10ms(&self) -> bool {
        *self == LTIMER::_10MS
    }
    #[doc = "Checks if the value of the field is `_11MS`"]
    #[inline]
    pub fn is_11ms(&self) -> bool {
        *self == LTIMER::_11MS
    }
}
#[doc = r" Value of the field"]
pub struct LBYPASSR {
    bits: bool,
}
impl LBYPASSR {
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
pub struct DIVR {
    bits: u16,
}
impl DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `FILTER`"]
pub enum FILTERW {
    #[doc = "Default filter mode"]
    DEFAULT,
    #[doc = "Low bandwidth filter"]
    LBFILT,
    #[doc = "High bandwidth filter"]
    HBFILT,
    #[doc = "High damping filter"]
    HDFILT,
}
impl FILTERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FILTERW::DEFAULT => 0,
            FILTERW::LBFILT => 1,
            FILTERW::HBFILT => 2,
            FILTERW::HDFILT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTERW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Default filter mode"]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(FILTERW::DEFAULT)
    }
    #[doc = "Low bandwidth filter"]
    #[inline]
    pub fn lbfilt(self) -> &'a mut W {
        self.variant(FILTERW::LBFILT)
    }
    #[doc = "High bandwidth filter"]
    #[inline]
    pub fn hbfilt(self) -> &'a mut W {
        self.variant(FILTERW::HBFILT)
    }
    #[doc = "High damping filter"]
    #[inline]
    pub fn hdfilt(self) -> &'a mut W {
        self.variant(FILTERW::HDFILT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPENW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WUFW<'a> {
    w: &'a mut W,
}
impl<'a> _WUFW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFCLK`"]
pub enum REFCLKW {
    #[doc = "CLK_DPLL_REF0 clock reference"]
    REF0,
    #[doc = "CLK_DPLL_REF1 clock reference"]
    REF1,
    #[doc = "GCLK_DPLL clock reference"]
    GCLK,
}
impl REFCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFCLKW::REF0 => 0,
            REFCLKW::REF1 => 1,
            REFCLKW::GCLK => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _REFCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFCLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CLK_DPLL_REF0 clock reference"]
    #[inline]
    pub fn ref0(self) -> &'a mut W {
        self.variant(REFCLKW::REF0)
    }
    #[doc = "CLK_DPLL_REF1 clock reference"]
    #[inline]
    pub fn ref1(self) -> &'a mut W {
        self.variant(REFCLKW::REF1)
    }
    #[doc = "GCLK_DPLL clock reference"]
    #[inline]
    pub fn gclk(self) -> &'a mut W {
        self.variant(REFCLKW::GCLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LTIME`"]
pub enum LTIMEW {
    #[doc = "No time-out"]
    DEFAULT,
    #[doc = "Time-out if no lock within 8 ms"]
    _8MS,
    #[doc = "Time-out if no lock within 9 ms"]
    _9MS,
    #[doc = "Time-out if no lock within 10 ms"]
    _10MS,
    #[doc = "Time-out if no lock within 11 ms"]
    _11MS,
}
impl LTIMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LTIMEW::DEFAULT => 0,
            LTIMEW::_8MS => 4,
            LTIMEW::_9MS => 5,
            LTIMEW::_10MS => 6,
            LTIMEW::_11MS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LTIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _LTIMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LTIMEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No time-out"]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(LTIMEW::DEFAULT)
    }
    #[doc = "Time-out if no lock within 8 ms"]
    #[inline]
    pub fn _8ms(self) -> &'a mut W {
        self.variant(LTIMEW::_8MS)
    }
    #[doc = "Time-out if no lock within 9 ms"]
    #[inline]
    pub fn _9ms(self) -> &'a mut W {
        self.variant(LTIMEW::_9MS)
    }
    #[doc = "Time-out if no lock within 10 ms"]
    #[inline]
    pub fn _10ms(self) -> &'a mut W {
        self.variant(LTIMEW::_10MS)
    }
    #[doc = "Time-out if no lock within 11 ms"]
    #[inline]
    pub fn _11ms(self) -> &'a mut W {
        self.variant(LTIMEW::_11MS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LBYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _LBYPASSW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Proportional Integral Filter Selection"]
    #[inline]
    pub fn filter(&self) -> FILTERR {
        FILTERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Low-Power Enable"]
    #[inline]
    pub fn lpen(&self) -> LPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPENR { bits }
    }
    #[doc = "Bit 3 - Wake Up Fast"]
    #[inline]
    pub fn wuf(&self) -> WUFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WUFR { bits }
    }
    #[doc = "Bits 4:5 - Reference Clock Selection"]
    #[inline]
    pub fn refclk(&self) -> REFCLKR {
        REFCLKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline]
    pub fn ltime(&self) -> LTIMER {
        LTIMER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Lock Bypass"]
    #[inline]
    pub fn lbypass(&self) -> LBYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LBYPASSR { bits }
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline]
    pub fn div(&self) -> DIVR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DIVR { bits }
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Proportional Integral Filter Selection"]
    #[inline]
    pub fn filter(&mut self) -> _FILTERW {
        _FILTERW { w: self }
    }
    #[doc = "Bit 2 - Low-Power Enable"]
    #[inline]
    pub fn lpen(&mut self) -> _LPENW {
        _LPENW { w: self }
    }
    #[doc = "Bit 3 - Wake Up Fast"]
    #[inline]
    pub fn wuf(&mut self) -> _WUFW {
        _WUFW { w: self }
    }
    #[doc = "Bits 4:5 - Reference Clock Selection"]
    #[inline]
    pub fn refclk(&mut self) -> _REFCLKW {
        _REFCLKW { w: self }
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline]
    pub fn ltime(&mut self) -> _LTIMEW {
        _LTIMEW { w: self }
    }
    #[doc = "Bit 12 - Lock Bypass"]
    #[inline]
    pub fn lbypass(&mut self) -> _LBYPASSW {
        _LBYPASSW { w: self }
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline]
    pub fn div(&mut self) -> _DIVW {
        _DIVW { w: self }
    }
}
