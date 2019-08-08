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
#[doc = r" Value of the field"]
pub struct FILTERR {
    bits: u8,
}
impl FILTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
    #[doc = "Dedicated GCLK clock reference"]
    GCLK,
    #[doc = "XOSC32K clock reference"]
    XOSC32,
    #[doc = "XOSC0 clock reference"]
    XOSC0,
    #[doc = "XOSC1 clock reference"]
    XOSC1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REFCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFCLKR::GCLK => 0,
            REFCLKR::XOSC32 => 1,
            REFCLKR::XOSC0 => 2,
            REFCLKR::XOSC1 => 3,
            REFCLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFCLKR {
        match value {
            0 => REFCLKR::GCLK,
            1 => REFCLKR::XOSC32,
            2 => REFCLKR::XOSC0,
            3 => REFCLKR::XOSC1,
            i => REFCLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline]
    pub fn is_gclk(&self) -> bool {
        *self == REFCLKR::GCLK
    }
    #[doc = "Checks if the value of the field is `XOSC32`"]
    #[inline]
    pub fn is_xosc32(&self) -> bool {
        *self == REFCLKR::XOSC32
    }
    #[doc = "Checks if the value of the field is `XOSC0`"]
    #[inline]
    pub fn is_xosc0(&self) -> bool {
        *self == REFCLKR::XOSC0
    }
    #[doc = "Checks if the value of the field is `XOSC1`"]
    #[inline]
    pub fn is_xosc1(&self) -> bool {
        *self == REFCLKR::XOSC1
    }
}
#[doc = "Possible values of the field `LTIME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTIMER {
    #[doc = "No time-out. Automatic lock"]
    DEFAULT,
    #[doc = "Time-out if no lock within 800us"]
    _800US,
    #[doc = "Time-out if no lock within 900us"]
    _900US,
    #[doc = "Time-out if no lock within 1ms"]
    _1MS,
    #[doc = "Time-out if no lock within 1.1ms"]
    _1P1MS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LTIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LTIMER::DEFAULT => 0,
            LTIMER::_800US => 4,
            LTIMER::_900US => 5,
            LTIMER::_1MS => 6,
            LTIMER::_1P1MS => 7,
            LTIMER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LTIMER {
        match value {
            0 => LTIMER::DEFAULT,
            4 => LTIMER::_800US,
            5 => LTIMER::_900US,
            6 => LTIMER::_1MS,
            7 => LTIMER::_1P1MS,
            i => LTIMER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == LTIMER::DEFAULT
    }
    #[doc = "Checks if the value of the field is `_800US`"]
    #[inline]
    pub fn is_800us(&self) -> bool {
        *self == LTIMER::_800US
    }
    #[doc = "Checks if the value of the field is `_900US`"]
    #[inline]
    pub fn is_900us(&self) -> bool {
        *self == LTIMER::_900US
    }
    #[doc = "Checks if the value of the field is `_1MS`"]
    #[inline]
    pub fn is_1ms(&self) -> bool {
        *self == LTIMER::_1MS
    }
    #[doc = "Checks if the value of the field is `_1P1MS`"]
    #[inline]
    pub fn is_1p1ms(&self) -> bool {
        *self == LTIMER::_1P1MS
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
pub struct DCOFILTERR {
    bits: u8,
}
impl DCOFILTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCOENR {
    bits: bool,
}
impl DCOENR {
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
#[doc = r" Proxy"]
pub struct _FILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFCLK`"]
pub enum REFCLKW {
    #[doc = "Dedicated GCLK clock reference"]
    GCLK,
    #[doc = "XOSC32K clock reference"]
    XOSC32,
    #[doc = "XOSC0 clock reference"]
    XOSC0,
    #[doc = "XOSC1 clock reference"]
    XOSC1,
}
impl REFCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFCLKW::GCLK => 0,
            REFCLKW::XOSC32 => 1,
            REFCLKW::XOSC0 => 2,
            REFCLKW::XOSC1 => 3,
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
    #[doc = "Dedicated GCLK clock reference"]
    #[inline]
    pub fn gclk(self) -> &'a mut W {
        self.variant(REFCLKW::GCLK)
    }
    #[doc = "XOSC32K clock reference"]
    #[inline]
    pub fn xosc32(self) -> &'a mut W {
        self.variant(REFCLKW::XOSC32)
    }
    #[doc = "XOSC0 clock reference"]
    #[inline]
    pub fn xosc0(self) -> &'a mut W {
        self.variant(REFCLKW::XOSC0)
    }
    #[doc = "XOSC1 clock reference"]
    #[inline]
    pub fn xosc1(self) -> &'a mut W {
        self.variant(REFCLKW::XOSC1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LTIME`"]
pub enum LTIMEW {
    #[doc = "No time-out. Automatic lock"]
    DEFAULT,
    #[doc = "Time-out if no lock within 800us"]
    _800US,
    #[doc = "Time-out if no lock within 900us"]
    _900US,
    #[doc = "Time-out if no lock within 1ms"]
    _1MS,
    #[doc = "Time-out if no lock within 1.1ms"]
    _1P1MS,
}
impl LTIMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LTIMEW::DEFAULT => 0,
            LTIMEW::_800US => 4,
            LTIMEW::_900US => 5,
            LTIMEW::_1MS => 6,
            LTIMEW::_1P1MS => 7,
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
    #[doc = "No time-out. Automatic lock"]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(LTIMEW::DEFAULT)
    }
    #[doc = "Time-out if no lock within 800us"]
    #[inline]
    pub fn _800us(self) -> &'a mut W {
        self.variant(LTIMEW::_800US)
    }
    #[doc = "Time-out if no lock within 900us"]
    #[inline]
    pub fn _900us(self) -> &'a mut W {
        self.variant(LTIMEW::_900US)
    }
    #[doc = "Time-out if no lock within 1ms"]
    #[inline]
    pub fn _1ms(self) -> &'a mut W {
        self.variant(LTIMEW::_1MS)
    }
    #[doc = "Time-out if no lock within 1.1ms"]
    #[inline]
    pub fn _1p1ms(self) -> &'a mut W {
        self.variant(LTIMEW::_1P1MS)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCOFILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOFILTERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCOENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOENW<'a> {
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:3 - Proportional Integral Filter Selection"]
    #[inline]
    pub fn filter(&self) -> FILTERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FILTERR { bits }
    }
    #[doc = "Bit 4 - Wake Up Fast"]
    #[inline]
    pub fn wuf(&self) -> WUFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WUFR { bits }
    }
    #[doc = "Bits 5:7 - Reference Clock Selection"]
    #[inline]
    pub fn refclk(&self) -> REFCLKR {
        REFCLKR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 11 - Lock Bypass"]
    #[inline]
    pub fn lbypass(&self) -> LBYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LBYPASSR { bits }
    }
    #[doc = "Bits 12:14 - Sigma-Delta DCO Filter Selection"]
    #[inline]
    pub fn dcofilter(&self) -> DCOFILTERR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCOFILTERR { bits }
    }
    #[doc = "Bit 15 - DCO Filter Enable"]
    #[inline]
    pub fn dcoen(&self) -> DCOENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCOENR { bits }
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
        W { bits: 32 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Proportional Integral Filter Selection"]
    #[inline]
    pub fn filter(&mut self) -> _FILTERW {
        _FILTERW { w: self }
    }
    #[doc = "Bit 4 - Wake Up Fast"]
    #[inline]
    pub fn wuf(&mut self) -> _WUFW {
        _WUFW { w: self }
    }
    #[doc = "Bits 5:7 - Reference Clock Selection"]
    #[inline]
    pub fn refclk(&mut self) -> _REFCLKW {
        _REFCLKW { w: self }
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline]
    pub fn ltime(&mut self) -> _LTIMEW {
        _LTIMEW { w: self }
    }
    #[doc = "Bit 11 - Lock Bypass"]
    #[inline]
    pub fn lbypass(&mut self) -> _LBYPASSW {
        _LBYPASSW { w: self }
    }
    #[doc = "Bits 12:14 - Sigma-Delta DCO Filter Selection"]
    #[inline]
    pub fn dcofilter(&mut self) -> _DCOFILTERW {
        _DCOFILTERW { w: self }
    }
    #[doc = "Bit 15 - DCO Filter Enable"]
    #[inline]
    pub fn dcoen(&mut self) -> _DCOENW {
        _DCOENW { w: self }
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline]
    pub fn div(&mut self) -> _DIVW {
        _DIVW { w: self }
    }
}
