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
    #[doc = "Peripheral function I selected"]
    I,
    #[doc = "Peripheral function J selected"]
    J,
    #[doc = "Peripheral function K selected"]
    K,
    #[doc = "Peripheral function L selected"]
    L,
    #[doc = "Peripheral function M selected"]
    M,
    #[doc = "Peripheral function N selected"]
    N,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PMUXER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PMUXER::A => 0,
            PMUXER::B => 1,
            PMUXER::C => 2,
            PMUXER::D => 3,
            PMUXER::E => 4,
            PMUXER::F => 5,
            PMUXER::G => 6,
            PMUXER::H => 7,
            PMUXER::I => 8,
            PMUXER::J => 9,
            PMUXER::K => 10,
            PMUXER::L => 11,
            PMUXER::M => 12,
            PMUXER::N => 13,
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
            8 => PMUXER::I,
            9 => PMUXER::J,
            10 => PMUXER::K,
            11 => PMUXER::L,
            12 => PMUXER::M,
            13 => PMUXER::N,
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
    #[doc = "Checks if the value of the field is `I`"]
    #[inline]
    pub fn is_i(&self) -> bool {
        *self == PMUXER::I
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline]
    pub fn is_j(&self) -> bool {
        *self == PMUXER::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline]
    pub fn is_k(&self) -> bool {
        *self == PMUXER::K
    }
    #[doc = "Checks if the value of the field is `L`"]
    #[inline]
    pub fn is_l(&self) -> bool {
        *self == PMUXER::L
    }
    #[doc = "Checks if the value of the field is `M`"]
    #[inline]
    pub fn is_m(&self) -> bool {
        *self == PMUXER::M
    }
    #[doc = "Checks if the value of the field is `N`"]
    #[inline]
    pub fn is_n(&self) -> bool {
        *self == PMUXER::N
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
    #[doc = "Peripheral function I selected"]
    I,
    #[doc = "Peripheral function J selected"]
    J,
    #[doc = "Peripheral function K selected"]
    K,
    #[doc = "Peripheral function L selected"]
    L,
    #[doc = "Peripheral function M selected"]
    M,
    #[doc = "Peripheral function N selected"]
    N,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PMUXOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PMUXOR::A => 0,
            PMUXOR::B => 1,
            PMUXOR::C => 2,
            PMUXOR::D => 3,
            PMUXOR::E => 4,
            PMUXOR::F => 5,
            PMUXOR::G => 6,
            PMUXOR::H => 7,
            PMUXOR::I => 8,
            PMUXOR::J => 9,
            PMUXOR::K => 10,
            PMUXOR::L => 11,
            PMUXOR::M => 12,
            PMUXOR::N => 13,
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
            8 => PMUXOR::I,
            9 => PMUXOR::J,
            10 => PMUXOR::K,
            11 => PMUXOR::L,
            12 => PMUXOR::M,
            13 => PMUXOR::N,
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
    #[doc = "Checks if the value of the field is `I`"]
    #[inline]
    pub fn is_i(&self) -> bool {
        *self == PMUXOR::I
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline]
    pub fn is_j(&self) -> bool {
        *self == PMUXOR::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline]
    pub fn is_k(&self) -> bool {
        *self == PMUXOR::K
    }
    #[doc = "Checks if the value of the field is `L`"]
    #[inline]
    pub fn is_l(&self) -> bool {
        *self == PMUXOR::L
    }
    #[doc = "Checks if the value of the field is `M`"]
    #[inline]
    pub fn is_m(&self) -> bool {
        *self == PMUXOR::M
    }
    #[doc = "Checks if the value of the field is `N`"]
    #[inline]
    pub fn is_n(&self) -> bool {
        *self == PMUXOR::N
    }
}
#[doc = "Values that can be written to the field `PMUXE`"]
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
    #[doc = "Peripheral function I selected"]
    I,
    #[doc = "Peripheral function J selected"]
    J,
    #[doc = "Peripheral function K selected"]
    K,
    #[doc = "Peripheral function L selected"]
    L,
    #[doc = "Peripheral function M selected"]
    M,
    #[doc = "Peripheral function N selected"]
    N,
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
            PMUXEW::I => 8,
            PMUXEW::J => 9,
            PMUXEW::K => 10,
            PMUXEW::L => 11,
            PMUXEW::M => 12,
            PMUXEW::N => 13,
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
    #[doc = "Peripheral function I selected"]
    #[inline]
    pub fn i(self) -> &'a mut W {
        self.variant(PMUXEW::I)
    }
    #[doc = "Peripheral function J selected"]
    #[inline]
    pub fn j(self) -> &'a mut W {
        self.variant(PMUXEW::J)
    }
    #[doc = "Peripheral function K selected"]
    #[inline]
    pub fn k(self) -> &'a mut W {
        self.variant(PMUXEW::K)
    }
    #[doc = "Peripheral function L selected"]
    #[inline]
    pub fn l(self) -> &'a mut W {
        self.variant(PMUXEW::L)
    }
    #[doc = "Peripheral function M selected"]
    #[inline]
    pub fn m(self) -> &'a mut W {
        self.variant(PMUXEW::M)
    }
    #[doc = "Peripheral function N selected"]
    #[inline]
    pub fn n(self) -> &'a mut W {
        self.variant(PMUXEW::N)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PMUXO`"]
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
    #[doc = "Peripheral function I selected"]
    I,
    #[doc = "Peripheral function J selected"]
    J,
    #[doc = "Peripheral function K selected"]
    K,
    #[doc = "Peripheral function L selected"]
    L,
    #[doc = "Peripheral function M selected"]
    M,
    #[doc = "Peripheral function N selected"]
    N,
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
            PMUXOW::I => 8,
            PMUXOW::J => 9,
            PMUXOW::K => 10,
            PMUXOW::L => 11,
            PMUXOW::M => 12,
            PMUXOW::N => 13,
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
    #[doc = "Peripheral function I selected"]
    #[inline]
    pub fn i(self) -> &'a mut W {
        self.variant(PMUXOW::I)
    }
    #[doc = "Peripheral function J selected"]
    #[inline]
    pub fn j(self) -> &'a mut W {
        self.variant(PMUXOW::J)
    }
    #[doc = "Peripheral function K selected"]
    #[inline]
    pub fn k(self) -> &'a mut W {
        self.variant(PMUXOW::K)
    }
    #[doc = "Peripheral function L selected"]
    #[inline]
    pub fn l(self) -> &'a mut W {
        self.variant(PMUXOW::L)
    }
    #[doc = "Peripheral function M selected"]
    #[inline]
    pub fn m(self) -> &'a mut W {
        self.variant(PMUXOW::M)
    }
    #[doc = "Peripheral function N selected"]
    #[inline]
    pub fn n(self) -> &'a mut W {
        self.variant(PMUXOW::N)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline]
    pub fn pmuxe(&self) -> PMUXER {
        PMUXER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline]
    pub fn pmuxo(&self) -> PMUXOR {
        PMUXOR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline]
    pub fn pmuxe(&mut self) -> _PMUXEW {
        _PMUXEW { w: self }
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline]
    pub fn pmuxo(&mut self) -> _PMUXOW {
        _PMUXOW { w: self }
    }
}
