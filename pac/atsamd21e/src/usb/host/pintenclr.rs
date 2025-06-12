#[doc = "Register `PINTENCLR%s` reader"]
pub type R = crate::R<PintenclrSpec>;
#[doc = "Register `PINTENCLR%s` writer"]
pub type W = crate::W<PintenclrSpec>;
#[doc = "Field `TRCPT0` reader - Transfer Complete 0 Disable"]
pub type Trcpt0R = crate::BitReader;
#[doc = "Field `TRCPT0` writer - Transfer Complete 0 Disable"]
pub type Trcpt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCPT1` reader - Transfer Complete 1 Disable"]
pub type Trcpt1R = crate::BitReader;
#[doc = "Field `TRCPT1` writer - Transfer Complete 1 Disable"]
pub type Trcpt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRFAIL` reader - Error Flow Interrupt Disable"]
pub type TrfailR = crate::BitReader;
#[doc = "Field `TRFAIL` writer - Error Flow Interrupt Disable"]
pub type TrfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` reader - Pipe Error Interrupt Disable"]
pub type PerrR = crate::BitReader;
#[doc = "Field `PERR` writer - Pipe Error Interrupt Disable"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTP` reader - Transmit Setup Interrupt Disable"]
pub type TxstpR = crate::BitReader;
#[doc = "Field `TXSTP` writer - Transmit Setup Interrupt Disable"]
pub type TxstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - Stall Inetrrupt Disable"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - Stall Inetrrupt Disable"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Complete 0 Disable"]
    #[inline(always)]
    pub fn trcpt0(&self) -> Trcpt0R {
        Trcpt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Disable"]
    #[inline(always)]
    pub fn trcpt1(&self) -> Trcpt1R {
        Trcpt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Flow Interrupt Disable"]
    #[inline(always)]
    pub fn trfail(&self) -> TrfailR {
        TrfailR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Disable"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Setup Interrupt Disable"]
    #[inline(always)]
    pub fn txstp(&self) -> TxstpR {
        TxstpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stall Inetrrupt Disable"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Complete 0 Disable"]
    #[inline(always)]
    pub fn trcpt0(&mut self) -> Trcpt0W<PintenclrSpec> {
        Trcpt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Disable"]
    #[inline(always)]
    pub fn trcpt1(&mut self) -> Trcpt1W<PintenclrSpec> {
        Trcpt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Error Flow Interrupt Disable"]
    #[inline(always)]
    pub fn trfail(&mut self) -> TrfailW<PintenclrSpec> {
        TrfailW::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Disable"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<PintenclrSpec> {
        PerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit Setup Interrupt Disable"]
    #[inline(always)]
    pub fn txstp(&mut self) -> TxstpW<PintenclrSpec> {
        TxstpW::new(self, 4)
    }
    #[doc = "Bit 5 - Stall Inetrrupt Disable"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<PintenclrSpec> {
        StallW::new(self, 5)
    }
}
#[doc = "HOST Pipe Interrupt Flag Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`pintenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pintenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PintenclrSpec;
impl crate::RegisterSpec for PintenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pintenclr::R`](R) reader structure"]
impl crate::Readable for PintenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`pintenclr::W`](W) writer structure"]
impl crate::Writable for PintenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINTENCLR%s to value 0"]
impl crate::Resettable for PintenclrSpec {}
