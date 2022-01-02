#[doc = "Register `PMUX1_%s` reader"]
pub struct R(crate::R<PMUX1__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMUX1__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMUX1__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMUX1__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMUX1_%s` writer"]
pub struct W(crate::W<PMUX1__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMUX1__SPEC>;
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
impl From<crate::W<PMUX1__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMUX1__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Peripheral Multiplexing Even\n\nValue on reset: 0"]
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
}
impl From<PMUXE_A> for u8 {
    #[inline(always)]
    fn from(variant: PMUXE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PMUXE` reader - Peripheral Multiplexing Even"]
pub struct PMUXE_R(crate::FieldReader<u8, PMUXE_A>);
impl PMUXE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PMUXE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PMUXE_A> {
        match self.bits {
            0 => Some(PMUXE_A::A),
            1 => Some(PMUXE_A::B),
            2 => Some(PMUXE_A::C),
            3 => Some(PMUXE_A::D),
            4 => Some(PMUXE_A::E),
            5 => Some(PMUXE_A::F),
            6 => Some(PMUXE_A::G),
            7 => Some(PMUXE_A::H),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        **self == PMUXE_A::A
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        **self == PMUXE_A::B
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        **self == PMUXE_A::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        **self == PMUXE_A::D
    }
    #[doc = "Checks if the value of the field is `E`"]
    #[inline(always)]
    pub fn is_e(&self) -> bool {
        **self == PMUXE_A::E
    }
    #[doc = "Checks if the value of the field is `F`"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        **self == PMUXE_A::F
    }
    #[doc = "Checks if the value of the field is `G`"]
    #[inline(always)]
    pub fn is_g(&self) -> bool {
        **self == PMUXE_A::G
    }
    #[doc = "Checks if the value of the field is `H`"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        **self == PMUXE_A::H
    }
}
impl core::ops::Deref for PMUXE_R {
    type Target = crate::FieldReader<u8, PMUXE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMUXE` writer - Peripheral Multiplexing Even"]
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Peripheral Multiplexing Odd\n\nValue on reset: 0"]
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
}
impl From<PMUXO_A> for u8 {
    #[inline(always)]
    fn from(variant: PMUXO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PMUXO` reader - Peripheral Multiplexing Odd"]
pub struct PMUXO_R(crate::FieldReader<u8, PMUXO_A>);
impl PMUXO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PMUXO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PMUXO_A> {
        match self.bits {
            0 => Some(PMUXO_A::A),
            1 => Some(PMUXO_A::B),
            2 => Some(PMUXO_A::C),
            3 => Some(PMUXO_A::D),
            4 => Some(PMUXO_A::E),
            5 => Some(PMUXO_A::F),
            6 => Some(PMUXO_A::G),
            7 => Some(PMUXO_A::H),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        **self == PMUXO_A::A
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        **self == PMUXO_A::B
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        **self == PMUXO_A::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        **self == PMUXO_A::D
    }
    #[doc = "Checks if the value of the field is `E`"]
    #[inline(always)]
    pub fn is_e(&self) -> bool {
        **self == PMUXO_A::E
    }
    #[doc = "Checks if the value of the field is `F`"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        **self == PMUXO_A::F
    }
    #[doc = "Checks if the value of the field is `G`"]
    #[inline(always)]
    pub fn is_g(&self) -> bool {
        **self == PMUXO_A::G
    }
    #[doc = "Checks if the value of the field is `H`"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        **self == PMUXO_A::H
    }
}
impl core::ops::Deref for PMUXO_R {
    type Target = crate::FieldReader<u8, PMUXO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMUXO` writer - Peripheral Multiplexing Odd"]
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u8 & 0x0f) << 4);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Multiplexing n - Group 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmux1_](index.html) module"]
pub struct PMUX1__SPEC;
impl crate::RegisterSpec for PMUX1__SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pmux1_::R](R) reader structure"]
impl crate::Readable for PMUX1__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmux1_::W](W) writer structure"]
impl crate::Writable for PMUX1__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMUX1_%s to value 0"]
impl crate::Resettable for PMUX1__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
