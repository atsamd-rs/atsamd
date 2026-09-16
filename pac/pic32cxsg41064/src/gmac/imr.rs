#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `MFS` reader - Management Frame Sent"]
pub type MfsR = crate::BitReader;
#[doc = "Field `RCOMP` reader - Receive Complete"]
pub type RcompR = crate::BitReader;
#[doc = "Field `RXUBR` reader - RX Used Bit Read"]
pub type RxubrR = crate::BitReader;
#[doc = "Field `TXUBR` reader - TX Used Bit Read"]
pub type TxubrR = crate::BitReader;
#[doc = "Field `TUR` reader - Transmit Underrun"]
pub type TurR = crate::BitReader;
#[doc = "Field `RLEX` reader - Retry Limit Exceeded"]
pub type RlexR = crate::BitReader;
#[doc = "Field `TFC` reader - Transmit Frame Corruption Due to AHB Error"]
pub type TfcR = crate::BitReader;
#[doc = "Field `TCOMP` reader - Transmit Complete"]
pub type TcompR = crate::BitReader;
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub type RovrR = crate::BitReader;
#[doc = "Field `HRESP` reader - HRESP Not OK"]
pub type HrespR = crate::BitReader;
#[doc = "Field `PFNZ` reader - Pause Frame with Non-zero Pause Quantum Received"]
pub type PfnzR = crate::BitReader;
#[doc = "Field `PTZ` reader - Pause Time Zero"]
pub type PtzR = crate::BitReader;
#[doc = "Field `PFTR` reader - Pause Frame Transmitted"]
pub type PftrR = crate::BitReader;
#[doc = "Field `EXINT` reader - External Interrupt"]
pub type ExintR = crate::BitReader;
#[doc = "Field `DRQFR` reader - PTP Delay Request Frame Received"]
pub type DrqfrR = crate::BitReader;
#[doc = "Field `SFR` reader - PTP Sync Frame Received"]
pub type SfrR = crate::BitReader;
#[doc = "Field `DRQFT` reader - PTP Delay Request Frame Transmitted"]
pub type DrqftR = crate::BitReader;
#[doc = "Field `SFT` reader - PTP Sync Frame Transmitted"]
pub type SftR = crate::BitReader;
#[doc = "Field `PDRQFR` reader - PDelay Request Frame Received"]
pub type PdrqfrR = crate::BitReader;
#[doc = "Field `PDRSFR` reader - PDelay Response Frame Received"]
pub type PdrsfrR = crate::BitReader;
#[doc = "Field `PDRQFT` reader - PDelay Request Frame Transmitted"]
pub type PdrqftR = crate::BitReader;
#[doc = "Field `PDRSFT` reader - PDelay Response Frame Transmitted"]
pub type PdrsftR = crate::BitReader;
#[doc = "Field `SRI` reader - TSU Seconds Register Increment"]
pub type SriR = crate::BitReader;
#[doc = "Field `RXLPISBC` reader - Enable RX LPI Indication"]
pub type RxlpisbcR = crate::BitReader;
#[doc = "Field `WOL` reader - Wake On Lan"]
pub type WolR = crate::BitReader;
#[doc = "Field `TSUCMP` reader - Tsu timer comparison"]
pub type TsucmpR = crate::BitReader;
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
    #[doc = "Bit 15 - External Interrupt"]
    #[inline(always)]
    pub fn exint(&self) -> ExintR {
        ExintR::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 28 - Wake On Lan"]
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
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0x3fff_ffff"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0x3fff_ffff;
}
