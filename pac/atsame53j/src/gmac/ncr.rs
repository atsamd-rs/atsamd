#[doc = "Register `NCR` reader"]
pub struct R(crate::R<NCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NCR` writer"]
pub struct W(crate::W<NCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<NCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBL` reader - Loop Back Local"]
pub type LBL_R = crate::BitReader<bool>;
#[doc = "Field `LBL` writer - Loop Back Local"]
pub type LBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `RXEN` reader - Receive Enable"]
pub type RXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` writer - Receive Enable"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `TXEN` reader - Transmit Enable"]
pub type TXEN_R = crate::BitReader<bool>;
#[doc = "Field `TXEN` writer - Transmit Enable"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `MPE` reader - Management Port Enable"]
pub type MPE_R = crate::BitReader<bool>;
#[doc = "Field `MPE` writer - Management Port Enable"]
pub type MPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `CLRSTAT` reader - Clear Statistics Registers"]
pub type CLRSTAT_R = crate::BitReader<bool>;
#[doc = "Field `CLRSTAT` writer - Clear Statistics Registers"]
pub type CLRSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `INCSTAT` reader - Increment Statistics Registers"]
pub type INCSTAT_R = crate::BitReader<bool>;
#[doc = "Field `INCSTAT` writer - Increment Statistics Registers"]
pub type INCSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `WESTAT` reader - Write Enable for Statistics Registers"]
pub type WESTAT_R = crate::BitReader<bool>;
#[doc = "Field `WESTAT` writer - Write Enable for Statistics Registers"]
pub type WESTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `BP` reader - Back pressure"]
pub type BP_R = crate::BitReader<bool>;
#[doc = "Field `BP` writer - Back pressure"]
pub type BP_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `TSTART` reader - Start Transmission"]
pub type TSTART_R = crate::BitReader<bool>;
#[doc = "Field `TSTART` writer - Start Transmission"]
pub type TSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `THALT` reader - Transmit Halt"]
pub type THALT_R = crate::BitReader<bool>;
#[doc = "Field `THALT` writer - Transmit Halt"]
pub type THALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `TXPF` reader - Transmit Pause Frame"]
pub type TXPF_R = crate::BitReader<bool>;
#[doc = "Field `TXPF` writer - Transmit Pause Frame"]
pub type TXPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `TXZQPF` reader - Transmit Zero Quantum Pause Frame"]
pub type TXZQPF_R = crate::BitReader<bool>;
#[doc = "Field `TXZQPF` writer - Transmit Zero Quantum Pause Frame"]
pub type TXZQPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `SRTSM` reader - Store Receive Time Stamp to Memory"]
pub type SRTSM_R = crate::BitReader<bool>;
#[doc = "Field `SRTSM` writer - Store Receive Time Stamp to Memory"]
pub type SRTSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `ENPBPR` reader - Enable PFC Priority-based Pause Reception"]
pub type ENPBPR_R = crate::BitReader<bool>;
#[doc = "Field `ENPBPR` writer - Enable PFC Priority-based Pause Reception"]
pub type ENPBPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `TXPBPF` reader - Transmit PFC Priority-based Pause Frame"]
pub type TXPBPF_R = crate::BitReader<bool>;
#[doc = "Field `TXPBPF` writer - Transmit PFC Priority-based Pause Frame"]
pub type TXPBPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `FNP` reader - Flush Next Packet"]
pub type FNP_R = crate::BitReader<bool>;
#[doc = "Field `FNP` writer - Flush Next Packet"]
pub type FNP_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `LPI` reader - Low Power Idle Enable"]
pub type LPI_R = crate::BitReader<bool>;
#[doc = "Field `LPI` writer - Low Power Idle Enable"]
pub type LPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
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
    pub fn lbl(&mut self) -> LBL_W<1> {
        LBL_W::new(self)
    }
    #[doc = "Bit 2 - Receive Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<2> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<3> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 4 - Management Port Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mpe(&mut self) -> MPE_W<4> {
        MPE_W::new(self)
    }
    #[doc = "Bit 5 - Clear Statistics Registers"]
    #[inline(always)]
    #[must_use]
    pub fn clrstat(&mut self) -> CLRSTAT_W<5> {
        CLRSTAT_W::new(self)
    }
    #[doc = "Bit 6 - Increment Statistics Registers"]
    #[inline(always)]
    #[must_use]
    pub fn incstat(&mut self) -> INCSTAT_W<6> {
        INCSTAT_W::new(self)
    }
    #[doc = "Bit 7 - Write Enable for Statistics Registers"]
    #[inline(always)]
    #[must_use]
    pub fn westat(&mut self) -> WESTAT_W<7> {
        WESTAT_W::new(self)
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    #[must_use]
    pub fn bp(&mut self) -> BP_W<8> {
        BP_W::new(self)
    }
    #[doc = "Bit 9 - Start Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TSTART_W<9> {
        TSTART_W::new(self)
    }
    #[doc = "Bit 10 - Transmit Halt"]
    #[inline(always)]
    #[must_use]
    pub fn thalt(&mut self) -> THALT_W<10> {
        THALT_W::new(self)
    }
    #[doc = "Bit 11 - Transmit Pause Frame"]
    #[inline(always)]
    #[must_use]
    pub fn txpf(&mut self) -> TXPF_W<11> {
        TXPF_W::new(self)
    }
    #[doc = "Bit 12 - Transmit Zero Quantum Pause Frame"]
    #[inline(always)]
    #[must_use]
    pub fn txzqpf(&mut self) -> TXZQPF_W<12> {
        TXZQPF_W::new(self)
    }
    #[doc = "Bit 15 - Store Receive Time Stamp to Memory"]
    #[inline(always)]
    #[must_use]
    pub fn srtsm(&mut self) -> SRTSM_W<15> {
        SRTSM_W::new(self)
    }
    #[doc = "Bit 16 - Enable PFC Priority-based Pause Reception"]
    #[inline(always)]
    #[must_use]
    pub fn enpbpr(&mut self) -> ENPBPR_W<16> {
        ENPBPR_W::new(self)
    }
    #[doc = "Bit 17 - Transmit PFC Priority-based Pause Frame"]
    #[inline(always)]
    #[must_use]
    pub fn txpbpf(&mut self) -> TXPBPF_W<17> {
        TXPBPF_W::new(self)
    }
    #[doc = "Bit 18 - Flush Next Packet"]
    #[inline(always)]
    #[must_use]
    pub fn fnp(&mut self) -> FNP_W<18> {
        FNP_W::new(self)
    }
    #[doc = "Bit 19 - Low Power Idle Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpi(&mut self) -> LPI_W<19> {
        LPI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Network Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncr](index.html) module"]
pub struct NCR_SPEC;
impl crate::RegisterSpec for NCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ncr::R](R) reader structure"]
impl crate::Readable for NCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ncr::W](W) writer structure"]
impl crate::Writable for NCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NCR to value 0"]
impl crate::Resettable for NCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
