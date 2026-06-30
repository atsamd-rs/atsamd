#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `QERR` reader - Quadrature Error Flag"]
pub type QerrR = crate::BitReader;
#[doc = "Field `QERR` writer - Quadrature Error Flag"]
pub type QerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDXERR` reader - Index Error Flag"]
pub type IdxerrR = crate::BitReader;
#[doc = "Field `IDXERR` writer - Index Error Flag"]
pub type IdxerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPERR` reader - Missing Pulse Error flag"]
pub type MperrR = crate::BitReader;
#[doc = "Field `MPERR` writer - Missing Pulse Error flag"]
pub type MperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINERR` reader - Window Error Flag"]
pub type WinerrR = crate::BitReader;
#[doc = "Field `WINERR` writer - Window Error Flag"]
pub type WinerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HERR` reader - Hall Error Flag"]
pub type HerrR = crate::BitReader;
#[doc = "Field `HERR` writer - Hall Error Flag"]
pub type HerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Stop"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Stop"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - Direction Status Flag"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Direction Status Flag"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCBUFV` reader - Prescaler Buffer Valid"]
pub type PrescbufvR = crate::BitReader;
#[doc = "Field `PRESCBUFV` writer - Prescaler Buffer Valid"]
pub type PrescbufvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILTERBUFV` reader - Filter Buffer Valid"]
pub type FilterbufvR = crate::BitReader;
#[doc = "Field `FILTERBUFV` writer - Filter Buffer Valid"]
pub type FilterbufvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCBUFV0` reader - Compare Channel 0 Buffer Valid"]
pub type Ccbufv0R = crate::BitReader;
#[doc = "Field `CCBUFV0` writer - Compare Channel 0 Buffer Valid"]
pub type Ccbufv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCBUFV1` reader - Compare Channel 1 Buffer Valid"]
pub type Ccbufv1R = crate::BitReader;
#[doc = "Field `CCBUFV1` writer - Compare Channel 1 Buffer Valid"]
pub type Ccbufv1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Quadrature Error Flag"]
    #[inline(always)]
    pub fn qerr(&self) -> QerrR {
        QerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Index Error Flag"]
    #[inline(always)]
    pub fn idxerr(&self) -> IdxerrR {
        IdxerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Missing Pulse Error flag"]
    #[inline(always)]
    pub fn mperr(&self) -> MperrR {
        MperrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Window Error Flag"]
    #[inline(always)]
    pub fn winerr(&self) -> WinerrR {
        WinerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hall Error Flag"]
    #[inline(always)]
    pub fn herr(&self) -> HerrR {
        HerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Direction Status Flag"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Prescaler Buffer Valid"]
    #[inline(always)]
    pub fn prescbufv(&self) -> PrescbufvR {
        PrescbufvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter Buffer Valid"]
    #[inline(always)]
    pub fn filterbufv(&self) -> FilterbufvR {
        FilterbufvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv0(&self) -> Ccbufv0R {
        Ccbufv0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv1(&self) -> Ccbufv1R {
        Ccbufv1R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Quadrature Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn qerr(&mut self) -> QerrW<StatusSpec> {
        QerrW::new(self, 0)
    }
    #[doc = "Bit 1 - Index Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn idxerr(&mut self) -> IdxerrW<StatusSpec> {
        IdxerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Missing Pulse Error flag"]
    #[inline(always)]
    #[must_use]
    pub fn mperr(&mut self) -> MperrW<StatusSpec> {
        MperrW::new(self, 2)
    }
    #[doc = "Bit 4 - Window Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn winerr(&mut self) -> WinerrW<StatusSpec> {
        WinerrW::new(self, 4)
    }
    #[doc = "Bit 5 - Hall Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn herr(&mut self) -> HerrW<StatusSpec> {
        HerrW::new(self, 5)
    }
    #[doc = "Bit 6 - Stop"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<StatusSpec> {
        StopW::new(self, 6)
    }
    #[doc = "Bit 7 - Direction Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<StatusSpec> {
        DirW::new(self, 7)
    }
    #[doc = "Bit 8 - Prescaler Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn prescbufv(&mut self) -> PrescbufvW<StatusSpec> {
        PrescbufvW::new(self, 8)
    }
    #[doc = "Bit 9 - Filter Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn filterbufv(&mut self) -> FilterbufvW<StatusSpec> {
        FilterbufvW::new(self, 9)
    }
    #[doc = "Bit 12 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv0(&mut self) -> Ccbufv0W<StatusSpec> {
        Ccbufv0W::new(self, 12)
    }
    #[doc = "Bit 13 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn ccbufv1(&mut self) -> Ccbufv1W<StatusSpec> {
        Ccbufv1W::new(self, 13)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x40"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u16 = 0x40;
}
