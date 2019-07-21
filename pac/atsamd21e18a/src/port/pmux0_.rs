#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PMUX0_ {
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
    pub const fn reset_value() -> u8 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `PMUXE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMUXER {
    #[doc = "Peripheral function A selected"]
    A,
    #[doc = "Peripheral function B selected"]
    B,
    #[doc = "Peripheral function C selected"]
    C,
    #[doc = "Peripheral function D selected"]
    D,
    #[doc = "Peripheral function E selected"]
    E,
    #[doc = "Peripheral function F selected"]
    F,
    #[doc = "Peripheral function G selected"]
    G,
    #[doc = "Peripheral function H selected"]
    H,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PMUXER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PMUXER::A => 0,
            PMUXER::B => 0x01,
            PMUXER::C => 0x02,
            PMUXER::D => 0x03,
            PMUXER::E => 0x04,
            PMUXER::F => 0x05,
            PMUXER::G => 0x06,
            PMUXER::H => 0x07,
            PMUXER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PMUXER {
        match value {
            0 => PMUXER::A,
            1 => PMUXER::B,
            2 => PMUXER::C,
            3 => PMUXER::D,
            4 => PMUXER::E,
            5 => PMUXER::F,
            6 => PMUXER::G,
            7 => PMUXER::H,
            i => PMUXER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline]
    pub fn is_a(&self) -> bool {
        *self == PMUXER::A
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline]
    pub fn is_b(&self) -> bool {
        *self == PMUXER::B
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline]
    pub fn is_c(&self) -> bool {
        *self == PMUXER::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline]
    pub fn is_d(&self) -> bool {
        *self == PMUXER::D
    }
    #[doc = "Checks if the value of the field is `E`"]
    #[inline]
    pub fn is_e(&self) -> bool {
        *self == PMUXER::E
    }
    #[doc = "Checks if the value of the field is `F`"]
    #[inline]
    pub fn is_f(&self) -> bool {
        *self == PMUXER::F
    }
    #[doc = "Checks if the value of the field is `G`"]
    #[inline]
    pub fn is_g(&self) -> bool {
        *self == PMUXER::G
    }
    #[doc = "Checks if the value of the field is `H`"]
    #[inline]
    pub fn is_h(&self) -> bool {
        *self == PMUXER::H
    }
}
#[doc = "Possible values of the field `PMUXO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMUXOR {
    #[doc = "Peripheral function A selected"]
    A,
    #[doc = "Peripheral function B selected"]
    B,
    #[doc = "Peripheral function C selected"]
    C,
    #[doc = "Peripheral function D selected"]
    D,
    #[doc = "Peripheral function E selected"]
    E,
    #[doc = "Peripheral function F selected"]
    F,
    #[doc = "Peripheral function G selected"]
    G,
    #[doc = "Peripheral function H selected"]
    H,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PMUXOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PMUXOR::A => 0,
            PMUXOR::B => 0x01,
            PMUXOR::C => 0x02,
            PMUXOR::D => 0x03,
            PMUXOR::E => 0x04,
            PMUXOR::F => 0x05,
            PMUXOR::G => 0x06,
            PMUXOR::H => 0x07,
            PMUXOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PMUXOR {
        match value {
            0 => PMUXOR::A,
            1 => PMUXOR::B,
            2 => PMUXOR::C,
            3 => PMUXOR::D,
            4 => PMUXOR::E,
            5 => PMUXOR::F,
            6 => PMUXOR::G,
            7 => PMUXOR::H,
            i => PMUXOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline]
    pub fn is_a(&self) -> bool {
        *self == PMUXOR::A
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline]
    pub fn is_b(&self) -> bool {
        *self == PMUXOR::B
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline]
    pub fn is_c(&self) -> bool {
        *self == PMUXOR::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline]
    pub fn is_d(&self) -> bool {
        *self == PMUXOR::D
    }
    #[doc = "Checks if the value of the field is `E`"]
    #[inline]
    pub fn is_e(&self) -> bool {
        *self == PMUXOR::E
    }
    #[doc = "Checks if the value of the field is `F`"]
    #[inline]
    pub fn is_f(&self) -> bool {
        *self == PMUXOR::F
    }
    #[doc = "Checks if the value of the field is `G`"]
    #[inline]
    pub fn is_g(&self) -> bool {
        *self == PMUXOR::G
    }
    #[doc = "Checks if the value of the field is `H`"]
    #[inline]
    pub fn is_h(&self) -> bool {
        *self == PMUXOR::H
    }
}
#[doc = "Values that can be written to the field `PMUXE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMUXEW {
    #[doc = "Peripheral function A selected"]
    A,
    #[doc = "Peripheral function B selected"]
    B,
    #[doc = "Peripheral function C selected"]
    C,
    #[doc = "Peripheral function D selected"]
    D,
    #[doc = "Peripheral function E selected"]
    E,
    #[doc = "Peripheral function F selected"]
    F,
    #[doc = "Peripheral function G selected"]
    G,
    #[doc = "Peripheral function H selected"]
    H,
}
impl PMUXEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PMUXEW::A => 0,
            PMUXEW::B => 1,
            PMUXEW::C => 2,
            PMUXEW::D => 3,
            PMUXEW::E => 4,
            PMUXEW::F => 5,
            PMUXEW::G => 6,
            PMUXEW::H => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMUXEW<'a> {
    w: &'a mut W,
}
impl<'a> _PMUXEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMUXEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Peripheral function A selected"]
    #[inline]
    pub fn a(self) -> &'a mut W {
        self.variant(PMUXEW::A)
    }
    #[doc = "Peripheral function B selected"]
    #[inline]
    pub fn b(self) -> &'a mut W {
        self.variant(PMUXEW::B)
    }
    #[doc = "Peripheral function C selected"]
    #[inline]
    pub fn c(self) -> &'a mut W {
        self.variant(PMUXEW::C)
    }
    #[doc = "Peripheral function D selected"]
    #[inline]
    pub fn d(self) -> &'a mut W {
        self.variant(PMUXEW::D)
    }
    #[doc = "Peripheral function E selected"]
    #[inline]
    pub fn e(self) -> &'a mut W {
        self.variant(PMUXEW::E)
    }
    #[doc = "Peripheral function F selected"]
    #[inline]
    pub fn f(self) -> &'a mut W {
        self.variant(PMUXEW::F)
    }
    #[doc = "Peripheral function G selected"]
    #[inline]
    pub fn g(self) -> &'a mut W {
        self.variant(PMUXEW::G)
    }
    #[doc = "Peripheral function H selected"]
    #[inline]
    pub fn h(self) -> &'a mut W {
        self.variant(PMUXEW::H)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 0);
        self.w.bits |= ((value as u8) & 0x0f) << 0;
        self.w
    }
}
#[doc = "Values that can be written to the field `PMUXO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMUXOW {
    #[doc = "Peripheral function A selected"]
    A,
    #[doc = "Peripheral function B selected"]
    B,
    #[doc = "Peripheral function C selected"]
    C,
    #[doc = "Peripheral function D selected"]
    D,
    #[doc = "Peripheral function E selected"]
    E,
    #[doc = "Peripheral function F selected"]
    F,
    #[doc = "Peripheral function G selected"]
    G,
    #[doc = "Peripheral function H selected"]
    H,
}
impl PMUXOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PMUXOW::A => 0,
            PMUXOW::B => 1,
            PMUXOW::C => 2,
            PMUXOW::D => 3,
            PMUXOW::E => 4,
            PMUXOW::F => 5,
            PMUXOW::G => 6,
            PMUXOW::H => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMUXOW<'a> {
    w: &'a mut W,
}
impl<'a> _PMUXOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMUXOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Peripheral function A selected"]
    #[inline]
    pub fn a(self) -> &'a mut W {
        self.variant(PMUXOW::A)
    }
    #[doc = "Peripheral function B selected"]
    #[inline]
    pub fn b(self) -> &'a mut W {
        self.variant(PMUXOW::B)
    }
    #[doc = "Peripheral function C selected"]
    #[inline]
    pub fn c(self) -> &'a mut W {
        self.variant(PMUXOW::C)
    }
    #[doc = "Peripheral function D selected"]
    #[inline]
    pub fn d(self) -> &'a mut W {
        self.variant(PMUXOW::D)
    }
    #[doc = "Peripheral function E selected"]
    #[inline]
    pub fn e(self) -> &'a mut W {
        self.variant(PMUXOW::E)
    }
    #[doc = "Peripheral function F selected"]
    #[inline]
    pub fn f(self) -> &'a mut W {
        self.variant(PMUXOW::F)
    }
    #[doc = "Peripheral function G selected"]
    #[inline]
    pub fn g(self) -> &'a mut W {
        self.variant(PMUXOW::G)
    }
    #[doc = "Peripheral function H selected"]
    #[inline]
    pub fn h(self) -> &'a mut W {
        self.variant(PMUXOW::H)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 4);
        self.w.bits |= ((value as u8) & 0x0f) << 4;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:3 - Peripheral Multiplexing Even"]
    #[inline]
    pub fn pmuxe(&self) -> PMUXER {
        PMUXER::_from(((self.bits >> 0) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing Odd"]
    #[inline]
    pub fn pmuxo(&self) -> PMUXOR {
        PMUXOR::_from(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Peripheral Multiplexing Even"]
    #[inline]
    pub fn pmuxe(&mut self) -> _PMUXEW {
        _PMUXEW { w: self }
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing Odd"]
    #[inline]
    pub fn pmuxo(&mut self) -> _PMUXOW {
        _PMUXOW { w: self }
    }
}
