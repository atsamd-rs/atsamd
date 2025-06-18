#[doc = "Register `EPINTENSET` reader"]
pub type R = crate::R<EpintensetSpec>;
#[doc = "Register `EPINTENSET` writer"]
pub type W = crate::W<EpintensetSpec>;
#[doc = "Field `TRCPT0` reader - Transfer Complete 0 Interrupt Enable"]
pub type Trcpt0R = crate::BitReader;
#[doc = "Field `TRCPT0` writer - Transfer Complete 0 Interrupt Enable"]
pub type Trcpt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCPT1` reader - Transfer Complete 1 Interrupt Enable"]
pub type Trcpt1R = crate::BitReader;
#[doc = "Field `TRCPT1` writer - Transfer Complete 1 Interrupt Enable"]
pub type Trcpt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRFAIL0` reader - Error Flow 0 Interrupt Enable"]
pub type Trfail0R = crate::BitReader;
#[doc = "Field `TRFAIL0` writer - Error Flow 0 Interrupt Enable"]
pub type Trfail0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRFAIL1` reader - Error Flow 1 Interrupt Enable"]
pub type Trfail1R = crate::BitReader;
#[doc = "Field `TRFAIL1` writer - Error Flow 1 Interrupt Enable"]
pub type Trfail1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTP` reader - Received Setup Interrupt Enable"]
pub type RxstpR = crate::BitReader;
#[doc = "Field `RXSTP` writer - Received Setup Interrupt Enable"]
pub type RxstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL0` reader - Stall 0 In/out Interrupt enable"]
pub type Stall0R = crate::BitReader;
#[doc = "Field `STALL0` writer - Stall 0 In/out Interrupt enable"]
pub type Stall0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL1` reader - Stall 1 In/out Interrupt enable"]
pub type Stall1R = crate::BitReader;
#[doc = "Field `STALL1` writer - Stall 1 In/out Interrupt enable"]
pub type Stall1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Complete 0 Interrupt Enable"]
    #[inline(always)]
    pub fn trcpt0(&self) -> Trcpt0R {
        Trcpt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Enable"]
    #[inline(always)]
    pub fn trcpt1(&self) -> Trcpt1R {
        Trcpt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Flow 0 Interrupt Enable"]
    #[inline(always)]
    pub fn trfail0(&self) -> Trfail0R {
        Trfail0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Flow 1 Interrupt Enable"]
    #[inline(always)]
    pub fn trfail1(&self) -> Trfail1R {
        Trfail1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Received Setup Interrupt Enable"]
    #[inline(always)]
    pub fn rxstp(&self) -> RxstpR {
        RxstpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stall 0 In/out Interrupt enable"]
    #[inline(always)]
    pub fn stall0(&self) -> Stall0R {
        Stall0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stall 1 In/out Interrupt enable"]
    #[inline(always)]
    pub fn stall1(&self) -> Stall1R {
        Stall1R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Complete 0 Interrupt Enable"]
    #[inline(always)]
    pub fn trcpt0(&mut self) -> Trcpt0W<EpintensetSpec> {
        Trcpt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Enable"]
    #[inline(always)]
    pub fn trcpt1(&mut self) -> Trcpt1W<EpintensetSpec> {
        Trcpt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Error Flow 0 Interrupt Enable"]
    #[inline(always)]
    pub fn trfail0(&mut self) -> Trfail0W<EpintensetSpec> {
        Trfail0W::new(self, 2)
    }
    #[doc = "Bit 3 - Error Flow 1 Interrupt Enable"]
    #[inline(always)]
    pub fn trfail1(&mut self) -> Trfail1W<EpintensetSpec> {
        Trfail1W::new(self, 3)
    }
    #[doc = "Bit 4 - Received Setup Interrupt Enable"]
    #[inline(always)]
    pub fn rxstp(&mut self) -> RxstpW<EpintensetSpec> {
        RxstpW::new(self, 4)
    }
    #[doc = "Bit 5 - Stall 0 In/out Interrupt enable"]
    #[inline(always)]
    pub fn stall0(&mut self) -> Stall0W<EpintensetSpec> {
        Stall0W::new(self, 5)
    }
    #[doc = "Bit 6 - Stall 1 In/out Interrupt enable"]
    #[inline(always)]
    pub fn stall1(&mut self) -> Stall1W<EpintensetSpec> {
        Stall1W::new(self, 6)
    }
}
#[doc = "DEVICE_ENDPOINT End Point Interrupt Set Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`epintenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epintenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpintensetSpec;
impl crate::RegisterSpec for EpintensetSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`epintenset::R`](R) reader structure"]
impl crate::Readable for EpintensetSpec {}
#[doc = "`write(|w| ..)` method takes [`epintenset::W`](W) writer structure"]
impl crate::Writable for EpintensetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EPINTENSET to value 0"]
impl crate::Resettable for EpintensetSpec {}
