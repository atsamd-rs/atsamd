#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `STOP` reader - Stop"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Stop"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDX` reader - Ramp"]
pub type IdxR = crate::BitReader;
#[doc = "Field `IDX` writer - Ramp"]
pub type IdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UFS` reader - Non-recoverable Update Fault State"]
pub type UfsR = crate::BitReader;
#[doc = "Field `UFS` writer - Non-recoverable Update Fault State"]
pub type UfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFS` reader - Non-Recoverable Debug Fault State"]
pub type DfsR = crate::BitReader;
#[doc = "Field `DFS` writer - Non-Recoverable Debug Fault State"]
pub type DfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE` reader - Slave"]
pub type SlaveR = crate::BitReader;
#[doc = "Field `SLAVE` writer - Slave"]
pub type SlaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATTBUFV` reader - Pattern Buffer Valid"]
pub type PattbufvR = crate::BitReader;
#[doc = "Field `PATTBUFV` writer - Pattern Buffer Valid"]
pub type PattbufvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERBUFV` reader - Period Buffer Valid"]
pub type PerbufvR = crate::BitReader;
#[doc = "Field `PERBUFV` writer - Period Buffer Valid"]
pub type PerbufvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTAIN` reader - Recoverable Fault A Input"]
pub type FaultainR = crate::BitReader;
#[doc = "Field `FAULTAIN` writer - Recoverable Fault A Input"]
pub type FaultainW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTBIN` reader - Recoverable Fault B Input"]
pub type FaultbinR = crate::BitReader;
#[doc = "Field `FAULTBIN` writer - Recoverable Fault B Input"]
pub type FaultbinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT0IN` reader - Non-Recoverable Fault0 Input"]
pub type Fault0inR = crate::BitReader;
#[doc = "Field `FAULT0IN` writer - Non-Recoverable Fault0 Input"]
pub type Fault0inW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT1IN` reader - Non-Recoverable Fault1 Input"]
pub type Fault1inR = crate::BitReader;
#[doc = "Field `FAULT1IN` writer - Non-Recoverable Fault1 Input"]
pub type Fault1inW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `CCBUFV0` reader - Compare Channel 0 Buffer Valid"]
pub type Ccbufv0R = crate::BitReader;
#[doc = "Field `CCBUFV0` writer - Compare Channel 0 Buffer Valid"]
pub type Ccbufv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCBUFV1` reader - Compare Channel 1 Buffer Valid"]
pub type Ccbufv1R = crate::BitReader;
#[doc = "Field `CCBUFV1` writer - Compare Channel 1 Buffer Valid"]
pub type Ccbufv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCBUFV2` reader - Compare Channel 2 Buffer Valid"]
pub type Ccbufv2R = crate::BitReader;
#[doc = "Field `CCBUFV2` writer - Compare Channel 2 Buffer Valid"]
pub type Ccbufv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCBUFV3` reader - Compare Channel 3 Buffer Valid"]
pub type Ccbufv3R = crate::BitReader;
#[doc = "Field `CCBUFV3` writer - Compare Channel 3 Buffer Valid"]
pub type Ccbufv3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCBUFV4` reader - Compare Channel 4 Buffer Valid"]
pub type Ccbufv4R = crate::BitReader;
#[doc = "Field `CCBUFV4` writer - Compare Channel 4 Buffer Valid"]
pub type Ccbufv4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCBUFV5` reader - Compare Channel 5 Buffer Valid"]
pub type Ccbufv5R = crate::BitReader;
#[doc = "Field `CCBUFV5` writer - Compare Channel 5 Buffer Valid"]
pub type Ccbufv5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0` reader - Compare Channel 0 Value"]
pub type Cmp0R = crate::BitReader;
#[doc = "Field `CMP0` writer - Compare Channel 0 Value"]
pub type Cmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1` reader - Compare Channel 1 Value"]
pub type Cmp1R = crate::BitReader;
#[doc = "Field `CMP1` writer - Compare Channel 1 Value"]
pub type Cmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2` reader - Compare Channel 2 Value"]
pub type Cmp2R = crate::BitReader;
#[doc = "Field `CMP2` writer - Compare Channel 2 Value"]
pub type Cmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP3` reader - Compare Channel 3 Value"]
pub type Cmp3R = crate::BitReader;
#[doc = "Field `CMP3` writer - Compare Channel 3 Value"]
pub type Cmp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP4` reader - Compare Channel 4 Value"]
pub type Cmp4R = crate::BitReader;
#[doc = "Field `CMP4` writer - Compare Channel 4 Value"]
pub type Cmp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP5` reader - Compare Channel 5 Value"]
pub type Cmp5R = crate::BitReader;
#[doc = "Field `CMP5` writer - Compare Channel 5 Value"]
pub type Cmp5W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - Non-recoverable Update Fault State"]
    #[inline(always)]
    pub fn ufs(&self) -> UfsR {
        UfsR::new(((self.bits >> 2) & 1) != 0)
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
    pub fn pattbufv(&self) -> PattbufvR {
        PattbufvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Period Buffer Valid"]
    #[inline(always)]
    pub fn perbufv(&self) -> PerbufvR {
        PerbufvR::new(((self.bits >> 7) & 1) != 0)
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
    pub fn ccbufv0(&self) -> Ccbufv0R {
        Ccbufv0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv1(&self) -> Ccbufv1R {
        Ccbufv1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv2(&self) -> Ccbufv2R {
        Ccbufv2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv3(&self) -> Ccbufv3R {
        Ccbufv3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Compare Channel 4 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv4(&self) -> Ccbufv4R {
        Ccbufv4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Compare Channel 5 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv5(&self) -> Ccbufv5R {
        Ccbufv5R::new(((self.bits >> 21) & 1) != 0)
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
    #[doc = "Bit 28 - Compare Channel 4 Value"]
    #[inline(always)]
    pub fn cmp4(&self) -> Cmp4R {
        Cmp4R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Compare Channel 5 Value"]
    #[inline(always)]
    pub fn cmp5(&self) -> Cmp5R {
        Cmp5R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<StatusSpec> {
        StopW::new(self, 0)
    }
    #[doc = "Bit 1 - Ramp"]
    #[inline(always)]
    #[must_use]
    pub fn idx(&mut self) -> IdxW<StatusSpec> {
        IdxW::new(self, 1)
    }
    #[doc = "Bit 2 - Non-recoverable Update Fault State"]
    #[inline(always)]
    #[must_use]
    pub fn ufs(&mut self) -> UfsW<StatusSpec> {
        UfsW::new(self, 2)
    }
    #[doc = "Bit 3 - Non-Recoverable Debug Fault State"]
    #[inline(always)]
    #[must_use]
    pub fn dfs(&mut self) -> DfsW<StatusSpec> {
        DfsW::new(self, 3)
    }
    #[doc = "Bit 4 - Slave"]
    #[inline(always)]
    #[must_use]
    pub fn slave(&mut self) -> SlaveW<StatusSpec> {
        SlaveW::new(self, 4)
    }
    #[doc = "Bit 5 - Pattern Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn pattbufv(&mut self) -> PattbufvW<StatusSpec> {
        PattbufvW::new(self, 5)
    }
    #[doc = "Bit 7 - Period Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn perbufv(&mut self) -> PerbufvW<StatusSpec> {
        PerbufvW::new(self, 7)
    }
    #[doc = "Bit 8 - Recoverable Fault A Input"]
    #[inline(always)]
    #[must_use]
    pub fn faultain(&mut self) -> FaultainW<StatusSpec> {
        FaultainW::new(self, 8)
    }
    #[doc = "Bit 9 - Recoverable Fault B Input"]
    #[inline(always)]
    #[must_use]
    pub fn faultbin(&mut self) -> FaultbinW<StatusSpec> {
        FaultbinW::new(self, 9)
    }
    #[doc = "Bit 10 - Non-Recoverable Fault0 Input"]
    #[inline(always)]
    #[must_use]
    pub fn fault0in(&mut self) -> Fault0inW<StatusSpec> {
        Fault0inW::new(self, 10)
    }
    #[doc = "Bit 11 - Non-Recoverable Fault1 Input"]
    #[inline(always)]
    #[must_use]
    pub fn fault1in(&mut self) -> Fault1inW<StatusSpec> {
        Fault1inW::new(self, 11)
    }
    #[doc = "Bit 12 - Recoverable Fault A State"]
    #[inline(always)]
    #[must_use]
    pub fn faulta(&mut self) -> FaultaW<StatusSpec> {
        FaultaW::new(self, 12)
    }
    #[doc = "Bit 13 - Recoverable Fault B State"]
    #[inline(always)]
    #[must_use]
    pub fn faultb(&mut self) -> FaultbW<StatusSpec> {
        FaultbW::new(self, 13)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0 State"]
    #[inline(always)]
    #[must_use]
    pub fn fault0(&mut self) -> Fault0W<StatusSpec> {
        Fault0W::new(self, 14)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1 State"]
    #[inline(always)]
    #[must_use]
    pub fn fault1(&mut self) -> Fault1W<StatusSpec> {
        Fault1W::new(self, 15)
    }
    #[doc = "Bit 16 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv0(&mut self) -> Ccbufv0W<StatusSpec> {
        Ccbufv0W::new(self, 16)
    }
    #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv1(&mut self) -> Ccbufv1W<StatusSpec> {
        Ccbufv1W::new(self, 17)
    }
    #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv2(&mut self) -> Ccbufv2W<StatusSpec> {
        Ccbufv2W::new(self, 18)
    }
    #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv3(&mut self) -> Ccbufv3W<StatusSpec> {
        Ccbufv3W::new(self, 19)
    }
    #[doc = "Bit 20 - Compare Channel 4 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv4(&mut self) -> Ccbufv4W<StatusSpec> {
        Ccbufv4W::new(self, 20)
    }
    #[doc = "Bit 21 - Compare Channel 5 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv5(&mut self) -> Ccbufv5W<StatusSpec> {
        Ccbufv5W::new(self, 21)
    }
    #[doc = "Bit 24 - Compare Channel 0 Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0(&mut self) -> Cmp0W<StatusSpec> {
        Cmp0W::new(self, 24)
    }
    #[doc = "Bit 25 - Compare Channel 1 Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1(&mut self) -> Cmp1W<StatusSpec> {
        Cmp1W::new(self, 25)
    }
    #[doc = "Bit 26 - Compare Channel 2 Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2(&mut self) -> Cmp2W<StatusSpec> {
        Cmp2W::new(self, 26)
    }
    #[doc = "Bit 27 - Compare Channel 3 Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3(&mut self) -> Cmp3W<StatusSpec> {
        Cmp3W::new(self, 27)
    }
    #[doc = "Bit 28 - Compare Channel 4 Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4(&mut self) -> Cmp4W<StatusSpec> {
        Cmp4W::new(self, 28)
    }
    #[doc = "Bit 29 - Compare Channel 5 Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp5(&mut self) -> Cmp5W<StatusSpec> {
        Cmp5W::new(self, 29)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x01"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x01;
}
