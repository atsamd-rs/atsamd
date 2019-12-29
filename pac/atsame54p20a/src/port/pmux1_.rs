#[doc = "Reader of register PMUX1_%s"]
pub type R = crate::R<u8, super::PMUX1_>;
#[doc = "Writer for register PMUX1_%s"]
pub type W = crate::W<u8, super::PMUX1_>;
#[doc = "Register PMUX1_%s `reset()`'s with value 0"]
impl crate::ResetValue for super::PMUX1_ {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Peripheral Multiplexing for Even-Numbered Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PMUXE_A {
    #[doc = "0: Peripheral function A selected"]
    A = 0,
    #[doc = "1: Peripheral function B selected"]
    B = 1,
    #[doc = "2: Peripheral function C selected"]
    C = 2,
    #[doc = "3: Peripheral function D selected"]
    D = 3,
    #[doc = "4: Peripheral function E selected"]
    E = 4,
    #[doc = "5: Peripheral function F selected"]
    F = 5,
    #[doc = "6: Peripheral function G selected"]
    G = 6,
    #[doc = "7: Peripheral function H selected"]
    H = 7,
    #[doc = "8: Peripheral function I selected"]
    I = 8,
    #[doc = "9: Peripheral function J selected"]
    J = 9,
    #[doc = "10: Peripheral function K selected"]
    K = 10,
    #[doc = "11: Peripheral function L selected"]
    L = 11,
    #[doc = "12: Peripheral function M selected"]
    M = 12,
    #[doc = "13: Peripheral function N selected"]
    N = 13,
}
impl From<PMUXE_A> for u8 {
    #[inline(always)]
    fn from(variant: PMUXE_A) -> Self {
        variant as _
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
            8 => Val(PMUXE_A::I),
            9 => Val(PMUXE_A::J),
            10 => Val(PMUXE_A::K),
            11 => Val(PMUXE_A::L),
            12 => Val(PMUXE_A::M),
            13 => Val(PMUXE_A::N),
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
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == PMUXE_A::I
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        *self == PMUXE_A::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == PMUXE_A::K
    }
    #[doc = "Checks if the value of the field is `L`"]
    #[inline(always)]
    pub fn is_l(&self) -> bool {
        *self == PMUXE_A::L
    }
    #[doc = "Checks if the value of the field is `M`"]
    #[inline(always)]
    pub fn is_m(&self) -> bool {
        *self == PMUXE_A::M
    }
    #[doc = "Checks if the value of the field is `N`"]
    #[inline(always)]
    pub fn is_n(&self) -> bool {
        *self == PMUXE_A::N
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
        unsafe { self.bits(variant.into()) }
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
    #[doc = "Peripheral function I selected"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(PMUXE_A::I)
    }
    #[doc = "Peripheral function J selected"]
    #[inline(always)]
    pub fn j(self) -> &'a mut W {
        self.variant(PMUXE_A::J)
    }
    #[doc = "Peripheral function K selected"]
    #[inline(always)]
    pub fn k(self) -> &'a mut W {
        self.variant(PMUXE_A::K)
    }
    #[doc = "Peripheral function L selected"]
    #[inline(always)]
    pub fn l(self) -> &'a mut W {
        self.variant(PMUXE_A::L)
    }
    #[doc = "Peripheral function M selected"]
    #[inline(always)]
    pub fn m(self) -> &'a mut W {
        self.variant(PMUXE_A::M)
    }
    #[doc = "Peripheral function N selected"]
    #[inline(always)]
    pub fn n(self) -> &'a mut W {
        self.variant(PMUXE_A::N)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Peripheral Multiplexing for Odd-Numbered Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PMUXO_A {
    #[doc = "0: Peripheral function A selected"]
    A = 0,
    #[doc = "1: Peripheral function B selected"]
    B = 1,
    #[doc = "2: Peripheral function C selected"]
    C = 2,
    #[doc = "3: Peripheral function D selected"]
    D = 3,
    #[doc = "4: Peripheral function E selected"]
    E = 4,
    #[doc = "5: Peripheral function F selected"]
    F = 5,
    #[doc = "6: Peripheral function G selected"]
    G = 6,
    #[doc = "7: Peripheral function H selected"]
    H = 7,
    #[doc = "8: Peripheral function I selected"]
    I = 8,
    #[doc = "9: Peripheral function J selected"]
    J = 9,
    #[doc = "10: Peripheral function K selected"]
    K = 10,
    #[doc = "11: Peripheral function L selected"]
    L = 11,
    #[doc = "12: Peripheral function M selected"]
    M = 12,
    #[doc = "13: Peripheral function N selected"]
    N = 13,
}
impl From<PMUXO_A> for u8 {
    #[inline(always)]
    fn from(variant: PMUXO_A) -> Self {
        variant as _
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
            8 => Val(PMUXO_A::I),
            9 => Val(PMUXO_A::J),
            10 => Val(PMUXO_A::K),
            11 => Val(PMUXO_A::L),
            12 => Val(PMUXO_A::M),
            13 => Val(PMUXO_A::N),
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
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == PMUXO_A::I
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        *self == PMUXO_A::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == PMUXO_A::K
    }
    #[doc = "Checks if the value of the field is `L`"]
    #[inline(always)]
    pub fn is_l(&self) -> bool {
        *self == PMUXO_A::L
    }
    #[doc = "Checks if the value of the field is `M`"]
    #[inline(always)]
    pub fn is_m(&self) -> bool {
        *self == PMUXO_A::M
    }
    #[doc = "Checks if the value of the field is `N`"]
    #[inline(always)]
    pub fn is_n(&self) -> bool {
        *self == PMUXO_A::N
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
        unsafe { self.bits(variant.into()) }
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
    #[doc = "Peripheral function I selected"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(PMUXO_A::I)
    }
    #[doc = "Peripheral function J selected"]
    #[inline(always)]
    pub fn j(self) -> &'a mut W {
        self.variant(PMUXO_A::J)
    }
    #[doc = "Peripheral function K selected"]
    #[inline(always)]
    pub fn k(self) -> &'a mut W {
        self.variant(PMUXO_A::K)
    }
    #[doc = "Peripheral function L selected"]
    #[inline(always)]
    pub fn l(self) -> &'a mut W {
        self.variant(PMUXO_A::L)
    }
    #[doc = "Peripheral function M selected"]
    #[inline(always)]
    pub fn m(self) -> &'a mut W {
        self.variant(PMUXO_A::M)
    }
    #[doc = "Peripheral function N selected"]
    #[inline(always)]
    pub fn n(self) -> &'a mut W {
        self.variant(PMUXO_A::N)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u8) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxe(&self) -> PMUXE_R {
        PMUXE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxo(&self) -> PMUXO_R {
        PMUXO_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxe(&mut self) -> PMUXE_W {
        PMUXE_W { w: self }
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxo(&mut self) -> PMUXO_W {
        PMUXO_W { w: self }
    }
}
