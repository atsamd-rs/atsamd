#[doc = "Register `PINTENSET%s` reader"]
pub type R = crate::R<PintensetSpec>;
#[doc = "Register `PINTENSET%s` writer"]
pub type W = crate::W<PintensetSpec>;
#[doc = "Field `TRCPT0` reader - Transfer Complete 0 Interrupt Enable"]
pub type Trcpt0R = crate::BitReader;
#[doc = "Field `TRCPT0` writer - Transfer Complete 0 Interrupt Enable"]
pub type Trcpt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCPT1` reader - Transfer Complete 1 Interrupt Enable"]
pub type Trcpt1R = crate::BitReader;
#[doc = "Field `TRCPT1` writer - Transfer Complete 1 Interrupt Enable"]
pub type Trcpt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRFAIL` reader - Error Flow Interrupt Enable"]
pub type TrfailR = crate::BitReader;
#[doc = "Field `TRFAIL` writer - Error Flow Interrupt Enable"]
pub type TrfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` reader - Pipe Error Interrupt Enable"]
pub type PerrR = crate::BitReader;
#[doc = "Field `PERR` writer - Pipe Error Interrupt Enable"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTP` reader - Transmit Setup Interrupt Enable"]
pub type TxstpR = crate::BitReader;
#[doc = "Field `TXSTP` writer - Transmit Setup Interrupt Enable"]
pub type TxstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - Stall Interrupt Enable"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - Stall Interrupt Enable"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - Error Flow Interrupt Enable"]
    #[inline(always)]
    pub fn trfail(&self) -> TrfailR {
        TrfailR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Enable"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Setup Interrupt Enable"]
    #[inline(always)]
    pub fn txstp(&self) -> TxstpR {
        TxstpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stall Interrupt Enable"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Complete 0 Interrupt Enable"]
    #[inline(always)]
    pub fn trcpt0(&mut self) -> Trcpt0W<PintensetSpec> {
        Trcpt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Enable"]
    #[inline(always)]
    pub fn trcpt1(&mut self) -> Trcpt1W<PintensetSpec> {
        Trcpt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Error Flow Interrupt Enable"]
    #[inline(always)]
    pub fn trfail(&mut self) -> TrfailW<PintensetSpec> {
        TrfailW::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Enable"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<PintensetSpec> {
        PerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit Setup Interrupt Enable"]
    #[inline(always)]
    pub fn txstp(&mut self) -> TxstpW<PintensetSpec> {
        TxstpW::new(self, 4)
    }
    #[doc = "Bit 5 - Stall Interrupt Enable"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<PintensetSpec> {
        StallW::new(self, 5)
    }
}
#[doc = "HOST Pipe Interrupt Flag Set\n\nYou can [`read`](crate::Reg::read) this register and get [`pintenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pintenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PintensetSpec;
impl crate::RegisterSpec for PintensetSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pintenset::R`](R) reader structure"]
impl crate::Readable for PintensetSpec {}
#[doc = "`write(|w| ..)` method takes [`pintenset::W`](W) writer structure"]
impl crate::Writable for PintensetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINTENSET%s to value 0"]
impl crate::Resettable for PintensetSpec {}
