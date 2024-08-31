#[doc = "Register `EPINTENCLR%s` reader"]
pub type R = crate::R<EpintenclrSpec>;
#[doc = "Register `EPINTENCLR%s` writer"]
pub type W = crate::W<EpintenclrSpec>;
#[doc = "Field `TRCPT0` reader - Transfer Complete 0 Interrupt Disable"]
pub type Trcpt0R = crate::BitReader;
#[doc = "Field `TRCPT0` writer - Transfer Complete 0 Interrupt Disable"]
pub type Trcpt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCPT1` reader - Transfer Complete 1 Interrupt Disable"]
pub type Trcpt1R = crate::BitReader;
#[doc = "Field `TRCPT1` writer - Transfer Complete 1 Interrupt Disable"]
pub type Trcpt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRFAIL0` reader - Error Flow 0 Interrupt Disable"]
pub type Trfail0R = crate::BitReader;
#[doc = "Field `TRFAIL0` writer - Error Flow 0 Interrupt Disable"]
pub type Trfail0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRFAIL1` reader - Error Flow 1 Interrupt Disable"]
pub type Trfail1R = crate::BitReader;
#[doc = "Field `TRFAIL1` writer - Error Flow 1 Interrupt Disable"]
pub type Trfail1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTP` reader - Received Setup Interrupt Disable"]
pub type RxstpR = crate::BitReader;
#[doc = "Field `RXSTP` writer - Received Setup Interrupt Disable"]
pub type RxstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL0` reader - Stall 0 In/Out Interrupt Disable"]
pub type Stall0R = crate::BitReader;
#[doc = "Field `STALL0` writer - Stall 0 In/Out Interrupt Disable"]
pub type Stall0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL1` reader - Stall 1 In/Out Interrupt Disable"]
pub type Stall1R = crate::BitReader;
#[doc = "Field `STALL1` writer - Stall 1 In/Out Interrupt Disable"]
pub type Stall1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Complete 0 Interrupt Disable"]
    #[inline(always)]
    pub fn trcpt0(&self) -> Trcpt0R {
        Trcpt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Disable"]
    #[inline(always)]
    pub fn trcpt1(&self) -> Trcpt1R {
        Trcpt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Flow 0 Interrupt Disable"]
    #[inline(always)]
    pub fn trfail0(&self) -> Trfail0R {
        Trfail0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Flow 1 Interrupt Disable"]
    #[inline(always)]
    pub fn trfail1(&self) -> Trfail1R {
        Trfail1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Received Setup Interrupt Disable"]
    #[inline(always)]
    pub fn rxstp(&self) -> RxstpR {
        RxstpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stall 0 In/Out Interrupt Disable"]
    #[inline(always)]
    pub fn stall0(&self) -> Stall0R {
        Stall0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stall 1 In/Out Interrupt Disable"]
    #[inline(always)]
    pub fn stall1(&self) -> Stall1R {
        Stall1R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Complete 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trcpt0(&mut self) -> Trcpt0W<EpintenclrSpec> {
        Trcpt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trcpt1(&mut self) -> Trcpt1W<EpintenclrSpec> {
        Trcpt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Error Flow 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trfail0(&mut self) -> Trfail0W<EpintenclrSpec> {
        Trfail0W::new(self, 2)
    }
    #[doc = "Bit 3 - Error Flow 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trfail1(&mut self) -> Trfail1W<EpintenclrSpec> {
        Trfail1W::new(self, 3)
    }
    #[doc = "Bit 4 - Received Setup Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxstp(&mut self) -> RxstpW<EpintenclrSpec> {
        RxstpW::new(self, 4)
    }
    #[doc = "Bit 5 - Stall 0 In/Out Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn stall0(&mut self) -> Stall0W<EpintenclrSpec> {
        Stall0W::new(self, 5)
    }
    #[doc = "Bit 6 - Stall 1 In/Out Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn stall1(&mut self) -> Stall1W<EpintenclrSpec> {
        Stall1W::new(self, 6)
    }
}
#[doc = "DEVICE End Point Interrupt Clear Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`epintenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epintenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpintenclrSpec;
impl crate::RegisterSpec for EpintenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`epintenclr::R`](R) reader structure"]
impl crate::Readable for EpintenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`epintenclr::W`](W) writer structure"]
impl crate::Writable for EpintenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EPINTENCLR%s to value 0"]
impl crate::Resettable for EpintenclrSpec {
    const RESET_VALUE: u8 = 0;
}
