#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
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
#[doc = "Field `WOL` writer - Wake On LAN"]
pub type WolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUCMP` writer - Tsu timer comparison"]
pub type TsucmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Management Frame Sent"]
    #[inline(always)]
    #[must_use]
    pub fn mfs(&mut self) -> MfsW<IdrSpec> {
        MfsW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    #[must_use]
    pub fn rcomp(&mut self) -> RcompW<IdrSpec> {
        RcompW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn rxubr(&mut self) -> RxubrW<IdrSpec> {
        RxubrW::new(self, 2)
    }
    #[doc = "Bit 3 - TX Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn txubr(&mut self) -> TxubrW<IdrSpec> {
        TxubrW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn tur(&mut self) -> TurW<IdrSpec> {
        TurW::new(self, 4)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded or Late Collision"]
    #[inline(always)]
    #[must_use]
    pub fn rlex(&mut self) -> RlexW<IdrSpec> {
        RlexW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn tfc(&mut self) -> TfcW<IdrSpec> {
        TfcW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tcomp(&mut self) -> TcompW<IdrSpec> {
        TcompW::new(self, 7)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rovr(&mut self) -> RovrW<IdrSpec> {
        RovrW::new(self, 10)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    #[must_use]
    pub fn hresp(&mut self) -> HrespW<IdrSpec> {
        HrespW::new(self, 11)
    }
    #[doc = "Bit 12 - Pause Frame with Non-zero Pause Quantum Received"]
    #[inline(always)]
    #[must_use]
    pub fn pfnz(&mut self) -> PfnzW<IdrSpec> {
        PfnzW::new(self, 12)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    #[must_use]
    pub fn ptz(&mut self) -> PtzW<IdrSpec> {
        PtzW::new(self, 13)
    }
    #[doc = "Bit 14 - Pause Frame Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn pftr(&mut self) -> PftrW<IdrSpec> {
        PftrW::new(self, 14)
    }
    #[doc = "Bit 15 - External Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn exint(&mut self) -> ExintW<IdrSpec> {
        ExintW::new(self, 15)
    }
    #[doc = "Bit 18 - PTP Delay Request Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn drqfr(&mut self) -> DrqfrW<IdrSpec> {
        DrqfrW::new(self, 18)
    }
    #[doc = "Bit 19 - PTP Sync Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn sfr(&mut self) -> SfrW<IdrSpec> {
        SfrW::new(self, 19)
    }
    #[doc = "Bit 20 - PTP Delay Request Frame Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn drqft(&mut self) -> DrqftW<IdrSpec> {
        DrqftW::new(self, 20)
    }
    #[doc = "Bit 21 - PTP Sync Frame Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn sft(&mut self) -> SftW<IdrSpec> {
        SftW::new(self, 21)
    }
    #[doc = "Bit 22 - PDelay Request Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn pdrqfr(&mut self) -> PdrqfrW<IdrSpec> {
        PdrqfrW::new(self, 22)
    }
    #[doc = "Bit 23 - PDelay Response Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn pdrsfr(&mut self) -> PdrsfrW<IdrSpec> {
        PdrsfrW::new(self, 23)
    }
    #[doc = "Bit 24 - PDelay Request Frame Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn pdrqft(&mut self) -> PdrqftW<IdrSpec> {
        PdrqftW::new(self, 24)
    }
    #[doc = "Bit 25 - PDelay Response Frame Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn pdrsft(&mut self) -> PdrsftW<IdrSpec> {
        PdrsftW::new(self, 25)
    }
    #[doc = "Bit 26 - TSU Seconds Register Increment"]
    #[inline(always)]
    #[must_use]
    pub fn sri(&mut self) -> SriW<IdrSpec> {
        SriW::new(self, 26)
    }
    #[doc = "Bit 28 - Wake On LAN"]
    #[inline(always)]
    #[must_use]
    pub fn wol(&mut self) -> WolW<IdrSpec> {
        WolW::new(self, 28)
    }
    #[doc = "Bit 29 - Tsu timer comparison"]
    #[inline(always)]
    #[must_use]
    pub fn tsucmp(&mut self) -> TsucmpW<IdrSpec> {
        TsucmpW::new(self, 29)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {
    const RESET_VALUE: u32 = 0;
}
