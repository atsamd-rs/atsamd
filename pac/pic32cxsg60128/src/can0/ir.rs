#[doc = "Register `IR` reader"]
pub type R = crate::R<IrSpec>;
#[doc = "Register `IR` writer"]
pub type W = crate::W<IrSpec>;
#[doc = "Field `RF0N` reader - Rx FIFO 0 New Message"]
pub type Rf0nR = crate::BitReader;
#[doc = "Field `RF0N` writer - Rx FIFO 0 New Message"]
pub type Rf0nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0W` reader - Rx FIFO 0 Watermark Reached"]
pub type Rf0wR = crate::BitReader;
#[doc = "Field `RF0W` writer - Rx FIFO 0 Watermark Reached"]
pub type Rf0wW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0F` reader - Rx FIFO 0 Full"]
pub type Rf0fR = crate::BitReader;
#[doc = "Field `RF0F` writer - Rx FIFO 0 Full"]
pub type Rf0fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0L` reader - Rx FIFO 0 Message Lost"]
pub type Rf0lR = crate::BitReader;
#[doc = "Field `RF0L` writer - Rx FIFO 0 Message Lost"]
pub type Rf0lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1N` reader - Rx FIFO 1 New Message"]
pub type Rf1nR = crate::BitReader;
#[doc = "Field `RF1N` writer - Rx FIFO 1 New Message"]
pub type Rf1nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1W` reader - Rx FIFO 1 Watermark Reached"]
pub type Rf1wR = crate::BitReader;
#[doc = "Field `RF1W` writer - Rx FIFO 1 Watermark Reached"]
pub type Rf1wW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1F` reader - Rx FIFO 1 FIFO Full"]
pub type Rf1fR = crate::BitReader;
#[doc = "Field `RF1F` writer - Rx FIFO 1 FIFO Full"]
pub type Rf1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1L` reader - Rx FIFO 1 Message Lost"]
pub type Rf1lR = crate::BitReader;
#[doc = "Field `RF1L` writer - Rx FIFO 1 Message Lost"]
pub type Rf1lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPM` reader - High Priority Message"]
pub type HpmR = crate::BitReader;
#[doc = "Field `HPM` writer - High Priority Message"]
pub type HpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - Timestamp Completed"]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - Timestamp Completed"]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCF` reader - Transmission Cancellation Finished"]
pub type TcfR = crate::BitReader;
#[doc = "Field `TCF` writer - Transmission Cancellation Finished"]
pub type TcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFE` reader - Tx FIFO Empty"]
pub type TfeR = crate::BitReader;
#[doc = "Field `TFE` writer - Tx FIFO Empty"]
pub type TfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFN` reader - Tx Event FIFO New Entry"]
pub type TefnR = crate::BitReader;
#[doc = "Field `TEFN` writer - Tx Event FIFO New Entry"]
pub type TefnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFW` reader - Tx Event FIFO Watermark Reached"]
pub type TefwR = crate::BitReader;
#[doc = "Field `TEFW` writer - Tx Event FIFO Watermark Reached"]
pub type TefwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFF` reader - Tx Event FIFO Full"]
pub type TeffR = crate::BitReader;
#[doc = "Field `TEFF` writer - Tx Event FIFO Full"]
pub type TeffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFL` reader - Tx Event FIFO Element Lost"]
pub type TeflR = crate::BitReader;
#[doc = "Field `TEFL` writer - Tx Event FIFO Element Lost"]
pub type TeflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSW` reader - Timestamp Wraparound"]
pub type TswR = crate::BitReader;
#[doc = "Field `TSW` writer - Timestamp Wraparound"]
pub type TswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRAF` reader - Message RAM Access Failure"]
pub type MrafR = crate::BitReader;
#[doc = "Field `MRAF` writer - Message RAM Access Failure"]
pub type MrafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOO` reader - Timeout Occurred"]
pub type TooR = crate::BitReader;
#[doc = "Field `TOO` writer - Timeout Occurred"]
pub type TooW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRX` reader - Message stored to Dedicated Rx Buffer"]
pub type DrxR = crate::BitReader;
#[doc = "Field `DRX` writer - Message stored to Dedicated Rx Buffer"]
pub type DrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEC` reader - Bit Error Corrected"]
pub type BecR = crate::BitReader;
#[doc = "Field `BEC` writer - Bit Error Corrected"]
pub type BecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEU` reader - Bit Error Uncorrected"]
pub type BeuR = crate::BitReader;
#[doc = "Field `BEU` writer - Bit Error Uncorrected"]
pub type BeuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELO` reader - Error Logging Overflow"]
pub type EloR = crate::BitReader;
#[doc = "Field `ELO` writer - Error Logging Overflow"]
pub type EloW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP` reader - Error Passive"]
pub type EpR = crate::BitReader;
#[doc = "Field `EP` writer - Error Passive"]
pub type EpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW` reader - Warning Status"]
pub type EwR = crate::BitReader;
#[doc = "Field `EW` writer - Warning Status"]
pub type EwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BO` reader - Bus_Off Status"]
pub type BoR = crate::BitReader;
#[doc = "Field `BO` writer - Bus_Off Status"]
pub type BoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDI` reader - Watchdog Interrupt"]
pub type WdiR = crate::BitReader;
#[doc = "Field `WDI` writer - Watchdog Interrupt"]
pub type WdiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEA` reader - Protocol Error in Arbitration Phase"]
pub type PeaR = crate::BitReader;
#[doc = "Field `PEA` writer - Protocol Error in Arbitration Phase"]
pub type PeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PED` reader - Protocol Error in Data Phase"]
pub type PedR = crate::BitReader;
#[doc = "Field `PED` writer - Protocol Error in Data Phase"]
pub type PedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARA` reader - Access to Reserved Address"]
pub type AraR = crate::BitReader;
#[doc = "Field `ARA` writer - Access to Reserved Address"]
pub type AraW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message"]
    #[inline(always)]
    pub fn rf0n(&self) -> Rf0nR {
        Rf0nR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached"]
    #[inline(always)]
    pub fn rf0w(&self) -> Rf0wR {
        Rf0wR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn rf0f(&self) -> Rf0fR {
        Rf0fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> Rf0lR {
        Rf0lR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message"]
    #[inline(always)]
    pub fn rf1n(&self) -> Rf1nR {
        Rf1nR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    pub fn rf1w(&self) -> Rf1wR {
        Rf1wR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 FIFO Full"]
    #[inline(always)]
    pub fn rf1f(&self) -> Rf1fR {
        Rf1fR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> Rf1lR {
        Rf1lR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message"]
    #[inline(always)]
    pub fn hpm(&self) -> HpmR {
        HpmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Completed"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished"]
    #[inline(always)]
    pub fn tcf(&self) -> TcfR {
        TcfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry"]
    #[inline(always)]
    pub fn tefn(&self) -> TefnR {
        TefnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached"]
    #[inline(always)]
    pub fn tefw(&self) -> TefwR {
        TefwR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full"]
    #[inline(always)]
    pub fn teff(&self) -> TeffR {
        TeffR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TeflR {
        TeflR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound"]
    #[inline(always)]
    pub fn tsw(&self) -> TswR {
        TswR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure"]
    #[inline(always)]
    pub fn mraf(&self) -> MrafR {
        MrafR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred"]
    #[inline(always)]
    pub fn too(&self) -> TooR {
        TooR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer"]
    #[inline(always)]
    pub fn drx(&self) -> DrxR {
        DrxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit Error Corrected"]
    #[inline(always)]
    pub fn bec(&self) -> BecR {
        BecR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected"]
    #[inline(always)]
    pub fn beu(&self) -> BeuR {
        BeuR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow"]
    #[inline(always)]
    pub fn elo(&self) -> EloR {
        EloR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EwR {
        EwR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BoR {
        BoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn wdi(&self) -> WdiR {
        WdiR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase"]
    #[inline(always)]
    pub fn pea(&self) -> PeaR {
        PeaR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase"]
    #[inline(always)]
    pub fn ped(&self) -> PedR {
        PedR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address"]
    #[inline(always)]
    pub fn ara(&self) -> AraR {
        AraR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 New Message"]
    #[inline(always)]
    #[must_use]
    pub fn rf0n(&mut self) -> Rf0nW<IrSpec> {
        Rf0nW::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached"]
    #[inline(always)]
    #[must_use]
    pub fn rf0w(&mut self) -> Rf0wW<IrSpec> {
        Rf0wW::new(self, 1)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full"]
    #[inline(always)]
    #[must_use]
    pub fn rf0f(&mut self) -> Rf0fW<IrSpec> {
        Rf0fW::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> Rf0lW<IrSpec> {
        Rf0lW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message"]
    #[inline(always)]
    #[must_use]
    pub fn rf1n(&mut self) -> Rf1nW<IrSpec> {
        Rf1nW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    #[must_use]
    pub fn rf1w(&mut self) -> Rf1wW<IrSpec> {
        Rf1wW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx FIFO 1 FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn rf1f(&mut self) -> Rf1fW<IrSpec> {
        Rf1fW::new(self, 6)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf1l(&mut self) -> Rf1lW<IrSpec> {
        Rf1lW::new(self, 7)
    }
    #[doc = "Bit 8 - High Priority Message"]
    #[inline(always)]
    #[must_use]
    pub fn hpm(&mut self) -> HpmW<IrSpec> {
        HpmW::new(self, 8)
    }
    #[doc = "Bit 9 - Timestamp Completed"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TcW<IrSpec> {
        TcW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TcfW<IrSpec> {
        TcfW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx FIFO Empty"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TfeW<IrSpec> {
        TfeW::new(self, 11)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry"]
    #[inline(always)]
    #[must_use]
    pub fn tefn(&mut self) -> TefnW<IrSpec> {
        TefnW::new(self, 12)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached"]
    #[inline(always)]
    #[must_use]
    pub fn tefw(&mut self) -> TefwW<IrSpec> {
        TefwW::new(self, 13)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn teff(&mut self) -> TeffW<IrSpec> {
        TeffW::new(self, 14)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost"]
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TeflW<IrSpec> {
        TeflW::new(self, 15)
    }
    #[doc = "Bit 16 - Timestamp Wraparound"]
    #[inline(always)]
    #[must_use]
    pub fn tsw(&mut self) -> TswW<IrSpec> {
        TswW::new(self, 16)
    }
    #[doc = "Bit 17 - Message RAM Access Failure"]
    #[inline(always)]
    #[must_use]
    pub fn mraf(&mut self) -> MrafW<IrSpec> {
        MrafW::new(self, 17)
    }
    #[doc = "Bit 18 - Timeout Occurred"]
    #[inline(always)]
    #[must_use]
    pub fn too(&mut self) -> TooW<IrSpec> {
        TooW::new(self, 18)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn drx(&mut self) -> DrxW<IrSpec> {
        DrxW::new(self, 19)
    }
    #[doc = "Bit 20 - Bit Error Corrected"]
    #[inline(always)]
    #[must_use]
    pub fn bec(&mut self) -> BecW<IrSpec> {
        BecW::new(self, 20)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected"]
    #[inline(always)]
    #[must_use]
    pub fn beu(&mut self) -> BeuW<IrSpec> {
        BeuW::new(self, 21)
    }
    #[doc = "Bit 22 - Error Logging Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn elo(&mut self) -> EloW<IrSpec> {
        EloW::new(self, 22)
    }
    #[doc = "Bit 23 - Error Passive"]
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EpW<IrSpec> {
        EpW::new(self, 23)
    }
    #[doc = "Bit 24 - Warning Status"]
    #[inline(always)]
    #[must_use]
    pub fn ew(&mut self) -> EwW<IrSpec> {
        EwW::new(self, 24)
    }
    #[doc = "Bit 25 - Bus_Off Status"]
    #[inline(always)]
    #[must_use]
    pub fn bo(&mut self) -> BoW<IrSpec> {
        BoW::new(self, 25)
    }
    #[doc = "Bit 26 - Watchdog Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wdi(&mut self) -> WdiW<IrSpec> {
        WdiW::new(self, 26)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase"]
    #[inline(always)]
    #[must_use]
    pub fn pea(&mut self) -> PeaW<IrSpec> {
        PeaW::new(self, 27)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase"]
    #[inline(always)]
    #[must_use]
    pub fn ped(&mut self) -> PedW<IrSpec> {
        PedW::new(self, 28)
    }
    #[doc = "Bit 29 - Access to Reserved Address"]
    #[inline(always)]
    #[must_use]
    pub fn ara(&mut self) -> AraW<IrSpec> {
        AraW::new(self, 29)
    }
}
#[doc = "Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrSpec;
impl crate::RegisterSpec for IrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IrSpec {}
#[doc = "`write(|w| ..)` method takes [`ir::W`](W) writer structure"]
impl crate::Writable for IrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IrSpec {
    const RESET_VALUE: u32 = 0;
}
