#[doc = "Reader of register PMUX0_%s"]
pub type R = crate::R<u8, super::PMUX0_>;
#[doc = "Writer for register PMUX0_%s"]
pub type W = crate::W<u8, super::PMUX0_>;
#[doc = "Register PMUX0_%s `reset()`'s with value 0"]
impl crate::ResetValue for super::PMUX0_ {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `PMUXE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMUXE_A {
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
impl crate::ToBits<u8> for PMUXE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PMUXE_A::A => 0,
            PMUXE_A::B => 1,
            PMUXE_A::C => 2,
            PMUXE_A::D => 3,
            PMUXE_A::E => 4,
            PMUXE_A::F => 5,
            PMUXE_A::G => 6,
            PMUXE_A::H => 7,
        }
    }
}
#[doc = "Reader of field `PMUXE`"]
pub type PMUXE_R = crate::R<u8, PMUXE_A>;
impl PMUXE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PMUXE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PMUXE_A::A),
            1 => Val(PMUXE_A::B),
            2 => Val(PMUXE_A::C),
            3 => Val(PMUXE_A::D),
            4 => Val(PMUXE_A::E),
            5 => Val(PMUXE_A::F),
            6 => Val(PMUXE_A::G),
            7 => Val(PMUXE_A::H),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == PMUXE_A::A
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == PMUXE_A::B
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == PMUXE_A::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == PMUXE_A::D
    }
    #[doc = "Checks if the value of the field is `E`"]
    #[inline(always)]
    pub fn is_e(&self) -> bool {
        *self == PMUXE_A::E
    }
    #[doc = "Checks if the value of the field is `F`"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        *self == PMUXE_A::F
    }
    #[doc = "Checks if the value of the field is `G`"]
    #[inline(always)]
    pub fn is_g(&self) -> bool {
        *self == PMUXE_A::G
    }
    #[doc = "Checks if the value of the field is `H`"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == PMUXE_A::H
    }
}
#[doc = "Write proxy for field `PMUXE`"]
pub struct PMUXE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMUXE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMUXE_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(PMUXE_A::A)
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn b(self) -> &'a mut W {
        self.variant(PMUXE_A::B)
    }
    #[doc = "Peripheral function C selected"]
    #[inline(always)]
    pub fn c(self) -> &'a mut W {
        self.variant(PMUXE_A::C)
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut W {
        self.variant(PMUXE_A::D)
    }
    #[doc = "Peripheral function E selected"]
    #[inline(always)]
    pub fn e(self) -> &'a mut W {
        self.variant(PMUXE_A::E)
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn f(self) -> &'a mut W {
        self.variant(PMUXE_A::F)
    }
    #[doc = "Peripheral function G selected"]
    #[inline(always)]
    pub fn g(self) -> &'a mut W {
        self.variant(PMUXE_A::G)
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn h(self) -> &'a mut W {
        self.variant(PMUXE_A::H)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Possible values of the field `PMUXO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMUXO_A {
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
impl crate::ToBits<u8> for PMUXO_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PMUXO_A::A => 0,
            PMUXO_A::B => 1,
            PMUXO_A::C => 2,
            PMUXO_A::D => 3,
            PMUXO_A::E => 4,
            PMUXO_A::F => 5,
            PMUXO_A::G => 6,
            PMUXO_A::H => 7,
        }
    }
}
#[doc = "Reader of field `PMUXO`"]
pub type PMUXO_R = crate::R<u8, PMUXO_A>;
impl PMUXO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PMUXO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PMUXO_A::A),
            1 => Val(PMUXO_A::B),
            2 => Val(PMUXO_A::C),
            3 => Val(PMUXO_A::D),
            4 => Val(PMUXO_A::E),
            5 => Val(PMUXO_A::F),
            6 => Val(PMUXO_A::G),
            7 => Val(PMUXO_A::H),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == PMUXO_A::A
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == PMUXO_A::B
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == PMUXO_A::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == PMUXO_A::D
    }
    #[doc = "Checks if the value of the field is `E`"]
    #[inline(always)]
    pub fn is_e(&self) -> bool {
        *self == PMUXO_A::E
    }
    #[doc = "Checks if the value of the field is `F`"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        *self == PMUXO_A::F
    }
    #[doc = "Checks if the value of the field is `G`"]
    #[inline(always)]
    pub fn is_g(&self) -> bool {
        *self == PMUXO_A::G
    }
    #[doc = "Checks if the value of the field is `H`"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == PMUXO_A::H
    }
}
#[doc = "Write proxy for field `PMUXO`"]
pub struct PMUXO_W<'a> {
    w: &'a mut W,
}
impl<'a> PMUXO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMUXO_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(PMUXO_A::A)
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn b(self) -> &'a mut W {
        self.variant(PMUXO_A::B)
    }
    #[doc = "Peripheral function C selected"]
    #[inline(always)]
    pub fn c(self) -> &'a mut W {
        self.variant(PMUXO_A::C)
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut W {
        self.variant(PMUXO_A::D)
    }
    #[doc = "Peripheral function E selected"]
    #[inline(always)]
    pub fn e(self) -> &'a mut W {
        self.variant(PMUXO_A::E)
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn f(self) -> &'a mut W {
        self.variant(PMUXO_A::F)
    }
    #[doc = "Peripheral function G selected"]
    #[inline(always)]
    pub fn g(self) -> &'a mut W {
        self.variant(PMUXO_A::G)
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn h(self) -> &'a mut W {
        self.variant(PMUXO_A::H)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u8) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Peripheral Multiplexing Even"]
    #[inline(always)]
    pub fn pmuxe(&self) -> PMUXE_R {
        PMUXE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing Odd"]
    #[inline(always)]
    pub fn pmuxo(&self) -> PMUXO_R {
        PMUXO_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Peripheral Multiplexing Even"]
    #[inline(always)]
    pub fn pmuxe(&mut self) -> PMUXE_W {
        PMUXE_W { w: self }
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing Odd"]
    #[inline(always)]
    pub fn pmuxo(&mut self) -> PMUXO_W {
        PMUXO_W { w: self }
    }
}
