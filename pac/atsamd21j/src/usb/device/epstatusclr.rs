#[doc = "Register `EPSTATUSCLR%s` writer"]
pub type W = crate::W<EpstatusclrSpec>;
#[doc = "Field `DTGLOUT` writer - Data Toggle OUT Clear"]
pub type DtgloutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTGLIN` writer - Data Toggle IN Clear"]
pub type DtglinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURBK` writer - Curren Bank Clear"]
pub type CurbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLRQ0` writer - Stall 0 Request Clear"]
pub type Stallrq0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLRQ1` writer - Stall 1 Request Clear"]
pub type Stallrq1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK0RDY` writer - Bank 0 Ready Clear"]
pub type Bk0rdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK1RDY` writer - Bank 1 Ready Clear"]
pub type Bk1rdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Data Toggle OUT Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtglout(&mut self) -> DtgloutW<EpstatusclrSpec> {
        DtgloutW::new(self, 0)
    }
    #[doc = "Bit 1 - Data Toggle IN Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtglin(&mut self) -> DtglinW<EpstatusclrSpec> {
        DtglinW::new(self, 1)
    }
    #[doc = "Bit 2 - Curren Bank Clear"]
    #[inline(always)]
    #[must_use]
    pub fn curbk(&mut self) -> CurbkW<EpstatusclrSpec> {
        CurbkW::new(self, 2)
    }
    #[doc = "Bit 4 - Stall 0 Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stallrq0(&mut self) -> Stallrq0W<EpstatusclrSpec> {
        Stallrq0W::new(self, 4)
    }
    #[doc = "Bit 5 - Stall 1 Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stallrq1(&mut self) -> Stallrq1W<EpstatusclrSpec> {
        Stallrq1W::new(self, 5)
    }
    #[doc = "Bit 6 - Bank 0 Ready Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bk0rdy(&mut self) -> Bk0rdyW<EpstatusclrSpec> {
        Bk0rdyW::new(self, 6)
    }
    #[doc = "Bit 7 - Bank 1 Ready Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bk1rdy(&mut self) -> Bk1rdyW<EpstatusclrSpec> {
        Bk1rdyW::new(self, 7)
    }
}
#[doc = "DEVICE End Point Pipe Status Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epstatusclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpstatusclrSpec;
impl crate::RegisterSpec for EpstatusclrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`epstatusclr::W`](W) writer structure"]
impl crate::Writable for EpstatusclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EPSTATUSCLR%s to value 0"]
impl crate::Resettable for EpstatusclrSpec {
    const RESET_VALUE: u8 = 0;
}
