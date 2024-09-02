#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `OVF` reader - Overflow Interrupt Enable"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow Interrupt Enable"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRG` reader - Retrigger Interrupt Enable"]
pub type TrgR = crate::BitReader;
#[doc = "Field `TRG` writer - Retrigger Interrupt Enable"]
pub type TrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT` reader - Counter Interrupt Enable"]
pub type CntR = crate::BitReader;
#[doc = "Field `CNT` writer - Counter Interrupt Enable"]
pub type CntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - Error Interrupt Enable"]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - Error Interrupt Enable"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFS` reader - Non-Recoverable Debug Fault Interrupt Enable"]
pub type DfsR = crate::BitReader;
#[doc = "Field `DFS` writer - Non-Recoverable Debug Fault Interrupt Enable"]
pub type DfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTA` reader - Recoverable Fault A Interrupt Enable"]
pub type FaultaR = crate::BitReader;
#[doc = "Field `FAULTA` writer - Recoverable Fault A Interrupt Enable"]
pub type FaultaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTB` reader - Recoverable Fault B Interrupt Enable"]
pub type FaultbR = crate::BitReader;
#[doc = "Field `FAULTB` writer - Recoverable Fault B Interrupt Enable"]
pub type FaultbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT0` reader - Non-Recoverable Fault 0 Interrupt Enable"]
pub type Fault0R = crate::BitReader;
#[doc = "Field `FAULT0` writer - Non-Recoverable Fault 0 Interrupt Enable"]
pub type Fault0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT1` reader - Non-Recoverable Fault 1 Interrupt Enable"]
pub type Fault1R = crate::BitReader;
#[doc = "Field `FAULT1` writer - Non-Recoverable Fault 1 Interrupt Enable"]
pub type Fault1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC0` reader - Match or Capture Channel 0 Interrupt Enable"]
pub type Mc0R = crate::BitReader;
#[doc = "Field `MC0` writer - Match or Capture Channel 0 Interrupt Enable"]
pub type Mc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC1` reader - Match or Capture Channel 1 Interrupt Enable"]
pub type Mc1R = crate::BitReader;
#[doc = "Field `MC1` writer - Match or Capture Channel 1 Interrupt Enable"]
pub type Mc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC2` reader - Match or Capture Channel 2 Interrupt Enable"]
pub type Mc2R = crate::BitReader;
#[doc = "Field `MC2` writer - Match or Capture Channel 2 Interrupt Enable"]
pub type Mc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC3` reader - Match or Capture Channel 3 Interrupt Enable"]
pub type Mc3R = crate::BitReader;
#[doc = "Field `MC3` writer - Match or Capture Channel 3 Interrupt Enable"]
pub type Mc3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Retrigger Interrupt Enable"]
    #[inline(always)]
    pub fn trg(&self) -> TrgR {
        TrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Counter Interrupt Enable"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Non-Recoverable Debug Fault Interrupt Enable"]
    #[inline(always)]
    pub fn dfs(&self) -> DfsR {
        DfsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Recoverable Fault A Interrupt Enable"]
    #[inline(always)]
    pub fn faulta(&self) -> FaultaR {
        FaultaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Recoverable Fault B Interrupt Enable"]
    #[inline(always)]
    pub fn faultb(&self) -> FaultbR {
        FaultbR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0 Interrupt Enable"]
    #[inline(always)]
    pub fn fault0(&self) -> Fault0R {
        Fault0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1 Interrupt Enable"]
    #[inline(always)]
    pub fn fault1(&self) -> Fault1R {
        Fault1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Match or Capture Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn mc0(&self) -> Mc0R {
        Mc0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Match or Capture Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn mc1(&self) -> Mc1R {
        Mc1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Match or Capture Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn mc2(&self) -> Mc2R {
        Mc2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Match or Capture Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn mc3(&self) -> Mc3R {
        Mc3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OvfW<IntenclrSpec> {
        OvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Retrigger Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trg(&mut self) -> TrgW<IntenclrSpec> {
        TrgW::new(self, 1)
    }
    #[doc = "Bit 2 - Counter Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<IntenclrSpec> {
        CntW::new(self, 2)
    }
    #[doc = "Bit 3 - Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ErrW<IntenclrSpec> {
        ErrW::new(self, 3)
    }
    #[doc = "Bit 11 - Non-Recoverable Debug Fault Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfs(&mut self) -> DfsW<IntenclrSpec> {
        DfsW::new(self, 11)
    }
    #[doc = "Bit 12 - Recoverable Fault A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn faulta(&mut self) -> FaultaW<IntenclrSpec> {
        FaultaW::new(self, 12)
    }
    #[doc = "Bit 13 - Recoverable Fault B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn faultb(&mut self) -> FaultbW<IntenclrSpec> {
        FaultbW::new(self, 13)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fault0(&mut self) -> Fault0W<IntenclrSpec> {
        Fault0W::new(self, 14)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fault1(&mut self) -> Fault1W<IntenclrSpec> {
        Fault1W::new(self, 15)
    }
    #[doc = "Bit 16 - Match or Capture Channel 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mc0(&mut self) -> Mc0W<IntenclrSpec> {
        Mc0W::new(self, 16)
    }
    #[doc = "Bit 17 - Match or Capture Channel 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mc1(&mut self) -> Mc1W<IntenclrSpec> {
        Mc1W::new(self, 17)
    }
    #[doc = "Bit 18 - Match or Capture Channel 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mc2(&mut self) -> Mc2W<IntenclrSpec> {
        Mc2W::new(self, 18)
    }
    #[doc = "Bit 19 - Match or Capture Channel 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mc3(&mut self) -> Mc3W<IntenclrSpec> {
        Mc3W::new(self, 19)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u32 = 0;
}
