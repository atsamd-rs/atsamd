#[doc = "Register `PMUX0_%s` reader"]
pub type R = crate::R<Pmux0_Spec>;
#[doc = "Register `PMUX0_%s` writer"]
pub type W = crate::W<Pmux0_Spec>;
#[doc = "Peripheral Multiplexing Even\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pmuxeselect {
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
impl From<Pmuxeselect> for u8 {
    #[inline(always)]
    fn from(variant: Pmuxeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pmuxeselect {
    type Ux = u8;
}
impl crate::IsEnum for Pmuxeselect {}
#[doc = "Field `PMUXE` reader - Peripheral Multiplexing Even"]
pub type PmuxeR = crate::FieldReader<Pmuxeselect>;
impl PmuxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pmuxeselect> {
        match self.bits {
            0 => Some(Pmuxeselect::A),
            1 => Some(Pmuxeselect::B),
            2 => Some(Pmuxeselect::C),
            3 => Some(Pmuxeselect::D),
            4 => Some(Pmuxeselect::E),
            5 => Some(Pmuxeselect::F),
            6 => Some(Pmuxeselect::G),
            7 => Some(Pmuxeselect::H),
            _ => None,
        }
    }
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == Pmuxeselect::A
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == Pmuxeselect::B
    }
    #[doc = "Peripheral function C selected"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == Pmuxeselect::C
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == Pmuxeselect::D
    }
    #[doc = "Peripheral function E selected"]
    #[inline(always)]
    pub fn is_e(&self) -> bool {
        *self == Pmuxeselect::E
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        *self == Pmuxeselect::F
    }
    #[doc = "Peripheral function G selected"]
    #[inline(always)]
    pub fn is_g(&self) -> bool {
        *self == Pmuxeselect::G
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == Pmuxeselect::H
    }
}
#[doc = "Field `PMUXE` writer - Peripheral Multiplexing Even"]
pub type PmuxeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pmuxeselect>;
impl<'a, REG> PmuxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::A)
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::B)
    }
    #[doc = "Peripheral function C selected"]
    #[inline(always)]
    pub fn c(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::C)
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::D)
    }
    #[doc = "Peripheral function E selected"]
    #[inline(always)]
    pub fn e(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::E)
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn f(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::F)
    }
    #[doc = "Peripheral function G selected"]
    #[inline(always)]
    pub fn g(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::G)
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn h(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::H)
    }
}
#[doc = "Peripheral Multiplexing Odd\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pmuxoselect {
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
impl From<Pmuxoselect> for u8 {
    #[inline(always)]
    fn from(variant: Pmuxoselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pmuxoselect {
    type Ux = u8;
}
impl crate::IsEnum for Pmuxoselect {}
#[doc = "Field `PMUXO` reader - Peripheral Multiplexing Odd"]
pub type PmuxoR = crate::FieldReader<Pmuxoselect>;
impl PmuxoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pmuxoselect> {
        match self.bits {
            0 => Some(Pmuxoselect::A),
            1 => Some(Pmuxoselect::B),
            2 => Some(Pmuxoselect::C),
            3 => Some(Pmuxoselect::D),
            4 => Some(Pmuxoselect::E),
            5 => Some(Pmuxoselect::F),
            6 => Some(Pmuxoselect::G),
            7 => Some(Pmuxoselect::H),
            _ => None,
        }
    }
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == Pmuxoselect::A
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == Pmuxoselect::B
    }
    #[doc = "Peripheral function C selected"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == Pmuxoselect::C
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == Pmuxoselect::D
    }
    #[doc = "Peripheral function E selected"]
    #[inline(always)]
    pub fn is_e(&self) -> bool {
        *self == Pmuxoselect::E
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        *self == Pmuxoselect::F
    }
    #[doc = "Peripheral function G selected"]
    #[inline(always)]
    pub fn is_g(&self) -> bool {
        *self == Pmuxoselect::G
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == Pmuxoselect::H
    }
}
#[doc = "Field `PMUXO` writer - Peripheral Multiplexing Odd"]
pub type PmuxoW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pmuxoselect>;
impl<'a, REG> PmuxoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral function A selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::A)
    }
    #[doc = "Peripheral function B selected"]
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::B)
    }
    #[doc = "Peripheral function C selected"]
    #[inline(always)]
    pub fn c(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::C)
    }
    #[doc = "Peripheral function D selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::D)
    }
    #[doc = "Peripheral function E selected"]
    #[inline(always)]
    pub fn e(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::E)
    }
    #[doc = "Peripheral function F selected"]
    #[inline(always)]
    pub fn f(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::F)
    }
    #[doc = "Peripheral function G selected"]
    #[inline(always)]
    pub fn g(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::G)
    }
    #[doc = "Peripheral function H selected"]
    #[inline(always)]
    pub fn h(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::H)
    }
}
impl R {
    #[doc = "Bits 0:3 - Peripheral Multiplexing Even"]
    #[inline(always)]
    pub fn pmuxe(&self) -> PmuxeR {
        PmuxeR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing Odd"]
    #[inline(always)]
    pub fn pmuxo(&self) -> PmuxoR {
        PmuxoR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Peripheral Multiplexing Even"]
    #[inline(always)]
    pub fn pmuxe(&mut self) -> PmuxeW<Pmux0_Spec> {
        PmuxeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing Odd"]
    #[inline(always)]
    pub fn pmuxo(&mut self) -> PmuxoW<Pmux0_Spec> {
        PmuxoW::new(self, 4)
    }
}
#[doc = "Peripheral Multiplexing n - Group 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pmux0_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmux0_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pmux0_Spec;
impl crate::RegisterSpec for Pmux0_Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pmux0_::R`](R) reader structure"]
impl crate::Readable for Pmux0_Spec {}
#[doc = "`write(|w| ..)` method takes [`pmux0_::W`](W) writer structure"]
impl crate::Writable for Pmux0_Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMUX0_%s to value 0"]
impl crate::Resettable for Pmux0_Spec {}
