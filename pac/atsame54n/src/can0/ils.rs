#[doc = "Register `ILS` reader"]
pub type R = crate::R<IlsSpec>;
#[doc = "Register `ILS` writer"]
pub type W = crate::W<IlsSpec>;
#[doc = "Field `RF0NL` reader - Rx FIFO 0 New Message Interrupt Line"]
pub type Rf0nlR = crate::BitReader;
#[doc = "Field `RF0NL` writer - Rx FIFO 0 New Message Interrupt Line"]
pub type Rf0nlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0WL` reader - Rx FIFO 0 Watermark Reached Interrupt Line"]
pub type Rf0wlR = crate::BitReader;
#[doc = "Field `RF0WL` writer - Rx FIFO 0 Watermark Reached Interrupt Line"]
pub type Rf0wlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0FL` reader - Rx FIFO 0 Full Interrupt Line"]
pub type Rf0flR = crate::BitReader;
#[doc = "Field `RF0FL` writer - Rx FIFO 0 Full Interrupt Line"]
pub type Rf0flW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0LL` reader - Rx FIFO 0 Message Lost Interrupt Line"]
pub type Rf0llR = crate::BitReader;
#[doc = "Field `RF0LL` writer - Rx FIFO 0 Message Lost Interrupt Line"]
pub type Rf0llW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1NL` reader - Rx FIFO 1 New Message Interrupt Line"]
pub type Rf1nlR = crate::BitReader;
#[doc = "Field `RF1NL` writer - Rx FIFO 1 New Message Interrupt Line"]
pub type Rf1nlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1WL` reader - Rx FIFO 1 Watermark Reached Interrupt Line"]
pub type Rf1wlR = crate::BitReader;
#[doc = "Field `RF1WL` writer - Rx FIFO 1 Watermark Reached Interrupt Line"]
pub type Rf1wlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1FL` reader - Rx FIFO 1 FIFO Full Interrupt Line"]
pub type Rf1flR = crate::BitReader;
#[doc = "Field `RF1FL` writer - Rx FIFO 1 FIFO Full Interrupt Line"]
pub type Rf1flW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1LL` reader - Rx FIFO 1 Message Lost Interrupt Line"]
pub type Rf1llR = crate::BitReader;
#[doc = "Field `RF1LL` writer - Rx FIFO 1 Message Lost Interrupt Line"]
pub type Rf1llW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPML` reader - High Priority Message Interrupt Line"]
pub type HpmlR = crate::BitReader;
#[doc = "Field `HPML` writer - High Priority Message Interrupt Line"]
pub type HpmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCL` reader - Timestamp Completed Interrupt Line"]
pub type TclR = crate::BitReader;
#[doc = "Field `TCL` writer - Timestamp Completed Interrupt Line"]
pub type TclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCFL` reader - Transmission Cancellation Finished Interrupt Line"]
pub type TcflR = crate::BitReader;
#[doc = "Field `TCFL` writer - Transmission Cancellation Finished Interrupt Line"]
pub type TcflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFEL` reader - Tx FIFO Empty Interrupt Line"]
pub type TfelR = crate::BitReader;
#[doc = "Field `TFEL` writer - Tx FIFO Empty Interrupt Line"]
pub type TfelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFNL` reader - Tx Event FIFO New Entry Interrupt Line"]
pub type TefnlR = crate::BitReader;
#[doc = "Field `TEFNL` writer - Tx Event FIFO New Entry Interrupt Line"]
pub type TefnlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFWL` reader - Tx Event FIFO Watermark Reached Interrupt Line"]
pub type TefwlR = crate::BitReader;
#[doc = "Field `TEFWL` writer - Tx Event FIFO Watermark Reached Interrupt Line"]
pub type TefwlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFFL` reader - Tx Event FIFO Full Interrupt Line"]
pub type TefflR = crate::BitReader;
#[doc = "Field `TEFFL` writer - Tx Event FIFO Full Interrupt Line"]
pub type TefflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFLL` reader - Tx Event FIFO Element Lost Interrupt Line"]
pub type TefllR = crate::BitReader;
#[doc = "Field `TEFLL` writer - Tx Event FIFO Element Lost Interrupt Line"]
pub type TefllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSWL` reader - Timestamp Wraparound Interrupt Line"]
pub type TswlR = crate::BitReader;
#[doc = "Field `TSWL` writer - Timestamp Wraparound Interrupt Line"]
pub type TswlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRAFL` reader - Message RAM Access Failure Interrupt Line"]
pub type MraflR = crate::BitReader;
#[doc = "Field `MRAFL` writer - Message RAM Access Failure Interrupt Line"]
pub type MraflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOOL` reader - Timeout Occurred Interrupt Line"]
pub type ToolR = crate::BitReader;
#[doc = "Field `TOOL` writer - Timeout Occurred Interrupt Line"]
pub type ToolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRXL` reader - Message stored to Dedicated Rx Buffer Interrupt Line"]
pub type DrxlR = crate::BitReader;
#[doc = "Field `DRXL` writer - Message stored to Dedicated Rx Buffer Interrupt Line"]
pub type DrxlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BECL` reader - Bit Error Corrected Interrupt Line"]
pub type BeclR = crate::BitReader;
#[doc = "Field `BECL` writer - Bit Error Corrected Interrupt Line"]
pub type BeclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEUL` reader - Bit Error Uncorrected Interrupt Line"]
pub type BeulR = crate::BitReader;
#[doc = "Field `BEUL` writer - Bit Error Uncorrected Interrupt Line"]
pub type BeulW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELOL` reader - Error Logging Overflow Interrupt Line"]
pub type ElolR = crate::BitReader;
#[doc = "Field `ELOL` writer - Error Logging Overflow Interrupt Line"]
pub type ElolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPL` reader - Error Passive Interrupt Line"]
pub type EplR = crate::BitReader;
#[doc = "Field `EPL` writer - Error Passive Interrupt Line"]
pub type EplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWL` reader - Warning Status Interrupt Line"]
pub type EwlR = crate::BitReader;
#[doc = "Field `EWL` writer - Warning Status Interrupt Line"]
pub type EwlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOL` reader - Bus_Off Status Interrupt Line"]
pub type BolR = crate::BitReader;
#[doc = "Field `BOL` writer - Bus_Off Status Interrupt Line"]
pub type BolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDIL` reader - Watchdog Interrupt Interrupt Line"]
pub type WdilR = crate::BitReader;
#[doc = "Field `WDIL` writer - Watchdog Interrupt Interrupt Line"]
pub type WdilW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEAL` reader - Protocol Error in Arbitration Phase Line"]
pub type PealR = crate::BitReader;
#[doc = "Field `PEAL` writer - Protocol Error in Arbitration Phase Line"]
pub type PealW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEDL` reader - Protocol Error in Data Phase Line"]
pub type PedlR = crate::BitReader;
#[doc = "Field `PEDL` writer - Protocol Error in Data Phase Line"]
pub type PedlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARAL` reader - Access to Reserved Address Line"]
pub type AralR = crate::BitReader;
#[doc = "Field `ARAL` writer - Access to Reserved Address Line"]
pub type AralW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Interrupt Line"]
    #[inline(always)]
    pub fn rf0nl(&self) -> Rf0nlR {
        Rf0nlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub fn rf0wl(&self) -> Rf0wlR {
        Rf0wlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Interrupt Line"]
    #[inline(always)]
    pub fn rf0fl(&self) -> Rf0flR {
        Rf0flR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Interrupt Line"]
    #[inline(always)]
    pub fn rf0ll(&self) -> Rf0llR {
        Rf0llR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Interrupt Line"]
    #[inline(always)]
    pub fn rf1nl(&self) -> Rf1nlR {
        Rf1nlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub fn rf1wl(&self) -> Rf1wlR {
        Rf1wlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 FIFO Full Interrupt Line"]
    #[inline(always)]
    pub fn rf1fl(&self) -> Rf1flR {
        Rf1flR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Interrupt Line"]
    #[inline(always)]
    pub fn rf1ll(&self) -> Rf1llR {
        Rf1llR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message Interrupt Line"]
    #[inline(always)]
    pub fn hpml(&self) -> HpmlR {
        HpmlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Completed Interrupt Line"]
    #[inline(always)]
    pub fn tcl(&self) -> TclR {
        TclR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Interrupt Line"]
    #[inline(always)]
    pub fn tcfl(&self) -> TcflR {
        TcflR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Interrupt Line"]
    #[inline(always)]
    pub fn tfel(&self) -> TfelR {
        TfelR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Interrupt Line"]
    #[inline(always)]
    pub fn tefnl(&self) -> TefnlR {
        TefnlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub fn tefwl(&self) -> TefwlR {
        TefwlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Interrupt Line"]
    #[inline(always)]
    pub fn teffl(&self) -> TefflR {
        TefflR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Interrupt Line"]
    #[inline(always)]
    pub fn tefll(&self) -> TefllR {
        TefllR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Interrupt Line"]
    #[inline(always)]
    pub fn tswl(&self) -> TswlR {
        TswlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Interrupt Line"]
    #[inline(always)]
    pub fn mrafl(&self) -> MraflR {
        MraflR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred Interrupt Line"]
    #[inline(always)]
    pub fn tool(&self) -> ToolR {
        ToolR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer Interrupt Line"]
    #[inline(always)]
    pub fn drxl(&self) -> DrxlR {
        DrxlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit Error Corrected Interrupt Line"]
    #[inline(always)]
    pub fn becl(&self) -> BeclR {
        BeclR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Interrupt Line"]
    #[inline(always)]
    pub fn beul(&self) -> BeulR {
        BeulR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow Interrupt Line"]
    #[inline(always)]
    pub fn elol(&self) -> ElolR {
        ElolR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive Interrupt Line"]
    #[inline(always)]
    pub fn epl(&self) -> EplR {
        EplR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status Interrupt Line"]
    #[inline(always)]
    pub fn ewl(&self) -> EwlR {
        EwlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status Interrupt Line"]
    #[inline(always)]
    pub fn bol(&self) -> BolR {
        BolR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Interrupt Line"]
    #[inline(always)]
    pub fn wdil(&self) -> WdilR {
        WdilR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Line"]
    #[inline(always)]
    pub fn peal(&self) -> PealR {
        PealR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Line"]
    #[inline(always)]
    pub fn pedl(&self) -> PedlR {
        PedlR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address Line"]
    #[inline(always)]
    pub fn aral(&self) -> AralR {
        AralR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn rf0nl(&mut self) -> Rf0nlW<IlsSpec> {
        Rf0nlW::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn rf0wl(&mut self) -> Rf0wlW<IlsSpec> {
        Rf0wlW::new(self, 1)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn rf0fl(&mut self) -> Rf0flW<IlsSpec> {
        Rf0flW::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn rf0ll(&mut self) -> Rf0llW<IlsSpec> {
        Rf0llW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn rf1nl(&mut self) -> Rf1nlW<IlsSpec> {
        Rf1nlW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn rf1wl(&mut self) -> Rf1wlW<IlsSpec> {
        Rf1wlW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx FIFO 1 FIFO Full Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn rf1fl(&mut self) -> Rf1flW<IlsSpec> {
        Rf1flW::new(self, 6)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn rf1ll(&mut self) -> Rf1llW<IlsSpec> {
        Rf1llW::new(self, 7)
    }
    #[doc = "Bit 8 - High Priority Message Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn hpml(&mut self) -> HpmlW<IlsSpec> {
        HpmlW::new(self, 8)
    }
    #[doc = "Bit 9 - Timestamp Completed Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn tcl(&mut self) -> TclW<IlsSpec> {
        TclW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn tcfl(&mut self) -> TcflW<IlsSpec> {
        TcflW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn tfel(&mut self) -> TfelW<IlsSpec> {
        TfelW::new(self, 11)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn tefnl(&mut self) -> TefnlW<IlsSpec> {
        TefnlW::new(self, 12)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn tefwl(&mut self) -> TefwlW<IlsSpec> {
        TefwlW::new(self, 13)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn teffl(&mut self) -> TefflW<IlsSpec> {
        TefflW::new(self, 14)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn tefll(&mut self) -> TefllW<IlsSpec> {
        TefllW::new(self, 15)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn tswl(&mut self) -> TswlW<IlsSpec> {
        TswlW::new(self, 16)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn mrafl(&mut self) -> MraflW<IlsSpec> {
        MraflW::new(self, 17)
    }
    #[doc = "Bit 18 - Timeout Occurred Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn tool(&mut self) -> ToolW<IlsSpec> {
        ToolW::new(self, 18)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn drxl(&mut self) -> DrxlW<IlsSpec> {
        DrxlW::new(self, 19)
    }
    #[doc = "Bit 20 - Bit Error Corrected Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn becl(&mut self) -> BeclW<IlsSpec> {
        BeclW::new(self, 20)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn beul(&mut self) -> BeulW<IlsSpec> {
        BeulW::new(self, 21)
    }
    #[doc = "Bit 22 - Error Logging Overflow Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn elol(&mut self) -> ElolW<IlsSpec> {
        ElolW::new(self, 22)
    }
    #[doc = "Bit 23 - Error Passive Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn epl(&mut self) -> EplW<IlsSpec> {
        EplW::new(self, 23)
    }
    #[doc = "Bit 24 - Warning Status Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn ewl(&mut self) -> EwlW<IlsSpec> {
        EwlW::new(self, 24)
    }
    #[doc = "Bit 25 - Bus_Off Status Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn bol(&mut self) -> BolW<IlsSpec> {
        BolW::new(self, 25)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn wdil(&mut self) -> WdilW<IlsSpec> {
        WdilW::new(self, 26)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Line"]
    #[inline(always)]
    #[must_use]
    pub fn peal(&mut self) -> PealW<IlsSpec> {
        PealW::new(self, 27)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Line"]
    #[inline(always)]
    #[must_use]
    pub fn pedl(&mut self) -> PedlW<IlsSpec> {
        PedlW::new(self, 28)
    }
    #[doc = "Bit 29 - Access to Reserved Address Line"]
    #[inline(always)]
    #[must_use]
    pub fn aral(&mut self) -> AralW<IlsSpec> {
        AralW::new(self, 29)
    }
}
#[doc = "Interrupt Line Select\n\nYou can [`read`](crate::Reg::read) this register and get [`ils::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ils::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IlsSpec;
impl crate::RegisterSpec for IlsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ils::R`](R) reader structure"]
impl crate::Readable for IlsSpec {}
#[doc = "`write(|w| ..)` method takes [`ils::W`](W) writer structure"]
impl crate::Writable for IlsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ILS to value 0"]
impl crate::Resettable for IlsSpec {
    const RESET_VALUE: u32 = 0;
}
