#[doc = "Register `PINTFLAG%s` reader"]
pub type R = crate::R<PINTFLAG_SPEC>;
#[doc = "Register `PINTFLAG%s` writer"]
pub type W = crate::W<PINTFLAG_SPEC>;
#[doc = "Field `TRCPT0` reader - Transfer Complete 0 Interrupt Flag"]
pub type TRCPT0_R = crate::BitReader;
#[doc = "Field `TRCPT0` writer - Transfer Complete 0 Interrupt Flag"]
pub type TRCPT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRCPT1` reader - Transfer Complete 1 Interrupt Flag"]
pub type TRCPT1_R = crate::BitReader;
#[doc = "Field `TRCPT1` writer - Transfer Complete 1 Interrupt Flag"]
pub type TRCPT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRFAIL` reader - Error Flow Interrupt Flag"]
pub type TRFAIL_R = crate::BitReader;
#[doc = "Field `TRFAIL` writer - Error Flow Interrupt Flag"]
pub type TRFAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERR` reader - Pipe Error Interrupt Flag"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `PERR` writer - Pipe Error Interrupt Flag"]
pub type PERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXSTP` reader - Transmit Setup Interrupt Flag"]
pub type TXSTP_R = crate::BitReader;
#[doc = "Field `TXSTP` writer - Transmit Setup Interrupt Flag"]
pub type TXSTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALL` reader - Stall Interrupt Flag"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - Stall Interrupt Flag"]
pub type STALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Transfer Complete 0 Interrupt Flag"]
    #[inline(always)]
    pub fn trcpt0(&self) -> TRCPT0_R {
        TRCPT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Flag"]
    #[inline(always)]
    pub fn trcpt1(&self) -> TRCPT1_R {
        TRCPT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Flow Interrupt Flag"]
    #[inline(always)]
    pub fn trfail(&self) -> TRFAIL_R {
        TRFAIL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Setup Interrupt Flag"]
    #[inline(always)]
    pub fn txstp(&self) -> TXSTP_R {
        TXSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stall Interrupt Flag"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Complete 0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn trcpt0(&mut self) -> TRCPT0_W<PINTFLAG_SPEC, 0> {
        TRCPT0_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn trcpt1(&mut self) -> TRCPT1_W<PINTFLAG_SPEC, 1> {
        TRCPT1_W::new(self)
    }
    #[doc = "Bit 2 - Error Flow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn trfail(&mut self) -> TRFAIL_W<PINTFLAG_SPEC, 2> {
        TRFAIL_W::new(self)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<PINTFLAG_SPEC, 3> {
        PERR_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Setup Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txstp(&mut self) -> TXSTP_W<PINTFLAG_SPEC, 4> {
        TXSTP_W::new(self)
    }
    #[doc = "Bit 5 - Stall Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<PINTFLAG_SPEC, 5> {
        STALL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HOST Pipe Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pintflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pintflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINTFLAG_SPEC;
impl crate::RegisterSpec for PINTFLAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pintflag::R`](R) reader structure"]
impl crate::Readable for PINTFLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pintflag::W`](W) writer structure"]
impl crate::Writable for PINTFLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINTFLAG%s to value 0"]
impl crate::Resettable for PINTFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
