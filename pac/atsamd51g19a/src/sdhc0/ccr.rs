#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CCR {
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
#[doc = "Possible values of the field `INTCLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTCLKENR {
    #[doc = "Stop"]
    OFF,
    #[doc = "Oscillate"]
    ON,
}
impl INTCLKENR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            INTCLKENR::OFF => false,
            INTCLKENR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTCLKENR {
        match value {
            false => INTCLKENR::OFF,
            true => INTCLKENR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == INTCLKENR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == INTCLKENR::ON
    }
}
#[doc = "Possible values of the field `INTCLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTCLKSR {
    #[doc = "Not Ready"]
    NOT_READY,
    #[doc = "Ready"]
    READY,
}
impl INTCLKSR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            INTCLKSR::NOT_READY => false,
            INTCLKSR::READY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTCLKSR {
        match value {
            false => INTCLKSR::NOT_READY,
            true => INTCLKSR::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline]
    pub fn is_not_ready(&self) -> bool {
        *self == INTCLKSR::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == INTCLKSR::READY
    }
}
#[doc = "Possible values of the field `SDCLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCLKENR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl SDCLKENR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SDCLKENR::DISABLE => false,
            SDCLKENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDCLKENR {
        match value {
            false => SDCLKENR::DISABLE,
            true => SDCLKENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SDCLKENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SDCLKENR::ENABLE
    }
}
#[doc = "Possible values of the field `CLKGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKGSELR {
    #[doc = "Divided Clock Mode"]
    DIV,
    #[doc = "Programmable Clock Mode"]
    PROG,
}
impl CLKGSELR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CLKGSELR::DIV => false,
            CLKGSELR::PROG => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKGSELR {
        match value {
            false => CLKGSELR::DIV,
            true => CLKGSELR::PROG,
        }
    }
    #[doc = "Checks if the value of the field is `DIV`"]
    #[inline]
    pub fn is_div(&self) -> bool {
        *self == CLKGSELR::DIV
    }
    #[doc = "Checks if the value of the field is `PROG`"]
    #[inline]
    pub fn is_prog(&self) -> bool {
        *self == CLKGSELR::PROG
    }
}
#[doc = r" Value of the field"]
pub struct USDCLKFSELR {
    bits: u8,
}
impl USDCLKFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SDCLKFSELR {
    bits: u8,
}
impl SDCLKFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `INTCLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTCLKENW {
    #[doc = "Stop"]
    OFF,
    #[doc = "Oscillate"]
    ON,
}
impl INTCLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTCLKENW::OFF => false,
            INTCLKENW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _INTCLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTCLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(INTCLKENW::OFF)
    }
    #[doc = "Oscillate"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(INTCLKENW::ON)
    }
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
#[doc = "Values that can be written to the field `SDCLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCLKENW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl SDCLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDCLKENW::DISABLE => false,
            SDCLKENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDCLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDCLKENW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDCLKENW::ENABLE)
    }
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
#[doc = "Values that can be written to the field `CLKGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKGSELW {
    #[doc = "Divided Clock Mode"]
    DIV,
    #[doc = "Programmable Clock Mode"]
    PROG,
}
impl CLKGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKGSELW::DIV => false,
            CLKGSELW::PROG => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Divided Clock Mode"]
    #[inline]
    pub fn div(self) -> &'a mut W {
        self.variant(CLKGSELW::DIV)
    }
    #[doc = "Programmable Clock Mode"]
    #[inline]
    pub fn prog(self) -> &'a mut W {
        self.variant(CLKGSELW::PROG)
    }
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
        self.w.bits &= !(0x01 << 5);
        self.w.bits |= ((value as u16) & 0x01) << 5;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USDCLKFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _USDCLKFSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 6);
        self.w.bits |= ((value as u16) & 0x03) << 6;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SDCLKFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCLKFSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0xff << 8);
        self.w.bits |= ((value as u16) & 0xff) << 8;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline]
    pub fn intclken(&self) -> INTCLKENR {
        INTCLKENR::_from(((self.bits >> 0) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline]
    pub fn intclks(&self) -> INTCLKSR {
        INTCLKSR::_from(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline]
    pub fn sdclken(&self) -> SDCLKENR {
        SDCLKENR::_from(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline]
    pub fn clkgsel(&self) -> CLKGSELR {
        CLKGSELR::_from(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Upper Bits of SDCLK Frequency Select"]
    #[inline]
    pub fn usdclkfsel(&self) -> USDCLKFSELR {
        let bits = ((self.bits >> 6) & 0x03) as u8;
        USDCLKFSELR { bits }
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline]
    pub fn sdclkfsel(&self) -> SDCLKFSELR {
        let bits = ((self.bits >> 8) & 0xff) as u8;
        SDCLKFSELR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline]
    pub fn intclken(&mut self) -> _INTCLKENW {
        _INTCLKENW { w: self }
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline]
    pub fn sdclken(&mut self) -> _SDCLKENW {
        _SDCLKENW { w: self }
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline]
    pub fn clkgsel(&mut self) -> _CLKGSELW {
        _CLKGSELW { w: self }
    }
    #[doc = "Bits 6:7 - Upper Bits of SDCLK Frequency Select"]
    #[inline]
    pub fn usdclkfsel(&mut self) -> _USDCLKFSELW {
        _USDCLKFSELW { w: self }
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline]
    pub fn sdclkfsel(&mut self) -> _SDCLKFSELW {
        _SDCLKFSELW { w: self }
    }
}
