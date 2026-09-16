#[doc = "Register `PINTFLAG` reader"]
pub type R = crate::R<PintflagSpec>;
#[doc = "Register `PINTFLAG` writer"]
pub type W = crate::W<PintflagSpec>;
#[doc = "Field `TRCPT0` reader - Transfer Complete 0 Interrupt Flag"]
pub type Trcpt0R = crate::BitReader;
#[doc = "Field `TRCPT0` writer - Transfer Complete 0 Interrupt Flag"]
pub type Trcpt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCPT1` reader - Transfer Complete 1 Interrupt Flag"]
pub type Trcpt1R = crate::BitReader;
#[doc = "Field `TRCPT1` writer - Transfer Complete 1 Interrupt Flag"]
pub type Trcpt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRFAIL` reader - Error Flow Interrupt Flag"]
pub type TrfailR = crate::BitReader;
#[doc = "Field `TRFAIL` writer - Error Flow Interrupt Flag"]
pub type TrfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` reader - Pipe Error Interrupt Flag"]
pub type PerrR = crate::BitReader;
#[doc = "Field `PERR` writer - Pipe Error Interrupt Flag"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTP` reader - Transmit Setup Interrupt Flag"]
pub type TxstpR = crate::BitReader;
#[doc = "Field `TXSTP` writer - Transmit Setup Interrupt Flag"]
pub type TxstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - Stall Interrupt Flag"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - Stall Interrupt Flag"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Complete 0 Interrupt Flag"]
    #[inline(always)]
    pub fn trcpt0(&self) -> Trcpt0R {
        Trcpt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Flag"]
    #[inline(always)]
    pub fn trcpt1(&self) -> Trcpt1R {
        Trcpt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Flow Interrupt Flag"]
    #[inline(always)]
    pub fn trfail(&self) -> TrfailR {
        TrfailR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Setup Interrupt Flag"]
    #[inline(always)]
    pub fn txstp(&self) -> TxstpR {
        TxstpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stall Interrupt Flag"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Complete 0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn trcpt0(&mut self) -> Trcpt0W<PintflagSpec> {
        Trcpt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn trcpt1(&mut self) -> Trcpt1W<PintflagSpec> {
        Trcpt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Error Flow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn trfail(&mut self) -> TrfailW<PintflagSpec> {
        TrfailW::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PerrW<PintflagSpec> {
        PerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit Setup Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txstp(&mut self) -> TxstpW<PintflagSpec> {
        TxstpW::new(self, 4)
    }
    #[doc = "Bit 5 - Stall Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> StallW<PintflagSpec> {
        StallW::new(self, 5)
    }
}
#[doc = "HOST_PIPE Pipe Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`pintflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pintflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PintflagSpec;
impl crate::RegisterSpec for PintflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pintflag::R`](R) reader structure"]
impl crate::Readable for PintflagSpec {}
#[doc = "`write(|w| ..)` method takes [`pintflag::W`](W) writer structure"]
impl crate::Writable for PintflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PINTFLAG to value 0"]
impl crate::Resettable for PintflagSpec {
    const RESET_VALUE: u8 = 0;
}
