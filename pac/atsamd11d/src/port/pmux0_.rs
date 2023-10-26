#[doc = "Register `PMUX0_%s` reader"]
pub type R = crate::R<PMUX0__SPEC>;
#[doc = "Register `PMUX0_%s` writer"]
pub type W = crate::W<PMUX0__SPEC>;
#[doc = "Field `PMUXE` reader - Peripheral Multiplexing Even"]
pub type PMUXE_R = crate::FieldReader<PMUXESELECT_A>;
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
impl crate::FieldSpec for PMUXESELECT_A {
    type Ux = u8;
}
impl PMUXE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PMUXESELECT_A> {
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
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == PMUXESELECT_A::A
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == PMUXESELECT_A::B
    }
    #[doc = "Peripheral function C selected"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == PMUXESELECT_A::C
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == PMUXESELECT_A::D
    }
    #[doc = "Peripheral function E selected"]
    #[inline(always)]
    pub fn is_e(&self) -> bool {
        *self == PMUXESELECT_A::E
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        *self == PMUXESELECT_A::F
    }
    #[doc = "Peripheral function G selected"]
    #[inline(always)]
    pub fn is_g(&self) -> bool {
        *self == PMUXESELECT_A::G
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == PMUXESELECT_A::H
    }
}
#[doc = "Field `PMUXE` writer - Peripheral Multiplexing Even"]
pub type PMUXE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, PMUXESELECT_A>;
impl<'a, REG, const O: u8> PMUXE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(PMUXESELECT_A::A)
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(PMUXESELECT_A::B)
    }
    #[doc = "Peripheral function C selected"]
    #[inline(always)]
    pub fn c(self) -> &'a mut crate::W<REG> {
        self.variant(PMUXESELECT_A::C)
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(PMUXESELECT_A::D)
    }
    #[doc = "Peripheral function E selected"]
    #[inline(always)]
    pub fn e(self) -> &'a mut crate::W<REG> {
        self.variant(PMUXESELECT_A::E)
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn f(self) -> &'a mut crate::W<REG> {
        self.variant(PMUXESELECT_A::F)
    }
    #[doc = "Peripheral function G selected"]
    #[inline(always)]
    pub fn g(self) -> &'a mut crate::W<REG> {
        self.variant(PMUXESELECT_A::G)
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn h(self) -> &'a mut crate::W<REG> {
        self.variant(PMUXESELECT_A::H)
    }
}
#[doc = "Field `PMUXO` reader - Peripheral Multiplexing Odd"]
pub type PMUXO_R = crate::FieldReader<PMUXOSELECT_A>;
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
impl crate::FieldSpec for PMUXOSELECT_A {
    type Ux = u8;
}
impl PMUXO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PMUXOSELECT_A> {
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
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == PMUXOSELECT_A::A
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == PMUXOSELECT_A::B
    }
    #[doc = "Peripheral function C selected"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == PMUXOSELECT_A::C
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == PMUXOSELECT_A::D
    }
    #[doc = "Peripheral function E selected"]
    #[inline(always)]
    pub fn is_e(&self) -> bool {
        *self == PMUXOSELECT_A::E
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        *self == PMUXOSELECT_A::F
    }
    #[doc = "Peripheral function G selected"]
    #[inline(always)]
    pub fn is_g(&self) -> bool {
        *self == PMUXOSELECT_A::G
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == PMUXOSELECT_A::H
    }
}
#[doc = "Field `PMUXO` writer - Peripheral Multiplexing Odd"]
pub type PMUXO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, PMUXOSELECT_A>;
impl<'a, REG, const O: u8> PMUXO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(PMUXOSELECT_A::A)
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(PMUXOSELECT_A::B)
    }
    #[doc = "Peripheral function C selected"]
    #[inline(always)]
    pub fn c(self) -> &'a mut crate::W<REG> {
        self.variant(PMUXOSELECT_A::C)
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(PMUXOSELECT_A::D)
    }
    #[doc = "Peripheral function E selected"]
    #[inline(always)]
    pub fn e(self) -> &'a mut crate::W<REG> {
        self.variant(PMUXOSELECT_A::E)
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn f(self) -> &'a mut crate::W<REG> {
        self.variant(PMUXOSELECT_A::F)
    }
    #[doc = "Peripheral function G selected"]
    #[inline(always)]
    pub fn g(self) -> &'a mut crate::W<REG> {
        self.variant(PMUXOSELECT_A::G)
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn h(self) -> &'a mut crate::W<REG> {
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
    pub fn pmuxe(&mut self) -> PMUXE_W<PMUX0__SPEC, 0> {
        PMUXE_W::new(self)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing Odd"]
    #[inline(always)]
    #[must_use]
    pub fn pmuxo(&mut self) -> PMUXO_W<PMUX0__SPEC, 4> {
        PMUXO_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral Multiplexing n - Group 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmux0_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmux0_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMUX0__SPEC;
impl crate::RegisterSpec for PMUX0__SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pmux0_::R`](R) reader structure"]
impl crate::Readable for PMUX0__SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmux0_::W`](W) writer structure"]
impl crate::Writable for PMUX0__SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMUX0_%s to value 0"]
impl crate::Resettable for PMUX0__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
