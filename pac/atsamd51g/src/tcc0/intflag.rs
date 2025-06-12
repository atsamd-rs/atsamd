#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `OVF` reader - Overflow"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRG` reader - Retrigger"]
pub type TrgR = crate::BitReader;
#[doc = "Field `TRG` writer - Retrigger"]
pub type TrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT` reader - Counter"]
pub type CntR = crate::BitReader;
#[doc = "Field `CNT` writer - Counter"]
pub type CntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - Error"]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - Error"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UFS` reader - Non-Recoverable Update Fault"]
pub type UfsR = crate::BitReader;
#[doc = "Field `UFS` writer - Non-Recoverable Update Fault"]
pub type UfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFS` reader - Non-Recoverable Debug Fault"]
pub type DfsR = crate::BitReader;
#[doc = "Field `DFS` writer - Non-Recoverable Debug Fault"]
pub type DfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTA` reader - Recoverable Fault A"]
pub type FaultaR = crate::BitReader;
#[doc = "Field `FAULTA` writer - Recoverable Fault A"]
pub type FaultaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTB` reader - Recoverable Fault B"]
pub type FaultbR = crate::BitReader;
#[doc = "Field `FAULTB` writer - Recoverable Fault B"]
pub type FaultbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT0` reader - Non-Recoverable Fault 0"]
pub type Fault0R = crate::BitReader;
#[doc = "Field `FAULT0` writer - Non-Recoverable Fault 0"]
pub type Fault0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT1` reader - Non-Recoverable Fault 1"]
pub type Fault1R = crate::BitReader;
#[doc = "Field `FAULT1` writer - Non-Recoverable Fault 1"]
pub type Fault1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC0` reader - Match or Capture 0"]
pub type Mc0R = crate::BitReader;
#[doc = "Field `MC0` writer - Match or Capture 0"]
pub type Mc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC1` reader - Match or Capture 1"]
pub type Mc1R = crate::BitReader;
#[doc = "Field `MC1` writer - Match or Capture 1"]
pub type Mc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC2` reader - Match or Capture 2"]
pub type Mc2R = crate::BitReader;
#[doc = "Field `MC2` writer - Match or Capture 2"]
pub type Mc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC3` reader - Match or Capture 3"]
pub type Mc3R = crate::BitReader;
#[doc = "Field `MC3` writer - Match or Capture 3"]
pub type Mc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC4` reader - Match or Capture 4"]
pub type Mc4R = crate::BitReader;
#[doc = "Field `MC4` writer - Match or Capture 4"]
pub type Mc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC5` reader - Match or Capture 5"]
pub type Mc5R = crate::BitReader;
#[doc = "Field `MC5` writer - Match or Capture 5"]
pub type Mc5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Retrigger"]
    #[inline(always)]
    pub fn trg(&self) -> TrgR {
        TrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 10 - Non-Recoverable Update Fault"]
    #[inline(always)]
    pub fn ufs(&self) -> UfsR {
        UfsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Non-Recoverable Debug Fault"]
    #[inline(always)]
    pub fn dfs(&self) -> DfsR {
        DfsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Recoverable Fault A"]
    #[inline(always)]
    pub fn faulta(&self) -> FaultaR {
        FaultaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Recoverable Fault B"]
    #[inline(always)]
    pub fn faultb(&self) -> FaultbR {
        FaultbR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0"]
    #[inline(always)]
    pub fn fault0(&self) -> Fault0R {
        Fault0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1"]
    #[inline(always)]
    pub fn fault1(&self) -> Fault1R {
        Fault1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Match or Capture 0"]
    #[inline(always)]
    pub fn mc0(&self) -> Mc0R {
        Mc0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Match or Capture 1"]
    #[inline(always)]
    pub fn mc1(&self) -> Mc1R {
        Mc1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Match or Capture 2"]
    #[inline(always)]
    pub fn mc2(&self) -> Mc2R {
        Mc2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Match or Capture 3"]
    #[inline(always)]
    pub fn mc3(&self) -> Mc3R {
        Mc3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Match or Capture 4"]
    #[inline(always)]
    pub fn mc4(&self) -> Mc4R {
        Mc4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Match or Capture 5"]
    #[inline(always)]
    pub fn mc5(&self) -> Mc5R {
        Mc5R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OvfW<IntflagSpec> {
        OvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Retrigger"]
    #[inline(always)]
    pub fn trg(&mut self) -> TrgW<IntflagSpec> {
        TrgW::new(self, 1)
    }
    #[doc = "Bit 2 - Counter"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<IntflagSpec> {
        CntW::new(self, 2)
    }
    #[doc = "Bit 3 - Error"]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<IntflagSpec> {
        ErrW::new(self, 3)
    }
    #[doc = "Bit 10 - Non-Recoverable Update Fault"]
    #[inline(always)]
    pub fn ufs(&mut self) -> UfsW<IntflagSpec> {
        UfsW::new(self, 10)
    }
    #[doc = "Bit 11 - Non-Recoverable Debug Fault"]
    #[inline(always)]
    pub fn dfs(&mut self) -> DfsW<IntflagSpec> {
        DfsW::new(self, 11)
    }
    #[doc = "Bit 12 - Recoverable Fault A"]
    #[inline(always)]
    pub fn faulta(&mut self) -> FaultaW<IntflagSpec> {
        FaultaW::new(self, 12)
    }
    #[doc = "Bit 13 - Recoverable Fault B"]
    #[inline(always)]
    pub fn faultb(&mut self) -> FaultbW<IntflagSpec> {
        FaultbW::new(self, 13)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0"]
    #[inline(always)]
    pub fn fault0(&mut self) -> Fault0W<IntflagSpec> {
        Fault0W::new(self, 14)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1"]
    #[inline(always)]
    pub fn fault1(&mut self) -> Fault1W<IntflagSpec> {
        Fault1W::new(self, 15)
    }
    #[doc = "Bit 16 - Match or Capture 0"]
    #[inline(always)]
    pub fn mc0(&mut self) -> Mc0W<IntflagSpec> {
        Mc0W::new(self, 16)
    }
    #[doc = "Bit 17 - Match or Capture 1"]
    #[inline(always)]
    pub fn mc1(&mut self) -> Mc1W<IntflagSpec> {
        Mc1W::new(self, 17)
    }
    #[doc = "Bit 18 - Match or Capture 2"]
    #[inline(always)]
    pub fn mc2(&mut self) -> Mc2W<IntflagSpec> {
        Mc2W::new(self, 18)
    }
    #[doc = "Bit 19 - Match or Capture 3"]
    #[inline(always)]
    pub fn mc3(&mut self) -> Mc3W<IntflagSpec> {
        Mc3W::new(self, 19)
    }
    #[doc = "Bit 20 - Match or Capture 4"]
    #[inline(always)]
    pub fn mc4(&mut self) -> Mc4W<IntflagSpec> {
        Mc4W::new(self, 20)
    }
    #[doc = "Bit 21 - Match or Capture 5"]
    #[inline(always)]
    pub fn mc5(&mut self) -> Mc5W<IntflagSpec> {
        Mc5W::new(self, 21)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
