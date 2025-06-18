#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `MFS` writer - Management Frame Sent"]
pub type MfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOMP` writer - Receive Complete"]
pub type RcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUBR` writer - RX Used Bit Read"]
pub type RxubrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUBR` writer - TX Used Bit Read"]
pub type TxubrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUR` writer - Transmit Underrun"]
pub type TurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLEX` writer - Retry Limit Exceeded or Late Collision"]
pub type RlexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFC` writer - Transmit Frame Corruption Due to AHB Error"]
pub type TfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCOMP` writer - Transmit Complete"]
pub type TcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type RovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESP` writer - HRESP Not OK"]
pub type HrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFNZ` writer - Pause Frame with Non-zero Pause Quantum Received"]
pub type PfnzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTZ` writer - Pause Time Zero"]
pub type PtzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFTR` writer - Pause Frame Transmitted"]
pub type PftrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXINT` writer - External Interrupt"]
pub type ExintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQFR` writer - PTP Delay Request Frame Received"]
pub type DrqfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFR` writer - PTP Sync Frame Received"]
pub type SfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQFT` writer - PTP Delay Request Frame Transmitted"]
pub type DrqftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFT` writer - PTP Sync Frame Transmitted"]
pub type SftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDRQFR` writer - PDelay Request Frame Received"]
pub type PdrqfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDRSFR` writer - PDelay Response Frame Received"]
pub type PdrsfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDRQFT` writer - PDelay Request Frame Transmitted"]
pub type PdrqftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDRSFT` writer - PDelay Response Frame Transmitted"]
pub type PdrsftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRI` writer - TSU Seconds Register Increment"]
pub type SriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLPISBC` writer - Enable RX LPI Indication"]
pub type RxlpisbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WOL` writer - Wake On LAN"]
pub type WolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUCMP` writer - Tsu timer comparison"]
pub type TsucmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Management Frame Sent"]
    #[inline(always)]
    pub fn mfs(&mut self) -> MfsW<IerSpec> {
        MfsW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&mut self) -> RcompW<IerSpec> {
        RcompW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&mut self) -> RxubrW<IerSpec> {
        RxubrW::new(self, 2)
    }
    #[doc = "Bit 3 - TX Used Bit Read"]
    #[inline(always)]
    pub fn txubr(&mut self) -> TxubrW<IerSpec> {
        TxubrW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit Underrun"]
    #[inline(always)]
    pub fn tur(&mut self) -> TurW<IerSpec> {
        TurW::new(self, 4)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded or Late Collision"]
    #[inline(always)]
    pub fn rlex(&mut self) -> RlexW<IerSpec> {
        RlexW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    pub fn tfc(&mut self) -> TfcW<IerSpec> {
        TfcW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&mut self) -> TcompW<IerSpec> {
        TcompW::new(self, 7)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&mut self) -> RovrW<IerSpec> {
        RovrW::new(self, 10)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&mut self) -> HrespW<IerSpec> {
        HrespW::new(self, 11)
    }
    #[doc = "Bit 12 - Pause Frame with Non-zero Pause Quantum Received"]
    #[inline(always)]
    pub fn pfnz(&mut self) -> PfnzW<IerSpec> {
        PfnzW::new(self, 12)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    pub fn ptz(&mut self) -> PtzW<IerSpec> {
        PtzW::new(self, 13)
    }
    #[doc = "Bit 14 - Pause Frame Transmitted"]
    #[inline(always)]
    pub fn pftr(&mut self) -> PftrW<IerSpec> {
        PftrW::new(self, 14)
    }
    #[doc = "Bit 15 - External Interrupt"]
    #[inline(always)]
    pub fn exint(&mut self) -> ExintW<IerSpec> {
        ExintW::new(self, 15)
    }
    #[doc = "Bit 18 - PTP Delay Request Frame Received"]
    #[inline(always)]
    pub fn drqfr(&mut self) -> DrqfrW<IerSpec> {
        DrqfrW::new(self, 18)
    }
    #[doc = "Bit 19 - PTP Sync Frame Received"]
    #[inline(always)]
    pub fn sfr(&mut self) -> SfrW<IerSpec> {
        SfrW::new(self, 19)
    }
    #[doc = "Bit 20 - PTP Delay Request Frame Transmitted"]
    #[inline(always)]
    pub fn drqft(&mut self) -> DrqftW<IerSpec> {
        DrqftW::new(self, 20)
    }
    #[doc = "Bit 21 - PTP Sync Frame Transmitted"]
    #[inline(always)]
    pub fn sft(&mut self) -> SftW<IerSpec> {
        SftW::new(self, 21)
    }
    #[doc = "Bit 22 - PDelay Request Frame Received"]
    #[inline(always)]
    pub fn pdrqfr(&mut self) -> PdrqfrW<IerSpec> {
        PdrqfrW::new(self, 22)
    }
    #[doc = "Bit 23 - PDelay Response Frame Received"]
    #[inline(always)]
    pub fn pdrsfr(&mut self) -> PdrsfrW<IerSpec> {
        PdrsfrW::new(self, 23)
    }
    #[doc = "Bit 24 - PDelay Request Frame Transmitted"]
    #[inline(always)]
    pub fn pdrqft(&mut self) -> PdrqftW<IerSpec> {
        PdrqftW::new(self, 24)
    }
    #[doc = "Bit 25 - PDelay Response Frame Transmitted"]
    #[inline(always)]
    pub fn pdrsft(&mut self) -> PdrsftW<IerSpec> {
        PdrsftW::new(self, 25)
    }
    #[doc = "Bit 26 - TSU Seconds Register Increment"]
    #[inline(always)]
    pub fn sri(&mut self) -> SriW<IerSpec> {
        SriW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable RX LPI Indication"]
    #[inline(always)]
    pub fn rxlpisbc(&mut self) -> RxlpisbcW<IerSpec> {
        RxlpisbcW::new(self, 27)
    }
    #[doc = "Bit 28 - Wake On LAN"]
    #[inline(always)]
    pub fn wol(&mut self) -> WolW<IerSpec> {
        WolW::new(self, 28)
    }
    #[doc = "Bit 29 - Tsu timer comparison"]
    #[inline(always)]
    pub fn tsucmp(&mut self) -> TsucmpW<IerSpec> {
        TsucmpW::new(self, 29)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
