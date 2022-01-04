#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MFS` reader - Management Frame Sent"]
pub struct MFS_R(crate::FieldReader<bool, bool>);
impl MFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCOMP` reader - Receive Complete"]
pub struct RCOMP_R(crate::FieldReader<bool, bool>);
impl RCOMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCOMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUBR` reader - RX Used Bit Read"]
pub struct RXUBR_R(crate::FieldReader<bool, bool>);
impl RXUBR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXUBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUBR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUBR` reader - TX Used Bit Read"]
pub struct TXUBR_R(crate::FieldReader<bool, bool>);
impl TXUBR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXUBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUBR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TUR` reader - Transmit Underrun"]
pub struct TUR_R(crate::FieldReader<bool, bool>);
impl TUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TUR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLEX` reader - Retry Limit Exceeded"]
pub struct RLEX_R(crate::FieldReader<bool, bool>);
impl RLEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RLEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLEX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFC` reader - Transmit Frame Corruption Due to AHB Error"]
pub struct TFC_R(crate::FieldReader<bool, bool>);
impl TFC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCOMP` reader - Transmit Complete"]
pub struct TCOMP_R(crate::FieldReader<bool, bool>);
impl TCOMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCOMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub struct ROVR_R(crate::FieldReader<bool, bool>);
impl ROVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRESP` reader - HRESP Not OK"]
pub struct HRESP_R(crate::FieldReader<bool, bool>);
impl HRESP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRESP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRESP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PFNZ` reader - Pause Frame with Non-zero Pause Quantum Received"]
pub struct PFNZ_R(crate::FieldReader<bool, bool>);
impl PFNZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PFNZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PFNZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTZ` reader - Pause Time Zero"]
pub struct PTZ_R(crate::FieldReader<bool, bool>);
impl PTZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PTZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PFTR` reader - Pause Frame Transmitted"]
pub struct PFTR_R(crate::FieldReader<bool, bool>);
impl PFTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PFTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PFTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXINT` reader - External Interrupt"]
pub struct EXINT_R(crate::FieldReader<bool, bool>);
impl EXINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRQFR` reader - PTP Delay Request Frame Received"]
pub struct DRQFR_R(crate::FieldReader<bool, bool>);
impl DRQFR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRQFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRQFR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFR` reader - PTP Sync Frame Received"]
pub struct SFR_R(crate::FieldReader<bool, bool>);
impl SFR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRQFT` reader - PTP Delay Request Frame Transmitted"]
pub struct DRQFT_R(crate::FieldReader<bool, bool>);
impl DRQFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRQFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRQFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFT` reader - PTP Sync Frame Transmitted"]
pub struct SFT_R(crate::FieldReader<bool, bool>);
impl SFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDRQFR` reader - PDelay Request Frame Received"]
pub struct PDRQFR_R(crate::FieldReader<bool, bool>);
impl PDRQFR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDRQFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDRQFR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDRSFR` reader - PDelay Response Frame Received"]
pub struct PDRSFR_R(crate::FieldReader<bool, bool>);
impl PDRSFR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDRSFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDRSFR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDRQFT` reader - PDelay Request Frame Transmitted"]
pub struct PDRQFT_R(crate::FieldReader<bool, bool>);
impl PDRQFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDRQFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDRQFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDRSFT` reader - PDelay Response Frame Transmitted"]
pub struct PDRSFT_R(crate::FieldReader<bool, bool>);
impl PDRSFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDRSFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDRSFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRI` reader - TSU Seconds Register Increment"]
pub struct SRI_R(crate::FieldReader<bool, bool>);
impl SRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WOL` reader - Wake On Lan"]
pub struct WOL_R(crate::FieldReader<bool, bool>);
impl WOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSUCMP` reader - Tsu timer comparison"]
pub struct TSUCMP_R(crate::FieldReader<bool, bool>);
impl TSUCMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSUCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSUCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0x3fff_ffff"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3fff_ffff
    }
}
