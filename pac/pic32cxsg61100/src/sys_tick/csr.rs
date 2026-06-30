#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "SysTick Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enableselect {
    #[doc = "0: Counter disabled"]
    Value0 = 0,
    #[doc = "1: Counter enabled"]
    Value1 = 1,
}
impl From<Enableselect> for bool {
    #[inline(always)]
    fn from(variant: Enableselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - SysTick Counter Enable"]
pub type EnableR = crate::BitReader<Enableselect>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enableselect {
        match self.bits {
            false => Enableselect::Value0,
            true => Enableselect::Value1,
        }
    }
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Enableselect::Value0
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Enableselect::Value1
    }
}
#[doc = "Field `ENABLE` writer - SysTick Counter Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enableselect>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Value0)
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Value1)
    }
}
#[doc = "SysTick Exception Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tickintselect {
    #[doc = "0: Counting down to 0 does not assert the SysTick exception request"]
    Value0 = 0,
    #[doc = "1: Counting down to 0 asserts the SysTick exception request"]
    Value1 = 1,
}
impl From<Tickintselect> for bool {
    #[inline(always)]
    fn from(variant: Tickintselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICKINT` reader - SysTick Exception Request Enable"]
pub type TickintR = crate::BitReader<Tickintselect>;
impl TickintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tickintselect {
        match self.bits {
            false => Tickintselect::Value0,
            true => Tickintselect::Value1,
        }
    }
    #[doc = "Counting down to 0 does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Tickintselect::Value0
    }
    #[doc = "Counting down to 0 asserts the SysTick exception request"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Tickintselect::Value1
    }
}
#[doc = "Field `TICKINT` writer - SysTick Exception Request Enable"]
pub type TickintW<'a, REG> = crate::BitWriter<'a, REG, Tickintselect>;
impl<'a, REG> TickintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counting down to 0 does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tickintselect::Value0)
    }
    #[doc = "Counting down to 0 asserts the SysTick exception request"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tickintselect::Value1)
    }
}
#[doc = "Clock Source 0=external, 1=processor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clksourceselect {
    #[doc = "0: External clock"]
    Value0 = 0,
    #[doc = "1: Processor clock"]
    Value1 = 1,
}
impl From<Clksourceselect> for bool {
    #[inline(always)]
    fn from(variant: Clksourceselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSOURCE` reader - Clock Source 0=external, 1=processor"]
pub type ClksourceR = crate::BitReader<Clksourceselect>;
impl ClksourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksourceselect {
        match self.bits {
            false => Clksourceselect::Value0,
            true => Clksourceselect::Value1,
        }
    }
    #[doc = "External clock"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Clksourceselect::Value0
    }
    #[doc = "Processor clock"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Clksourceselect::Value1
    }
}
#[doc = "Field `CLKSOURCE` writer - Clock Source 0=external, 1=processor"]
pub type ClksourceW<'a, REG> = crate::BitWriter<'a, REG, Clksourceselect>;
impl<'a, REG> ClksourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External clock"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clksourceselect::Value0)
    }
    #[doc = "Processor clock"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clksourceselect::Value1)
    }
}
#[doc = "Field `COUNTFLAG` reader - Timer counted to 0 since last read of register"]
pub type CountflagR = crate::BitReader;
#[doc = "Field `COUNTFLAG` writer - Timer counted to 0 since last read of register"]
pub type CountflagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SysTick Counter Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SysTick Exception Request Enable"]
    #[inline(always)]
    pub fn tickint(&self) -> TickintR {
        TickintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Source 0=external, 1=processor"]
    #[inline(always)]
    pub fn clksource(&self) -> ClksourceR {
        ClksourceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer counted to 0 since last read of register"]
    #[inline(always)]
    pub fn countflag(&self) -> CountflagR {
        CountflagR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SysTick Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CsrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - SysTick Exception Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tickint(&mut self) -> TickintW<CsrSpec> {
        TickintW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock Source 0=external, 1=processor"]
    #[inline(always)]
    #[must_use]
    pub fn clksource(&mut self) -> ClksourceW<CsrSpec> {
        ClksourceW::new(self, 2)
    }
    #[doc = "Bit 16 - Timer counted to 0 since last read of register"]
    #[inline(always)]
    #[must_use]
    pub fn countflag(&mut self) -> CountflagW<CsrSpec> {
        CountflagW::new(self, 16)
    }
}
#[doc = "SysTick Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0x04"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0x04;
}
