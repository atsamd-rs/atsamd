#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `MFS`"]
pub type MFS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCOMP`"]
pub type RCOMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXUBR`"]
pub type RXUBR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXUBR`"]
pub type TXUBR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TUR`"]
pub type TUR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RLEX`"]
pub type RLEX_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFC`"]
pub type TFC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCOMP`"]
pub type TCOMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `ROVR`"]
pub type ROVR_R = crate::R<bool, bool>;
#[doc = "Reader of field `HRESP`"]
pub type HRESP_R = crate::R<bool, bool>;
#[doc = "Reader of field `PFNZ`"]
pub type PFNZ_R = crate::R<bool, bool>;
#[doc = "Reader of field `PTZ`"]
pub type PTZ_R = crate::R<bool, bool>;
#[doc = "Reader of field `PFTR`"]
pub type PFTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXINT`"]
pub type EXINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRQFR`"]
pub type DRQFR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SFR`"]
pub type SFR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRQFT`"]
pub type DRQFT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SFT`"]
pub type SFT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PDRQFR`"]
pub type PDRQFR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PDRSFR`"]
pub type PDRSFR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PDRQFT`"]
pub type PDRQFT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PDRSFT`"]
pub type PDRSFT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRI`"]
pub type SRI_R = crate::R<bool, bool>;
#[doc = "Reader of field `WOL`"]
pub type WOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSUCMP`"]
pub type TSUCMP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Management Frame Sent"]
    #[inline(always)]
    pub fn mfs(&self) -> MFS_R {
        MFS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&self) -> RCOMP_R {
        RCOMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&self) -> RXUBR_R {
        RXUBR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX Used Bit Read"]
    #[inline(always)]
    pub fn txubr(&self) -> TXUBR_R {
        TXUBR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit Underrun"]
    #[inline(always)]
    pub fn tur(&self) -> TUR_R {
        TUR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded"]
    #[inline(always)]
    pub fn rlex(&self) -> RLEX_R {
        RLEX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    pub fn tfc(&self) -> TFC_R {
        TFC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TCOMP_R {
        TCOMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HRESP_R {
        HRESP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pause Frame with Non-zero Pause Quantum Received"]
    #[inline(always)]
    pub fn pfnz(&self) -> PFNZ_R {
        PFNZ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    pub fn ptz(&self) -> PTZ_R {
        PTZ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pause Frame Transmitted"]
    #[inline(always)]
    pub fn pftr(&self) -> PFTR_R {
        PFTR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - External Interrupt"]
    #[inline(always)]
    pub fn exint(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PTP Delay Request Frame Received"]
    #[inline(always)]
    pub fn drqfr(&self) -> DRQFR_R {
        DRQFR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PTP Sync Frame Received"]
    #[inline(always)]
    pub fn sfr(&self) -> SFR_R {
        SFR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PTP Delay Request Frame Transmitted"]
    #[inline(always)]
    pub fn drqft(&self) -> DRQFT_R {
        DRQFT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PTP Sync Frame Transmitted"]
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PDelay Request Frame Received"]
    #[inline(always)]
    pub fn pdrqfr(&self) -> PDRQFR_R {
        PDRQFR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PDelay Response Frame Received"]
    #[inline(always)]
    pub fn pdrsfr(&self) -> PDRSFR_R {
        PDRSFR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PDelay Request Frame Transmitted"]
    #[inline(always)]
    pub fn pdrqft(&self) -> PDRQFT_R {
        PDRQFT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - PDelay Response Frame Transmitted"]
    #[inline(always)]
    pub fn pdrsft(&self) -> PDRSFT_R {
        PDRSFT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TSU Seconds Register Increment"]
    #[inline(always)]
    pub fn sri(&self) -> SRI_R {
        SRI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Wake On Lan"]
    #[inline(always)]
    pub fn wol(&self) -> WOL_R {
        WOL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Tsu timer comparison"]
    #[inline(always)]
    pub fn tsucmp(&self) -> TSUCMP_R {
        TSUCMP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
