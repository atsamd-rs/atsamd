#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Field `NONBASETHRDENA` reader - Indicates how processor enters Thread mode"]
pub type NonbasethrdenaR = crate::BitReader;
#[doc = "Field `NONBASETHRDENA` writer - Indicates how processor enters Thread mode"]
pub type NonbasethrdenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USERSETMPEND` reader - Enables unprivileged software access to STIR register"]
pub type UsersetmpendR = crate::BitReader;
#[doc = "Field `USERSETMPEND` writer - Enables unprivileged software access to STIR register"]
pub type UsersetmpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enables unaligned access traps\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnalignTrpselect {
    #[doc = "0: Do not trap unaligned halfword and word accesses"]
    Value0 = 0,
    #[doc = "1: Trap unaligned halfword and word accesses"]
    Value1 = 1,
}
impl From<UnalignTrpselect> for bool {
    #[inline(always)]
    fn from(variant: UnalignTrpselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNALIGN_TRP` reader - Enables unaligned access traps"]
pub type UnalignTrpR = crate::BitReader<UnalignTrpselect>;
impl UnalignTrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UnalignTrpselect {
        match self.bits {
            false => UnalignTrpselect::Value0,
            true => UnalignTrpselect::Value1,
        }
    }
    #[doc = "Do not trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == UnalignTrpselect::Value0
    }
    #[doc = "Trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == UnalignTrpselect::Value1
    }
}
#[doc = "Field `UNALIGN_TRP` writer - Enables unaligned access traps"]
pub type UnalignTrpW<'a, REG> = crate::BitWriter<'a, REG, UnalignTrpselect>;
impl<'a, REG> UnalignTrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(UnalignTrpselect::Value0)
    }
    #[doc = "Trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(UnalignTrpselect::Value1)
    }
}
#[doc = "Field `DIV_0_TRP` reader - Enables divide by 0 trap"]
pub type Div0TrpR = crate::BitReader;
#[doc = "Field `DIV_0_TRP` writer - Enables divide by 0 trap"]
pub type Div0TrpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFHFNMIGN` reader - Ignore LDM/STM BusFault for -1/-2 priority handlers"]
pub type BfhfnmignR = crate::BitReader;
#[doc = "Field `BFHFNMIGN` writer - Ignore LDM/STM BusFault for -1/-2 priority handlers"]
pub type BfhfnmignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Indicates stack alignment on exception entry\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stkalignselect {
    #[doc = "0: 4-byte aligned"]
    Value0 = 0,
    #[doc = "1: 8-byte aligned"]
    Value1 = 1,
}
impl From<Stkalignselect> for bool {
    #[inline(always)]
    fn from(variant: Stkalignselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STKALIGN` reader - Indicates stack alignment on exception entry"]
pub type StkalignR = crate::BitReader<Stkalignselect>;
impl StkalignR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stkalignselect {
        match self.bits {
            false => Stkalignselect::Value0,
            true => Stkalignselect::Value1,
        }
    }
    #[doc = "4-byte aligned"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Stkalignselect::Value0
    }
    #[doc = "8-byte aligned"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Stkalignselect::Value1
    }
}
#[doc = "Field `STKALIGN` writer - Indicates stack alignment on exception entry"]
pub type StkalignW<'a, REG> = crate::BitWriter<'a, REG, Stkalignselect>;
impl<'a, REG> StkalignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4-byte aligned"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Stkalignselect::Value0)
    }
    #[doc = "8-byte aligned"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Stkalignselect::Value1)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates how processor enters Thread mode"]
    #[inline(always)]
    pub fn nonbasethrdena(&self) -> NonbasethrdenaR {
        NonbasethrdenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables unprivileged software access to STIR register"]
    #[inline(always)]
    pub fn usersetmpend(&self) -> UsersetmpendR {
        UsersetmpendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UnalignTrpR {
        UnalignTrpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables divide by 0 trap"]
    #[inline(always)]
    pub fn div_0_trp(&self) -> Div0TrpR {
        Div0TrpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Ignore LDM/STM BusFault for -1/-2 priority handlers"]
    #[inline(always)]
    pub fn bfhfnmign(&self) -> BfhfnmignR {
        BfhfnmignR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    pub fn stkalign(&self) -> StkalignR {
        StkalignR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates how processor enters Thread mode"]
    #[inline(always)]
    #[must_use]
    pub fn nonbasethrdena(&mut self) -> NonbasethrdenaW<CcrSpec> {
        NonbasethrdenaW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables unprivileged software access to STIR register"]
    #[inline(always)]
    #[must_use]
    pub fn usersetmpend(&mut self) -> UsersetmpendW<CcrSpec> {
        UsersetmpendW::new(self, 1)
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline(always)]
    #[must_use]
    pub fn unalign_trp(&mut self) -> UnalignTrpW<CcrSpec> {
        UnalignTrpW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables divide by 0 trap"]
    #[inline(always)]
    #[must_use]
    pub fn div_0_trp(&mut self) -> Div0TrpW<CcrSpec> {
        Div0TrpW::new(self, 4)
    }
    #[doc = "Bit 8 - Ignore LDM/STM BusFault for -1/-2 priority handlers"]
    #[inline(always)]
    #[must_use]
    pub fn bfhfnmign(&mut self) -> BfhfnmignW<CcrSpec> {
        BfhfnmignW::new(self, 8)
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    #[must_use]
    pub fn stkalign(&mut self) -> StkalignW<CcrSpec> {
        StkalignW::new(self, 9)
    }
}
#[doc = "Configuration and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0x0200"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0x0200;
}
