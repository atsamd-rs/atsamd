#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::PVR {
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
pub struct SDCLKFSELR {
    bits: u16,
}
impl SDCLKFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `CLKGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKGSELR {
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator (Divider)"]
    DIV,
    #[doc = "Programmable Clock Generator"]
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
#[doc = "Possible values of the field `DRVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRVSELR {
    #[doc = "Driver Type B is Selected"]
    B,
    #[doc = "Driver Type A is Selected"]
    A,
    #[doc = "Driver Type C is Selected"]
    C,
    #[doc = "Driver Type D is Selected"]
    D,
}
impl DRVSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DRVSELR::B => 0,
            DRVSELR::A => 0x01,
            DRVSELR::C => 0x02,
            DRVSELR::D => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DRVSELR {
        match value {
            0 => DRVSELR::B,
            1 => DRVSELR::A,
            2 => DRVSELR::C,
            3 => DRVSELR::D,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline]
    pub fn is_b(&self) -> bool {
        *self == DRVSELR::B
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline]
    pub fn is_a(&self) -> bool {
        *self == DRVSELR::A
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline]
    pub fn is_c(&self) -> bool {
        *self == DRVSELR::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline]
    pub fn is_d(&self) -> bool {
        *self == DRVSELR::D
    }
}
#[doc = r" Proxy"]
pub struct _SDCLKFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCLKFSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(0x03ff << 0);
        self.w.bits |= ((value as u16) & 0x03ff) << 0;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKGSELW {
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator (Divider)"]
    DIV,
    #[doc = "Programmable Clock Generator"]
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
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator (Divider)"]
    #[inline]
    pub fn div(self) -> &'a mut W {
        self.variant(CLKGSELW::DIV)
    }
    #[doc = "Programmable Clock Generator"]
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
        self.w.bits &= !(0x01 << 10);
        self.w.bits |= ((value as u16) & 0x01) << 10;
        self.w
    }
}
#[doc = "Values that can be written to the field `DRVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRVSELW {
    #[doc = "Driver Type B is Selected"]
    B,
    #[doc = "Driver Type A is Selected"]
    A,
    #[doc = "Driver Type C is Selected"]
    C,
    #[doc = "Driver Type D is Selected"]
    D,
}
impl DRVSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DRVSELW::B => 0,
            DRVSELW::A => 1,
            DRVSELW::C => 2,
            DRVSELW::D => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DRVSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRVSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Driver Type B is Selected"]
    #[inline]
    pub fn b(self) -> &'a mut W {
        self.variant(DRVSELW::B)
    }
    #[doc = "Driver Type A is Selected"]
    #[inline]
    pub fn a(self) -> &'a mut W {
        self.variant(DRVSELW::A)
    }
    #[doc = "Driver Type C is Selected"]
    #[inline]
    pub fn c(self) -> &'a mut W {
        self.variant(DRVSELW::C)
    }
    #[doc = "Driver Type D is Selected"]
    #[inline]
    pub fn d(self) -> &'a mut W {
        self.variant(DRVSELW::D)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 14);
        self.w.bits |= ((value as u16) & 0x03) << 14;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:9 - SDCLK Frequency Select Value for Initialization"]
    #[inline]
    pub fn sdclkfsel(&self) -> SDCLKFSELR {
        let bits = ((self.bits >> 0) & 0x03ff) as u16;
        SDCLKFSELR { bits }
    }
    #[doc = "Bit 10 - Clock Generator Select Value for Initialization"]
    #[inline]
    pub fn clkgsel(&self) -> CLKGSELR {
        CLKGSELR::_from(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for Initialization"]
    #[inline]
    pub fn drvsel(&self) -> DRVSELR {
        DRVSELR::_from(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - SDCLK Frequency Select Value for Initialization"]
    #[inline]
    pub fn sdclkfsel(&mut self) -> _SDCLKFSELW {
        _SDCLKFSELW { w: self }
    }
    #[doc = "Bit 10 - Clock Generator Select Value for Initialization"]
    #[inline]
    pub fn clkgsel(&mut self) -> _CLKGSELW {
        _CLKGSELW { w: self }
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for Initialization"]
    #[inline]
    pub fn drvsel(&mut self) -> _DRVSELW {
        _DRVSELW { w: self }
    }
}
