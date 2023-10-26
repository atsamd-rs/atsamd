#[doc = "Register `NCR` reader"]
pub type R = crate::R<NCR_SPEC>;
#[doc = "Register `NCR` writer"]
pub type W = crate::W<NCR_SPEC>;
#[doc = "Field `LBL` reader - Loop Back Local"]
pub type LBL_R = crate::BitReader;
#[doc = "Field `LBL` writer - Loop Back Local"]
pub type LBL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXEN` reader - Receive Enable"]
pub type RXEN_R = crate::BitReader;
#[doc = "Field `RXEN` writer - Receive Enable"]
pub type RXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXEN` reader - Transmit Enable"]
pub type TXEN_R = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmit Enable"]
pub type TXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MPE` reader - Management Port Enable"]
pub type MPE_R = crate::BitReader;
#[doc = "Field `MPE` writer - Management Port Enable"]
pub type MPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLRSTAT` reader - Clear Statistics Registers"]
pub type CLRSTAT_R = crate::BitReader;
#[doc = "Field `CLRSTAT` writer - Clear Statistics Registers"]
pub type CLRSTAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INCSTAT` reader - Increment Statistics Registers"]
pub type INCSTAT_R = crate::BitReader;
#[doc = "Field `INCSTAT` writer - Increment Statistics Registers"]
pub type INCSTAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WESTAT` reader - Write Enable for Statistics Registers"]
pub type WESTAT_R = crate::BitReader;
#[doc = "Field `WESTAT` writer - Write Enable for Statistics Registers"]
pub type WESTAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BP` reader - Back pressure"]
pub type BP_R = crate::BitReader;
#[doc = "Field `BP` writer - Back pressure"]
pub type BP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSTART` reader - Start Transmission"]
pub type TSTART_R = crate::BitReader;
#[doc = "Field `TSTART` writer - Start Transmission"]
pub type TSTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `THALT` reader - Transmit Halt"]
pub type THALT_R = crate::BitReader;
#[doc = "Field `THALT` writer - Transmit Halt"]
pub type THALT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXPF` reader - Transmit Pause Frame"]
pub type TXPF_R = crate::BitReader;
#[doc = "Field `TXPF` writer - Transmit Pause Frame"]
pub type TXPF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXZQPF` reader - Transmit Zero Quantum Pause Frame"]
pub type TXZQPF_R = crate::BitReader;
#[doc = "Field `TXZQPF` writer - Transmit Zero Quantum Pause Frame"]
pub type TXZQPF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRTSM` reader - Store Receive Time Stamp to Memory"]
pub type SRTSM_R = crate::BitReader;
#[doc = "Field `SRTSM` writer - Store Receive Time Stamp to Memory"]
pub type SRTSM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENPBPR` reader - Enable PFC Priority-based Pause Reception"]
pub type ENPBPR_R = crate::BitReader;
#[doc = "Field `ENPBPR` writer - Enable PFC Priority-based Pause Reception"]
pub type ENPBPR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXPBPF` reader - Transmit PFC Priority-based Pause Frame"]
pub type TXPBPF_R = crate::BitReader;
#[doc = "Field `TXPBPF` writer - Transmit PFC Priority-based Pause Frame"]
pub type TXPBPF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FNP` reader - Flush Next Packet"]
pub type FNP_R = crate::BitReader;
#[doc = "Field `FNP` writer - Flush Next Packet"]
pub type FNP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPI` reader - Low Power Idle Enable"]
pub type LPI_R = crate::BitReader;
#[doc = "Field `LPI` writer - Low Power Idle Enable"]
pub type LPI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Loop Back Local"]
    #[inline(always)]
    pub fn lbl(&self) -> LBL_R {
        LBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Management Port Enable"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear Statistics Registers"]
    #[inline(always)]
    pub fn clrstat(&self) -> CLRSTAT_R {
        CLRSTAT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Increment Statistics Registers"]
    #[inline(always)]
    pub fn incstat(&self) -> INCSTAT_R {
        INCSTAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Enable for Statistics Registers"]
    #[inline(always)]
    pub fn westat(&self) -> WESTAT_R {
        WESTAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    pub fn bp(&self) -> BP_R {
        BP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start Transmission"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Halt"]
    #[inline(always)]
    pub fn thalt(&self) -> THALT_R {
        THALT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit Pause Frame"]
    #[inline(always)]
    pub fn txpf(&self) -> TXPF_R {
        TXPF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Zero Quantum Pause Frame"]
    #[inline(always)]
    pub fn txzqpf(&self) -> TXZQPF_R {
        TXZQPF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Store Receive Time Stamp to Memory"]
    #[inline(always)]
    pub fn srtsm(&self) -> SRTSM_R {
        SRTSM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable PFC Priority-based Pause Reception"]
    #[inline(always)]
    pub fn enpbpr(&self) -> ENPBPR_R {
        ENPBPR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit PFC Priority-based Pause Frame"]
    #[inline(always)]
    pub fn txpbpf(&self) -> TXPBPF_R {
        TXPBPF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Flush Next Packet"]
    #[inline(always)]
    pub fn fnp(&self) -> FNP_R {
        FNP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Low Power Idle Enable"]
    #[inline(always)]
    pub fn lpi(&self) -> LPI_R {
        LPI_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Loop Back Local"]
    #[inline(always)]
    #[must_use]
    pub fn lbl(&mut self) -> LBL_W<NCR_SPEC, 1> {
        LBL_W::new(self)
    }
    #[doc = "Bit 2 - Receive Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<NCR_SPEC, 2> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<NCR_SPEC, 3> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 4 - Management Port Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mpe(&mut self) -> MPE_W<NCR_SPEC, 4> {
        MPE_W::new(self)
    }
    #[doc = "Bit 5 - Clear Statistics Registers"]
    #[inline(always)]
    #[must_use]
    pub fn clrstat(&mut self) -> CLRSTAT_W<NCR_SPEC, 5> {
        CLRSTAT_W::new(self)
    }
    #[doc = "Bit 6 - Increment Statistics Registers"]
    #[inline(always)]
    #[must_use]
    pub fn incstat(&mut self) -> INCSTAT_W<NCR_SPEC, 6> {
        INCSTAT_W::new(self)
    }
    #[doc = "Bit 7 - Write Enable for Statistics Registers"]
    #[inline(always)]
    #[must_use]
    pub fn westat(&mut self) -> WESTAT_W<NCR_SPEC, 7> {
        WESTAT_W::new(self)
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    #[must_use]
    pub fn bp(&mut self) -> BP_W<NCR_SPEC, 8> {
        BP_W::new(self)
    }
    #[doc = "Bit 9 - Start Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TSTART_W<NCR_SPEC, 9> {
        TSTART_W::new(self)
    }
    #[doc = "Bit 10 - Transmit Halt"]
    #[inline(always)]
    #[must_use]
    pub fn thalt(&mut self) -> THALT_W<NCR_SPEC, 10> {
        THALT_W::new(self)
    }
    #[doc = "Bit 11 - Transmit Pause Frame"]
    #[inline(always)]
    #[must_use]
    pub fn txpf(&mut self) -> TXPF_W<NCR_SPEC, 11> {
        TXPF_W::new(self)
    }
    #[doc = "Bit 12 - Transmit Zero Quantum Pause Frame"]
    #[inline(always)]
    #[must_use]
    pub fn txzqpf(&mut self) -> TXZQPF_W<NCR_SPEC, 12> {
        TXZQPF_W::new(self)
    }
    #[doc = "Bit 15 - Store Receive Time Stamp to Memory"]
    #[inline(always)]
    #[must_use]
    pub fn srtsm(&mut self) -> SRTSM_W<NCR_SPEC, 15> {
        SRTSM_W::new(self)
    }
    #[doc = "Bit 16 - Enable PFC Priority-based Pause Reception"]
    #[inline(always)]
    #[must_use]
    pub fn enpbpr(&mut self) -> ENPBPR_W<NCR_SPEC, 16> {
        ENPBPR_W::new(self)
    }
    #[doc = "Bit 17 - Transmit PFC Priority-based Pause Frame"]
    #[inline(always)]
    #[must_use]
    pub fn txpbpf(&mut self) -> TXPBPF_W<NCR_SPEC, 17> {
        TXPBPF_W::new(self)
    }
    #[doc = "Bit 18 - Flush Next Packet"]
    #[inline(always)]
    #[must_use]
    pub fn fnp(&mut self) -> FNP_W<NCR_SPEC, 18> {
        FNP_W::new(self)
    }
    #[doc = "Bit 19 - Low Power Idle Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpi(&mut self) -> LPI_W<NCR_SPEC, 19> {
        LPI_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Network Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ncr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ncr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NCR_SPEC;
impl crate::RegisterSpec for NCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncr::R`](R) reader structure"]
impl crate::Readable for NCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ncr::W`](W) writer structure"]
impl crate::Writable for NCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NCR to value 0"]
impl crate::Resettable for NCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
