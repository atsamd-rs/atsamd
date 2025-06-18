#[doc = "Register `PSTATUSSET` writer"]
pub type W = crate::W<PstatussetSpec>;
#[doc = "Field `DTGL` writer - Data Toggle Set"]
pub type DtglW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURBK` writer - Current Bank Set"]
pub type CurbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFREEZE` writer - Pipe Freeze Set"]
pub type PfreezeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK0RDY` writer - Bank 0 Ready Set"]
pub type Bk0rdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK1RDY` writer - Bank 1 Ready Set"]
pub type Bk1rdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Data Toggle Set"]
    #[inline(always)]
    pub fn dtgl(&mut self) -> DtglW<PstatussetSpec> {
        DtglW::new(self, 0)
    }
    #[doc = "Bit 2 - Current Bank Set"]
    #[inline(always)]
    pub fn curbk(&mut self) -> CurbkW<PstatussetSpec> {
        CurbkW::new(self, 2)
    }
    #[doc = "Bit 4 - Pipe Freeze Set"]
    #[inline(always)]
    pub fn pfreeze(&mut self) -> PfreezeW<PstatussetSpec> {
        PfreezeW::new(self, 4)
    }
    #[doc = "Bit 6 - Bank 0 Ready Set"]
    #[inline(always)]
    pub fn bk0rdy(&mut self) -> Bk0rdyW<PstatussetSpec> {
        Bk0rdyW::new(self, 6)
    }
    #[doc = "Bit 7 - Bank 1 Ready Set"]
    #[inline(always)]
    pub fn bk1rdy(&mut self) -> Bk1rdyW<PstatussetSpec> {
        Bk1rdyW::new(self, 7)
    }
}
#[doc = "HOST_PIPE End Point Pipe Status Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pstatusset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PstatussetSpec;
impl crate::RegisterSpec for PstatussetSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`pstatusset::W`](W) writer structure"]
impl crate::Writable for PstatussetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSTATUSSET to value 0"]
impl crate::Resettable for PstatussetSpec {}
