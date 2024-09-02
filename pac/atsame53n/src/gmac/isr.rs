#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `MFS` reader - Management Frame Sent"]
pub type MfsR = crate::BitReader;
#[doc = "Field `MFS` writer - Management Frame Sent"]
pub type MfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOMP` reader - Receive Complete"]
pub type RcompR = crate::BitReader;
#[doc = "Field `RCOMP` writer - Receive Complete"]
pub type RcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUBR` reader - RX Used Bit Read"]
pub type RxubrR = crate::BitReader;
#[doc = "Field `RXUBR` writer - RX Used Bit Read"]
pub type RxubrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUBR` reader - TX Used Bit Read"]
pub type TxubrR = crate::BitReader;
#[doc = "Field `TXUBR` writer - TX Used Bit Read"]
pub type TxubrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUR` reader - Transmit Underrun"]
pub type TurR = crate::BitReader;
#[doc = "Field `TUR` writer - Transmit Underrun"]
pub type TurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLEX` reader - Retry Limit Exceeded"]
pub type RlexR = crate::BitReader;
#[doc = "Field `RLEX` writer - Retry Limit Exceeded"]
pub type RlexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFC` reader - Transmit Frame Corruption Due to AHB Error"]
pub type TfcR = crate::BitReader;
#[doc = "Field `TFC` writer - Transmit Frame Corruption Due to AHB Error"]
pub type TfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCOMP` reader - Transmit Complete"]
pub type TcompR = crate::BitReader;
#[doc = "Field `TCOMP` writer - Transmit Complete"]
pub type TcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub type RovrR = crate::BitReader;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type RovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESP` reader - HRESP Not OK"]
pub type HrespR = crate::BitReader;
#[doc = "Field `HRESP` writer - HRESP Not OK"]
pub type HrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFNZ` reader - Pause Frame with Non-zero Pause Quantum Received"]
pub type PfnzR = crate::BitReader;
#[doc = "Field `PFNZ` writer - Pause Frame with Non-zero Pause Quantum Received"]
pub type PfnzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTZ` reader - Pause Time Zero"]
pub type PtzR = crate::BitReader;
#[doc = "Field `PTZ` writer - Pause Time Zero"]
pub type PtzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFTR` reader - Pause Frame Transmitted"]
pub type PftrR = crate::BitReader;
#[doc = "Field `PFTR` writer - Pause Frame Transmitted"]
pub type PftrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQFR` reader - PTP Delay Request Frame Received"]
pub type DrqfrR = crate::BitReader;
#[doc = "Field `DRQFR` writer - PTP Delay Request Frame Received"]
pub type DrqfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFR` reader - PTP Sync Frame Received"]
pub type SfrR = crate::BitReader;
#[doc = "Field `SFR` writer - PTP Sync Frame Received"]
pub type SfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQFT` reader - PTP Delay Request Frame Transmitted"]
pub type DrqftR = crate::BitReader;
#[doc = "Field `DRQFT` writer - PTP Delay Request Frame Transmitted"]
pub type DrqftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFT` reader - PTP Sync Frame Transmitted"]
pub type SftR = crate::BitReader;
#[doc = "Field `SFT` writer - PTP Sync Frame Transmitted"]
pub type SftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDRQFR` reader - PDelay Request Frame Received"]
pub type PdrqfrR = crate::BitReader;
#[doc = "Field `PDRQFR` writer - PDelay Request Frame Received"]
pub type PdrqfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDRSFR` reader - PDelay Response Frame Received"]
pub type PdrsfrR = crate::BitReader;
#[doc = "Field `PDRSFR` writer - PDelay Response Frame Received"]
pub type PdrsfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDRQFT` reader - PDelay Request Frame Transmitted"]
pub type PdrqftR = crate::BitReader;
#[doc = "Field `PDRQFT` writer - PDelay Request Frame Transmitted"]
pub type PdrqftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDRSFT` reader - PDelay Response Frame Transmitted"]
pub type PdrsftR = crate::BitReader;
#[doc = "Field `PDRSFT` writer - PDelay Response Frame Transmitted"]
pub type PdrsftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRI` reader - TSU Seconds Register Increment"]
pub type SriR = crate::BitReader;
#[doc = "Field `SRI` writer - TSU Seconds Register Increment"]
pub type SriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLPISBC` reader - Enable RX LPI Indication"]
pub type RxlpisbcR = crate::BitReader;
#[doc = "Field `RXLPISBC` writer - Enable RX LPI Indication"]
pub type RxlpisbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WOL` reader - Wake On LAN"]
pub type WolR = crate::BitReader;
#[doc = "Field `WOL` writer - Wake On LAN"]
pub type WolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUCMP` reader - Tsu timer comparison"]
pub type TsucmpR = crate::BitReader;
#[doc = "Field `TSUCMP` writer - Tsu timer comparison"]
pub type TsucmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Management Frame Sent"]
    #[inline(always)]
    pub fn mfs(&self) -> MfsR {
        MfsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&self) -> RcompR {
        RcompR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&self) -> RxubrR {
        RxubrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX Used Bit Read"]
    #[inline(always)]
    pub fn txubr(&self) -> TxubrR {
        TxubrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Underrun"]
    #[inline(always)]
    pub fn tur(&self) -> TurR {
        TurR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded"]
    #[inline(always)]
    pub fn rlex(&self) -> RlexR {
        RlexR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    pub fn tfc(&self) -> TfcR {
        TfcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TcompR {
        TcompR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> RovrR {
        RovrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HrespR {
        HrespR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pause Frame with Non-zero Pause Quantum Received"]
    #[inline(always)]
    pub fn pfnz(&self) -> PfnzR {
        PfnzR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    pub fn ptz(&self) -> PtzR {
        PtzR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pause Frame Transmitted"]
    #[inline(always)]
    pub fn pftr(&self) -> PftrR {
        PftrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - PTP Delay Request Frame Received"]
    #[inline(always)]
    pub fn drqfr(&self) -> DrqfrR {
        DrqfrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PTP Sync Frame Received"]
    #[inline(always)]
    pub fn sfr(&self) -> SfrR {
        SfrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PTP Delay Request Frame Transmitted"]
    #[inline(always)]
    pub fn drqft(&self) -> DrqftR {
        DrqftR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PTP Sync Frame Transmitted"]
    #[inline(always)]
    pub fn sft(&self) -> SftR {
        SftR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PDelay Request Frame Received"]
    #[inline(always)]
    pub fn pdrqfr(&self) -> PdrqfrR {
        PdrqfrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PDelay Response Frame Received"]
    #[inline(always)]
    pub fn pdrsfr(&self) -> PdrsfrR {
        PdrsfrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PDelay Request Frame Transmitted"]
    #[inline(always)]
    pub fn pdrqft(&self) -> PdrqftR {
        PdrqftR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PDelay Response Frame Transmitted"]
    #[inline(always)]
    pub fn pdrsft(&self) -> PdrsftR {
        PdrsftR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TSU Seconds Register Increment"]
    #[inline(always)]
    pub fn sri(&self) -> SriR {
        SriR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable RX LPI Indication"]
    #[inline(always)]
    pub fn rxlpisbc(&self) -> RxlpisbcR {
        RxlpisbcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wake On LAN"]
    #[inline(always)]
    pub fn wol(&self) -> WolR {
        WolR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Tsu timer comparison"]
    #[inline(always)]
    pub fn tsucmp(&self) -> TsucmpR {
        TsucmpR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Management Frame Sent"]
    #[inline(always)]
    #[must_use]
    pub fn mfs(&mut self) -> MfsW<IsrSpec> {
        MfsW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    #[must_use]
    pub fn rcomp(&mut self) -> RcompW<IsrSpec> {
        RcompW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn rxubr(&mut self) -> RxubrW<IsrSpec> {
        RxubrW::new(self, 2)
    }
    #[doc = "Bit 3 - TX Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn txubr(&mut self) -> TxubrW<IsrSpec> {
        TxubrW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn tur(&mut self) -> TurW<IsrSpec> {
        TurW::new(self, 4)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded"]
    #[inline(always)]
    #[must_use]
    pub fn rlex(&mut self) -> RlexW<IsrSpec> {
        RlexW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn tfc(&mut self) -> TfcW<IsrSpec> {
        TfcW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tcomp(&mut self) -> TcompW<IsrSpec> {
        TcompW::new(self, 7)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rovr(&mut self) -> RovrW<IsrSpec> {
        RovrW::new(self, 10)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    #[must_use]
    pub fn hresp(&mut self) -> HrespW<IsrSpec> {
        HrespW::new(self, 11)
    }
    #[doc = "Bit 12 - Pause Frame with Non-zero Pause Quantum Received"]
    #[inline(always)]
    #[must_use]
    pub fn pfnz(&mut self) -> PfnzW<IsrSpec> {
        PfnzW::new(self, 12)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    #[must_use]
    pub fn ptz(&mut self) -> PtzW<IsrSpec> {
        PtzW::new(self, 13)
    }
    #[doc = "Bit 14 - Pause Frame Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn pftr(&mut self) -> PftrW<IsrSpec> {
        PftrW::new(self, 14)
    }
    #[doc = "Bit 18 - PTP Delay Request Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn drqfr(&mut self) -> DrqfrW<IsrSpec> {
        DrqfrW::new(self, 18)
    }
    #[doc = "Bit 19 - PTP Sync Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn sfr(&mut self) -> SfrW<IsrSpec> {
        SfrW::new(self, 19)
    }
    #[doc = "Bit 20 - PTP Delay Request Frame Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn drqft(&mut self) -> DrqftW<IsrSpec> {
        DrqftW::new(self, 20)
    }
    #[doc = "Bit 21 - PTP Sync Frame Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn sft(&mut self) -> SftW<IsrSpec> {
        SftW::new(self, 21)
    }
    #[doc = "Bit 22 - PDelay Request Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn pdrqfr(&mut self) -> PdrqfrW<IsrSpec> {
        PdrqfrW::new(self, 22)
    }
    #[doc = "Bit 23 - PDelay Response Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn pdrsfr(&mut self) -> PdrsfrW<IsrSpec> {
        PdrsfrW::new(self, 23)
    }
    #[doc = "Bit 24 - PDelay Request Frame Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn pdrqft(&mut self) -> PdrqftW<IsrSpec> {
        PdrqftW::new(self, 24)
    }
    #[doc = "Bit 25 - PDelay Response Frame Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn pdrsft(&mut self) -> PdrsftW<IsrSpec> {
        PdrsftW::new(self, 25)
    }
    #[doc = "Bit 26 - TSU Seconds Register Increment"]
    #[inline(always)]
    #[must_use]
    pub fn sri(&mut self) -> SriW<IsrSpec> {
        SriW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable RX LPI Indication"]
    #[inline(always)]
    #[must_use]
    pub fn rxlpisbc(&mut self) -> RxlpisbcW<IsrSpec> {
        RxlpisbcW::new(self, 27)
    }
    #[doc = "Bit 28 - Wake On LAN"]
    #[inline(always)]
    #[must_use]
    pub fn wol(&mut self) -> WolW<IsrSpec> {
        WolW::new(self, 28)
    }
    #[doc = "Bit 29 - Tsu timer comparison"]
    #[inline(always)]
    #[must_use]
    pub fn tsucmp(&mut self) -> TsucmpW<IsrSpec> {
        TsucmpW::new(self, 29)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}
