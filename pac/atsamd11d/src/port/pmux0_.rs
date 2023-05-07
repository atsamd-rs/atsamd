#[doc = "Register `PMUX0_%s` reader"]
pub struct R(crate::R<PMUX0__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMUX0__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMUX0__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMUX0__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMUX0_%s` writer"]
pub struct W(crate::W<PMUX0__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMUX0__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PMUX0__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMUX0__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMUXE` reader - Peripheral Multiplexing Even"]
pub type PMUXE_R = crate::FieldReader<u8, PMUXESELECT_A>;
#[doc = "Peripheral Multiplexing Even\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PMUXESELECT_A {
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
}
impl From<PMUXESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PMUXESELECT_A) -> Self {
        variant as _
    }
}
impl PMUXE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PMUXESELECT_A> {
        match self.bits {
            0 => Some(PMUXESELECT_A::A),
            1 => Some(PMUXESELECT_A::B),
            2 => Some(PMUXESELECT_A::C),
            3 => Some(PMUXESELECT_A::D),
            4 => Some(PMUXESELECT_A::E),
            5 => Some(PMUXESELECT_A::F),
            6 => Some(PMUXESELECT_A::G),
            7 => Some(PMUXESELECT_A::H),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == PMUXESELECT_A::A
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == PMUXESELECT_A::B
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == PMUXESELECT_A::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == PMUXESELECT_A::D
    }
    #[doc = "Checks if the value of the field is `E`"]
    #[inline(always)]
    pub fn is_e(&self) -> bool {
        *self == PMUXESELECT_A::E
    }
    #[doc = "Checks if the value of the field is `F`"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        *self == PMUXESELECT_A::F
    }
    #[doc = "Checks if the value of the field is `G`"]
    #[inline(always)]
    pub fn is_g(&self) -> bool {
        *self == PMUXESELECT_A::G
    }
    #[doc = "Checks if the value of the field is `H`"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == PMUXESELECT_A::H
    }
}
#[doc = "Field `PMUXE` writer - Peripheral Multiplexing Even"]
pub type PMUXE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, PMUX0__SPEC, u8, PMUXESELECT_A, 4, O>;
impl<'a, const O: u8> PMUXE_W<'a, O> {
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(PMUXESELECT_A::A)
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn b(self) -> &'a mut W {
        self.variant(PMUXESELECT_A::B)
    }
    #[doc = "Peripheral function C selected"]
    #[inline(always)]
    pub fn c(self) -> &'a mut W {
        self.variant(PMUXESELECT_A::C)
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut W {
        self.variant(PMUXESELECT_A::D)
    }
    #[doc = "Peripheral function E selected"]
    #[inline(always)]
    pub fn e(self) -> &'a mut W {
        self.variant(PMUXESELECT_A::E)
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn f(self) -> &'a mut W {
        self.variant(PMUXESELECT_A::F)
    }
    #[doc = "Peripheral function G selected"]
    #[inline(always)]
    pub fn g(self) -> &'a mut W {
        self.variant(PMUXESELECT_A::G)
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn h(self) -> &'a mut W {
        self.variant(PMUXESELECT_A::H)
    }
}
#[doc = "Field `PMUXO` reader - Peripheral Multiplexing Odd"]
pub type PMUXO_R = crate::FieldReader<u8, PMUXOSELECT_A>;
#[doc = "Peripheral Multiplexing Odd\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PMUXOSELECT_A {
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
}
impl From<PMUXOSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PMUXOSELECT_A) -> Self {
        variant as _
    }
}
impl PMUXO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PMUXOSELECT_A> {
        match self.bits {
            0 => Some(PMUXOSELECT_A::A),
            1 => Some(PMUXOSELECT_A::B),
            2 => Some(PMUXOSELECT_A::C),
            3 => Some(PMUXOSELECT_A::D),
            4 => Some(PMUXOSELECT_A::E),
            5 => Some(PMUXOSELECT_A::F),
            6 => Some(PMUXOSELECT_A::G),
            7 => Some(PMUXOSELECT_A::H),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == PMUXOSELECT_A::A
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == PMUXOSELECT_A::B
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == PMUXOSELECT_A::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == PMUXOSELECT_A::D
    }
    #[doc = "Checks if the value of the field is `E`"]
    #[inline(always)]
    pub fn is_e(&self) -> bool {
        *self == PMUXOSELECT_A::E
    }
    #[doc = "Checks if the value of the field is `F`"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        *self == PMUXOSELECT_A::F
    }
    #[doc = "Checks if the value of the field is `G`"]
    #[inline(always)]
    pub fn is_g(&self) -> bool {
        *self == PMUXOSELECT_A::G
    }
    #[doc = "Checks if the value of the field is `H`"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == PMUXOSELECT_A::H
    }
}
#[doc = "Field `PMUXO` writer - Peripheral Multiplexing Odd"]
pub type PMUXO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, PMUX0__SPEC, u8, PMUXOSELECT_A, 4, O>;
impl<'a, const O: u8> PMUXO_W<'a, O> {
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(PMUXOSELECT_A::A)
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn b(self) -> &'a mut W {
        self.variant(PMUXOSELECT_A::B)
    }
    #[doc = "Peripheral function C selected"]
    #[inline(always)]
    pub fn c(self) -> &'a mut W {
        self.variant(PMUXOSELECT_A::C)
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut W {
        self.variant(PMUXOSELECT_A::D)
    }
    #[doc = "Peripheral function E selected"]
    #[inline(always)]
    pub fn e(self) -> &'a mut W {
        self.variant(PMUXOSELECT_A::E)
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn f(self) -> &'a mut W {
        self.variant(PMUXOSELECT_A::F)
    }
    #[doc = "Peripheral function G selected"]
    #[inline(always)]
    pub fn g(self) -> &'a mut W {
        self.variant(PMUXOSELECT_A::G)
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn h(self) -> &'a mut W {
        self.variant(PMUXOSELECT_A::H)
    }
}
impl R {
    #[doc = "Bits 0:3 - Peripheral Multiplexing Even"]
    #[inline(always)]
    pub fn pmuxe(&self) -> PMUXE_R {
        PMUXE_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing Odd"]
    #[inline(always)]
    pub fn pmuxo(&self) -> PMUXO_R {
        PMUXO_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Peripheral Multiplexing Even"]
    #[inline(always)]
    #[must_use]
    pub fn pmuxe(&mut self) -> PMUXE_W<0> {
        PMUXE_W::new(self)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing Odd"]
    #[inline(always)]
    #[must_use]
    pub fn pmuxo(&mut self) -> PMUXO_W<4> {
        PMUXO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Multiplexing n - Group 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmux0_](index.html) module"]
pub struct PMUX0__SPEC;
impl crate::RegisterSpec for PMUX0__SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pmux0_::R](R) reader structure"]
impl crate::Readable for PMUX0__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmux0_::W](W) writer structure"]
impl crate::Writable for PMUX0__SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMUX0_%s to value 0"]
impl crate::Resettable for PMUX0__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
