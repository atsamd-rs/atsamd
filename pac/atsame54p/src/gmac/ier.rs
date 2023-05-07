#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MFS` writer - Management Frame Sent"]
pub type MFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `RCOMP` writer - Receive Complete"]
pub type RCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `RXUBR` writer - RX Used Bit Read"]
pub type RXUBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TXUBR` writer - TX Used Bit Read"]
pub type TXUBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TUR` writer - Transmit Underrun"]
pub type TUR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `RLEX` writer - Retry Limit Exceeded or Late Collision"]
pub type RLEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TFC` writer - Transmit Frame Corruption Due to AHB Error"]
pub type TFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TCOMP` writer - Transmit Complete"]
pub type TCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type ROVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `HRESP` writer - HRESP Not OK"]
pub type HRESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `PFNZ` writer - Pause Frame with Non-zero Pause Quantum Received"]
pub type PFNZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `PTZ` writer - Pause Time Zero"]
pub type PTZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `PFTR` writer - Pause Frame Transmitted"]
pub type PFTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EXINT` writer - External Interrupt"]
pub type EXINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `DRQFR` writer - PTP Delay Request Frame Received"]
pub type DRQFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `SFR` writer - PTP Sync Frame Received"]
pub type SFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `DRQFT` writer - PTP Delay Request Frame Transmitted"]
pub type DRQFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `SFT` writer - PTP Sync Frame Transmitted"]
pub type SFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `PDRQFR` writer - PDelay Request Frame Received"]
pub type PDRQFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `PDRSFR` writer - PDelay Response Frame Received"]
pub type PDRSFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `PDRQFT` writer - PDelay Request Frame Transmitted"]
pub type PDRQFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `PDRSFT` writer - PDelay Response Frame Transmitted"]
pub type PDRSFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `SRI` writer - TSU Seconds Register Increment"]
pub type SRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `WOL` writer - Wake On LAN"]
pub type WOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TSUCMP` writer - Tsu timer comparison"]
pub type TSUCMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Management Frame Sent"]
    #[inline(always)]
    #[must_use]
    pub fn mfs(&mut self) -> MFS_W<0> {
        MFS_W::new(self)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    #[must_use]
    pub fn rcomp(&mut self) -> RCOMP_W<1> {
        RCOMP_W::new(self)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn rxubr(&mut self) -> RXUBR_W<2> {
        RXUBR_W::new(self)
    }
    #[doc = "Bit 3 - TX Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn txubr(&mut self) -> TXUBR_W<3> {
        TXUBR_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn tur(&mut self) -> TUR_W<4> {
        TUR_W::new(self)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded or Late Collision"]
    #[inline(always)]
    #[must_use]
    pub fn rlex(&mut self) -> RLEX_W<5> {
        RLEX_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn tfc(&mut self) -> TFC_W<6> {
        TFC_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tcomp(&mut self) -> TCOMP_W<7> {
        TCOMP_W::new(self)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rovr(&mut self) -> ROVR_W<10> {
        ROVR_W::new(self)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    #[must_use]
    pub fn hresp(&mut self) -> HRESP_W<11> {
        HRESP_W::new(self)
    }
    #[doc = "Bit 12 - Pause Frame with Non-zero Pause Quantum Received"]
    #[inline(always)]
    #[must_use]
    pub fn pfnz(&mut self) -> PFNZ_W<12> {
        PFNZ_W::new(self)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    #[must_use]
    pub fn ptz(&mut self) -> PTZ_W<13> {
        PTZ_W::new(self)
    }
    #[doc = "Bit 14 - Pause Frame Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn pftr(&mut self) -> PFTR_W<14> {
        PFTR_W::new(self)
    }
    #[doc = "Bit 15 - External Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn exint(&mut self) -> EXINT_W<15> {
        EXINT_W::new(self)
    }
    #[doc = "Bit 18 - PTP Delay Request Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn drqfr(&mut self) -> DRQFR_W<18> {
        DRQFR_W::new(self)
    }
    #[doc = "Bit 19 - PTP Sync Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn sfr(&mut self) -> SFR_W<19> {
        SFR_W::new(self)
    }
    #[doc = "Bit 20 - PTP Delay Request Frame Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn drqft(&mut self) -> DRQFT_W<20> {
        DRQFT_W::new(self)
    }
    #[doc = "Bit 21 - PTP Sync Frame Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn sft(&mut self) -> SFT_W<21> {
        SFT_W::new(self)
    }
    #[doc = "Bit 22 - PDelay Request Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn pdrqfr(&mut self) -> PDRQFR_W<22> {
        PDRQFR_W::new(self)
    }
    #[doc = "Bit 23 - PDelay Response Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn pdrsfr(&mut self) -> PDRSFR_W<23> {
        PDRSFR_W::new(self)
    }
    #[doc = "Bit 24 - PDelay Request Frame Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn pdrqft(&mut self) -> PDRQFT_W<24> {
        PDRQFT_W::new(self)
    }
    #[doc = "Bit 25 - PDelay Response Frame Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn pdrsft(&mut self) -> PDRSFT_W<25> {
        PDRSFT_W::new(self)
    }
    #[doc = "Bit 26 - TSU Seconds Register Increment"]
    #[inline(always)]
    #[must_use]
    pub fn sri(&mut self) -> SRI_W<26> {
        SRI_W::new(self)
    }
    #[doc = "Bit 28 - Wake On LAN"]
    #[inline(always)]
    #[must_use]
    pub fn wol(&mut self) -> WOL_W<28> {
        WOL_W::new(self)
    }
    #[doc = "Bit 29 - Tsu timer comparison"]
    #[inline(always)]
    #[must_use]
    pub fn tsucmp(&mut self) -> TSUCMP_W<29> {
        TSUCMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
