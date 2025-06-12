#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `STOP` reader - Stop"]
pub type StopR = crate::BitReader;
#[doc = "Field `IDX` reader - Ramp"]
pub type IdxR = crate::BitReader;
#[doc = "Field `DFS` reader - Non-Recoverable Debug Fault State"]
pub type DfsR = crate::BitReader;
#[doc = "Field `DFS` writer - Non-Recoverable Debug Fault State"]
pub type DfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE` reader - Slave"]
pub type SlaveR = crate::BitReader;
#[doc = "Field `PATTBV` reader - Pattern Buffer Valid"]
pub type PattbvR = crate::BitReader;
#[doc = "Field `PATTBV` writer - Pattern Buffer Valid"]
pub type PattbvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVEBV` reader - Wave Buffer Valid"]
pub type WavebvR = crate::BitReader;
#[doc = "Field `WAVEBV` writer - Wave Buffer Valid"]
pub type WavebvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERBV` reader - Period Buffer Valid"]
pub type PerbvR = crate::BitReader;
#[doc = "Field `PERBV` writer - Period Buffer Valid"]
pub type PerbvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTAIN` reader - Recoverable Fault A Input"]
pub type FaultainR = crate::BitReader;
#[doc = "Field `FAULTBIN` reader - Recoverable Fault B Input"]
pub type FaultbinR = crate::BitReader;
#[doc = "Field `FAULT0IN` reader - Non-Recoverable Fault0 Input"]
pub type Fault0inR = crate::BitReader;
#[doc = "Field `FAULT1IN` reader - Non-Recoverable Fault1 Input"]
pub type Fault1inR = crate::BitReader;
#[doc = "Field `FAULTA` reader - Recoverable Fault A State"]
pub type FaultaR = crate::BitReader;
#[doc = "Field `FAULTA` writer - Recoverable Fault A State"]
pub type FaultaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTB` reader - Recoverable Fault B State"]
pub type FaultbR = crate::BitReader;
#[doc = "Field `FAULTB` writer - Recoverable Fault B State"]
pub type FaultbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT0` reader - Non-Recoverable Fault 0 State"]
pub type Fault0R = crate::BitReader;
#[doc = "Field `FAULT0` writer - Non-Recoverable Fault 0 State"]
pub type Fault0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT1` reader - Non-Recoverable Fault 1 State"]
pub type Fault1R = crate::BitReader;
#[doc = "Field `FAULT1` writer - Non-Recoverable Fault 1 State"]
pub type Fault1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCBV0` reader - Compare Channel 0 Buffer Valid"]
pub type Ccbv0R = crate::BitReader;
#[doc = "Field `CCBV0` writer - Compare Channel 0 Buffer Valid"]
pub type Ccbv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCBV1` reader - Compare Channel 1 Buffer Valid"]
pub type Ccbv1R = crate::BitReader;
#[doc = "Field `CCBV1` writer - Compare Channel 1 Buffer Valid"]
pub type Ccbv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCBV2` reader - Compare Channel 2 Buffer Valid"]
pub type Ccbv2R = crate::BitReader;
#[doc = "Field `CCBV2` writer - Compare Channel 2 Buffer Valid"]
pub type Ccbv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCBV3` reader - Compare Channel 3 Buffer Valid"]
pub type Ccbv3R = crate::BitReader;
#[doc = "Field `CCBV3` writer - Compare Channel 3 Buffer Valid"]
pub type Ccbv3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0` reader - Compare Channel 0 Value"]
pub type Cmp0R = crate::BitReader;
#[doc = "Field `CMP1` reader - Compare Channel 1 Value"]
pub type Cmp1R = crate::BitReader;
#[doc = "Field `CMP2` reader - Compare Channel 2 Value"]
pub type Cmp2R = crate::BitReader;
#[doc = "Field `CMP3` reader - Compare Channel 3 Value"]
pub type Cmp3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Stop"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ramp"]
    #[inline(always)]
    pub fn idx(&self) -> IdxR {
        IdxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Non-Recoverable Debug Fault State"]
    #[inline(always)]
    pub fn dfs(&self) -> DfsR {
        DfsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave"]
    #[inline(always)]
    pub fn slave(&self) -> SlaveR {
        SlaveR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pattern Buffer Valid"]
    #[inline(always)]
    pub fn pattbv(&self) -> PattbvR {
        PattbvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wave Buffer Valid"]
    #[inline(always)]
    pub fn wavebv(&self) -> WavebvR {
        WavebvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Period Buffer Valid"]
    #[inline(always)]
    pub fn perbv(&self) -> PerbvR {
        PerbvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Recoverable Fault A Input"]
    #[inline(always)]
    pub fn faultain(&self) -> FaultainR {
        FaultainR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Recoverable Fault B Input"]
    #[inline(always)]
    pub fn faultbin(&self) -> FaultbinR {
        FaultbinR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Non-Recoverable Fault0 Input"]
    #[inline(always)]
    pub fn fault0in(&self) -> Fault0inR {
        Fault0inR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Non-Recoverable Fault1 Input"]
    #[inline(always)]
    pub fn fault1in(&self) -> Fault1inR {
        Fault1inR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Recoverable Fault A State"]
    #[inline(always)]
    pub fn faulta(&self) -> FaultaR {
        FaultaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Recoverable Fault B State"]
    #[inline(always)]
    pub fn faultb(&self) -> FaultbR {
        FaultbR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0 State"]
    #[inline(always)]
    pub fn fault0(&self) -> Fault0R {
        Fault0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1 State"]
    #[inline(always)]
    pub fn fault1(&self) -> Fault1R {
        Fault1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv0(&self) -> Ccbv0R {
        Ccbv0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv1(&self) -> Ccbv1R {
        Ccbv1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv2(&self) -> Ccbv2R {
        Ccbv2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv3(&self) -> Ccbv3R {
        Ccbv3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Compare Channel 0 Value"]
    #[inline(always)]
    pub fn cmp0(&self) -> Cmp0R {
        Cmp0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Compare Channel 1 Value"]
    #[inline(always)]
    pub fn cmp1(&self) -> Cmp1R {
        Cmp1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Compare Channel 2 Value"]
    #[inline(always)]
    pub fn cmp2(&self) -> Cmp2R {
        Cmp2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Compare Channel 3 Value"]
    #[inline(always)]
    pub fn cmp3(&self) -> Cmp3R {
        Cmp3R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Non-Recoverable Debug Fault State"]
    #[inline(always)]
    pub fn dfs(&mut self) -> DfsW<StatusSpec> {
        DfsW::new(self, 3)
    }
    #[doc = "Bit 5 - Pattern Buffer Valid"]
    #[inline(always)]
    pub fn pattbv(&mut self) -> PattbvW<StatusSpec> {
        PattbvW::new(self, 5)
    }
    #[doc = "Bit 6 - Wave Buffer Valid"]
    #[inline(always)]
    pub fn wavebv(&mut self) -> WavebvW<StatusSpec> {
        WavebvW::new(self, 6)
    }
    #[doc = "Bit 7 - Period Buffer Valid"]
    #[inline(always)]
    pub fn perbv(&mut self) -> PerbvW<StatusSpec> {
        PerbvW::new(self, 7)
    }
    #[doc = "Bit 12 - Recoverable Fault A State"]
    #[inline(always)]
    pub fn faulta(&mut self) -> FaultaW<StatusSpec> {
        FaultaW::new(self, 12)
    }
    #[doc = "Bit 13 - Recoverable Fault B State"]
    #[inline(always)]
    pub fn faultb(&mut self) -> FaultbW<StatusSpec> {
        FaultbW::new(self, 13)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0 State"]
    #[inline(always)]
    pub fn fault0(&mut self) -> Fault0W<StatusSpec> {
        Fault0W::new(self, 14)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1 State"]
    #[inline(always)]
    pub fn fault1(&mut self) -> Fault1W<StatusSpec> {
        Fault1W::new(self, 15)
    }
    #[doc = "Bit 16 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv0(&mut self) -> Ccbv0W<StatusSpec> {
        Ccbv0W::new(self, 16)
    }
    #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv1(&mut self) -> Ccbv1W<StatusSpec> {
        Ccbv1W::new(self, 17)
    }
    #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv2(&mut self) -> Ccbv2W<StatusSpec> {
        Ccbv2W::new(self, 18)
    }
    #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv3(&mut self) -> Ccbv3W<StatusSpec> {
        Ccbv3W::new(self, 19)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0x01"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x01;
}
