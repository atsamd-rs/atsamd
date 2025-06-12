#[doc = "Register `NCR` reader"]
pub type R = crate::R<NcrSpec>;
#[doc = "Register `NCR` writer"]
pub type W = crate::W<NcrSpec>;
#[doc = "Field `LBL` reader - Loop Back Local"]
pub type LblR = crate::BitReader;
#[doc = "Field `LBL` writer - Loop Back Local"]
pub type LblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - Receive Enable"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - Receive Enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` reader - Transmit Enable"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmit Enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPE` reader - Management Port Enable"]
pub type MpeR = crate::BitReader;
#[doc = "Field `MPE` writer - Management Port Enable"]
pub type MpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRSTAT` reader - Clear Statistics Registers"]
pub type ClrstatR = crate::BitReader;
#[doc = "Field `CLRSTAT` writer - Clear Statistics Registers"]
pub type ClrstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCSTAT` reader - Increment Statistics Registers"]
pub type IncstatR = crate::BitReader;
#[doc = "Field `INCSTAT` writer - Increment Statistics Registers"]
pub type IncstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WESTAT` reader - Write Enable for Statistics Registers"]
pub type WestatR = crate::BitReader;
#[doc = "Field `WESTAT` writer - Write Enable for Statistics Registers"]
pub type WestatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BP` reader - Back pressure"]
pub type BpR = crate::BitReader;
#[doc = "Field `BP` writer - Back pressure"]
pub type BpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTART` reader - Start Transmission"]
pub type TstartR = crate::BitReader;
#[doc = "Field `TSTART` writer - Start Transmission"]
pub type TstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THALT` reader - Transmit Halt"]
pub type ThaltR = crate::BitReader;
#[doc = "Field `THALT` writer - Transmit Halt"]
pub type ThaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPF` reader - Transmit Pause Frame"]
pub type TxpfR = crate::BitReader;
#[doc = "Field `TXPF` writer - Transmit Pause Frame"]
pub type TxpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXZQPF` reader - Transmit Zero Quantum Pause Frame"]
pub type TxzqpfR = crate::BitReader;
#[doc = "Field `TXZQPF` writer - Transmit Zero Quantum Pause Frame"]
pub type TxzqpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRTSM` reader - Store Receive Time Stamp to Memory"]
pub type SrtsmR = crate::BitReader;
#[doc = "Field `SRTSM` writer - Store Receive Time Stamp to Memory"]
pub type SrtsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENPBPR` reader - Enable PFC Priority-based Pause Reception"]
pub type EnpbprR = crate::BitReader;
#[doc = "Field `ENPBPR` writer - Enable PFC Priority-based Pause Reception"]
pub type EnpbprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPBPF` reader - Transmit PFC Priority-based Pause Frame"]
pub type TxpbpfR = crate::BitReader;
#[doc = "Field `TXPBPF` writer - Transmit PFC Priority-based Pause Frame"]
pub type TxpbpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FNP` reader - Flush Next Packet"]
pub type FnpR = crate::BitReader;
#[doc = "Field `FNP` writer - Flush Next Packet"]
pub type FnpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPI` reader - Low Power Idle Enable"]
pub type LpiR = crate::BitReader;
#[doc = "Field `LPI` writer - Low Power Idle Enable"]
pub type LpiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Loop Back Local"]
    #[inline(always)]
    pub fn lbl(&self) -> LblR {
        LblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Management Port Enable"]
    #[inline(always)]
    pub fn mpe(&self) -> MpeR {
        MpeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear Statistics Registers"]
    #[inline(always)]
    pub fn clrstat(&self) -> ClrstatR {
        ClrstatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Increment Statistics Registers"]
    #[inline(always)]
    pub fn incstat(&self) -> IncstatR {
        IncstatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Enable for Statistics Registers"]
    #[inline(always)]
    pub fn westat(&self) -> WestatR {
        WestatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    pub fn bp(&self) -> BpR {
        BpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start Transmission"]
    #[inline(always)]
    pub fn tstart(&self) -> TstartR {
        TstartR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Halt"]
    #[inline(always)]
    pub fn thalt(&self) -> ThaltR {
        ThaltR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit Pause Frame"]
    #[inline(always)]
    pub fn txpf(&self) -> TxpfR {
        TxpfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Zero Quantum Pause Frame"]
    #[inline(always)]
    pub fn txzqpf(&self) -> TxzqpfR {
        TxzqpfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Store Receive Time Stamp to Memory"]
    #[inline(always)]
    pub fn srtsm(&self) -> SrtsmR {
        SrtsmR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable PFC Priority-based Pause Reception"]
    #[inline(always)]
    pub fn enpbpr(&self) -> EnpbprR {
        EnpbprR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit PFC Priority-based Pause Frame"]
    #[inline(always)]
    pub fn txpbpf(&self) -> TxpbpfR {
        TxpbpfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Flush Next Packet"]
    #[inline(always)]
    pub fn fnp(&self) -> FnpR {
        FnpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Low Power Idle Enable"]
    #[inline(always)]
    pub fn lpi(&self) -> LpiR {
        LpiR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Loop Back Local"]
    #[inline(always)]
    pub fn lbl(&mut self) -> LblW<NcrSpec> {
        LblW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<NcrSpec> {
        RxenW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<NcrSpec> {
        TxenW::new(self, 3)
    }
    #[doc = "Bit 4 - Management Port Enable"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MpeW<NcrSpec> {
        MpeW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Statistics Registers"]
    #[inline(always)]
    pub fn clrstat(&mut self) -> ClrstatW<NcrSpec> {
        ClrstatW::new(self, 5)
    }
    #[doc = "Bit 6 - Increment Statistics Registers"]
    #[inline(always)]
    pub fn incstat(&mut self) -> IncstatW<NcrSpec> {
        IncstatW::new(self, 6)
    }
    #[doc = "Bit 7 - Write Enable for Statistics Registers"]
    #[inline(always)]
    pub fn westat(&mut self) -> WestatW<NcrSpec> {
        WestatW::new(self, 7)
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    pub fn bp(&mut self) -> BpW<NcrSpec> {
        BpW::new(self, 8)
    }
    #[doc = "Bit 9 - Start Transmission"]
    #[inline(always)]
    pub fn tstart(&mut self) -> TstartW<NcrSpec> {
        TstartW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmit Halt"]
    #[inline(always)]
    pub fn thalt(&mut self) -> ThaltW<NcrSpec> {
        ThaltW::new(self, 10)
    }
    #[doc = "Bit 11 - Transmit Pause Frame"]
    #[inline(always)]
    pub fn txpf(&mut self) -> TxpfW<NcrSpec> {
        TxpfW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit Zero Quantum Pause Frame"]
    #[inline(always)]
    pub fn txzqpf(&mut self) -> TxzqpfW<NcrSpec> {
        TxzqpfW::new(self, 12)
    }
    #[doc = "Bit 15 - Store Receive Time Stamp to Memory"]
    #[inline(always)]
    pub fn srtsm(&mut self) -> SrtsmW<NcrSpec> {
        SrtsmW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable PFC Priority-based Pause Reception"]
    #[inline(always)]
    pub fn enpbpr(&mut self) -> EnpbprW<NcrSpec> {
        EnpbprW::new(self, 16)
    }
    #[doc = "Bit 17 - Transmit PFC Priority-based Pause Frame"]
    #[inline(always)]
    pub fn txpbpf(&mut self) -> TxpbpfW<NcrSpec> {
        TxpbpfW::new(self, 17)
    }
    #[doc = "Bit 18 - Flush Next Packet"]
    #[inline(always)]
    pub fn fnp(&mut self) -> FnpW<NcrSpec> {
        FnpW::new(self, 18)
    }
    #[doc = "Bit 19 - Low Power Idle Enable"]
    #[inline(always)]
    pub fn lpi(&mut self) -> LpiW<NcrSpec> {
        LpiW::new(self, 19)
    }
}
#[doc = "Network Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ncr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NcrSpec;
impl crate::RegisterSpec for NcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncr::R`](R) reader structure"]
impl crate::Readable for NcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ncr::W`](W) writer structure"]
impl crate::Writable for NcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NCR to value 0"]
impl crate::Resettable for NcrSpec {}
