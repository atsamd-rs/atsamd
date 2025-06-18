#[doc = "Register `PSTATUSCLR` writer"]
pub type W = crate::W<PstatusclrSpec>;
#[doc = "Field `DTGL` writer - Data Toggle clear"]
pub type DtglW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURBK` writer - Curren Bank clear"]
pub type CurbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFREEZE` writer - Pipe Freeze Clear"]
pub type PfreezeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK0RDY` writer - Bank 0 Ready Clear"]
pub type Bk0rdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK1RDY` writer - Bank 1 Ready Clear"]
pub type Bk1rdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Data Toggle clear"]
    #[inline(always)]
    pub fn dtgl(&mut self) -> DtglW<PstatusclrSpec> {
        DtglW::new(self, 0)
    }
    #[doc = "Bit 2 - Curren Bank clear"]
    #[inline(always)]
    pub fn curbk(&mut self) -> CurbkW<PstatusclrSpec> {
        CurbkW::new(self, 2)
    }
    #[doc = "Bit 4 - Pipe Freeze Clear"]
    #[inline(always)]
    pub fn pfreeze(&mut self) -> PfreezeW<PstatusclrSpec> {
        PfreezeW::new(self, 4)
    }
    #[doc = "Bit 6 - Bank 0 Ready Clear"]
    #[inline(always)]
    pub fn bk0rdy(&mut self) -> Bk0rdyW<PstatusclrSpec> {
        Bk0rdyW::new(self, 6)
    }
    #[doc = "Bit 7 - Bank 1 Ready Clear"]
    #[inline(always)]
    pub fn bk1rdy(&mut self) -> Bk1rdyW<PstatusclrSpec> {
        Bk1rdyW::new(self, 7)
    }
}
#[doc = "HOST_PIPE End Point Pipe Status Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pstatusclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PstatusclrSpec;
impl crate::RegisterSpec for PstatusclrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`pstatusclr::W`](W) writer structure"]
impl crate::Writable for PstatusclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSTATUSCLR to value 0"]
impl crate::Resettable for PstatusclrSpec {}
