#[doc = "Register `IR` reader"]
pub type R = crate::R<IR_SPEC>;
#[doc = "Register `IR` writer"]
pub type W = crate::W<IR_SPEC>;
#[doc = "Field `RF0N` reader - Rx FIFO 0 New Message"]
pub type RF0N_R = crate::BitReader;
#[doc = "Field `RF0N` writer - Rx FIFO 0 New Message"]
pub type RF0N_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF0W` reader - Rx FIFO 0 Watermark Reached"]
pub type RF0W_R = crate::BitReader;
#[doc = "Field `RF0W` writer - Rx FIFO 0 Watermark Reached"]
pub type RF0W_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF0F` reader - Rx FIFO 0 Full"]
pub type RF0F_R = crate::BitReader;
#[doc = "Field `RF0F` writer - Rx FIFO 0 Full"]
pub type RF0F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF0L` reader - Rx FIFO 0 Message Lost"]
pub type RF0L_R = crate::BitReader;
#[doc = "Field `RF0L` writer - Rx FIFO 0 Message Lost"]
pub type RF0L_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF1N` reader - Rx FIFO 1 New Message"]
pub type RF1N_R = crate::BitReader;
#[doc = "Field `RF1N` writer - Rx FIFO 1 New Message"]
pub type RF1N_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF1W` reader - Rx FIFO 1 Watermark Reached"]
pub type RF1W_R = crate::BitReader;
#[doc = "Field `RF1W` writer - Rx FIFO 1 Watermark Reached"]
pub type RF1W_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF1F` reader - Rx FIFO 1 FIFO Full"]
pub type RF1F_R = crate::BitReader;
#[doc = "Field `RF1F` writer - Rx FIFO 1 FIFO Full"]
pub type RF1F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF1L` reader - Rx FIFO 1 Message Lost"]
pub type RF1L_R = crate::BitReader;
#[doc = "Field `RF1L` writer - Rx FIFO 1 Message Lost"]
pub type RF1L_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPM` reader - High Priority Message"]
pub type HPM_R = crate::BitReader;
#[doc = "Field `HPM` writer - High Priority Message"]
pub type HPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC` reader - Timestamp Completed"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TC` writer - Timestamp Completed"]
pub type TC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCF` reader - Transmission Cancellation Finished"]
pub type TCF_R = crate::BitReader;
#[doc = "Field `TCF` writer - Transmission Cancellation Finished"]
pub type TCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFE` reader - Tx FIFO Empty"]
pub type TFE_R = crate::BitReader;
#[doc = "Field `TFE` writer - Tx FIFO Empty"]
pub type TFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEFN` reader - Tx Event FIFO New Entry"]
pub type TEFN_R = crate::BitReader;
#[doc = "Field `TEFN` writer - Tx Event FIFO New Entry"]
pub type TEFN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEFW` reader - Tx Event FIFO Watermark Reached"]
pub type TEFW_R = crate::BitReader;
#[doc = "Field `TEFW` writer - Tx Event FIFO Watermark Reached"]
pub type TEFW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEFF` reader - Tx Event FIFO Full"]
pub type TEFF_R = crate::BitReader;
#[doc = "Field `TEFF` writer - Tx Event FIFO Full"]
pub type TEFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEFL` reader - Tx Event FIFO Element Lost"]
pub type TEFL_R = crate::BitReader;
#[doc = "Field `TEFL` writer - Tx Event FIFO Element Lost"]
pub type TEFL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSW` reader - Timestamp Wraparound"]
pub type TSW_R = crate::BitReader;
#[doc = "Field `TSW` writer - Timestamp Wraparound"]
pub type TSW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MRAF` reader - Message RAM Access Failure"]
pub type MRAF_R = crate::BitReader;
#[doc = "Field `MRAF` writer - Message RAM Access Failure"]
pub type MRAF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TOO` reader - Timeout Occurred"]
pub type TOO_R = crate::BitReader;
#[doc = "Field `TOO` writer - Timeout Occurred"]
pub type TOO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRX` reader - Message stored to Dedicated Rx Buffer"]
pub type DRX_R = crate::BitReader;
#[doc = "Field `DRX` writer - Message stored to Dedicated Rx Buffer"]
pub type DRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BEC` reader - Bit Error Corrected"]
pub type BEC_R = crate::BitReader;
#[doc = "Field `BEC` writer - Bit Error Corrected"]
pub type BEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BEU` reader - Bit Error Uncorrected"]
pub type BEU_R = crate::BitReader;
#[doc = "Field `BEU` writer - Bit Error Uncorrected"]
pub type BEU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ELO` reader - Error Logging Overflow"]
pub type ELO_R = crate::BitReader;
#[doc = "Field `ELO` writer - Error Logging Overflow"]
pub type ELO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP` reader - Error Passive"]
pub type EP_R = crate::BitReader;
#[doc = "Field `EP` writer - Error Passive"]
pub type EP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EW` reader - Warning Status"]
pub type EW_R = crate::BitReader;
#[doc = "Field `EW` writer - Warning Status"]
pub type EW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BO` reader - Bus_Off Status"]
pub type BO_R = crate::BitReader;
#[doc = "Field `BO` writer - Bus_Off Status"]
pub type BO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDI` reader - Watchdog Interrupt"]
pub type WDI_R = crate::BitReader;
#[doc = "Field `WDI` writer - Watchdog Interrupt"]
pub type WDI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEA` reader - Protocol Error in Arbitration Phase"]
pub type PEA_R = crate::BitReader;
#[doc = "Field `PEA` writer - Protocol Error in Arbitration Phase"]
pub type PEA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PED` reader - Protocol Error in Data Phase"]
pub type PED_R = crate::BitReader;
#[doc = "Field `PED` writer - Protocol Error in Data Phase"]
pub type PED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ARA` reader - Access to Reserved Address"]
pub type ARA_R = crate::BitReader;
#[doc = "Field `ARA` writer - Access to Reserved Address"]
pub type ARA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message"]
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached"]
    #[inline(always)]
    pub fn rf0w(&self) -> RF0W_R {
        RF0W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message"]
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    pub fn rf1w(&self) -> RF1W_R {
        RF1W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 FIFO Full"]
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message"]
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Completed"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry"]
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached"]
    #[inline(always)]
    pub fn tefw(&self) -> TEFW_R {
        TEFW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full"]
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound"]
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure"]
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred"]
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer"]
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit Error Corrected"]
    #[inline(always)]
    pub fn bec(&self) -> BEC_R {
        BEC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected"]
    #[inline(always)]
    pub fn beu(&self) -> BEU_R {
        BEU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow"]
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase"]
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase"]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address"]
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 New Message"]
    #[inline(always)]
    #[must_use]
    pub fn rf0n(&mut self) -> RF0N_W<IR_SPEC, 0> {
        RF0N_W::new(self)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached"]
    #[inline(always)]
    #[must_use]
    pub fn rf0w(&mut self) -> RF0W_W<IR_SPEC, 1> {
        RF0W_W::new(self)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full"]
    #[inline(always)]
    #[must_use]
    pub fn rf0f(&mut self) -> RF0F_W<IR_SPEC, 2> {
        RF0F_W::new(self)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> RF0L_W<IR_SPEC, 3> {
        RF0L_W::new(self)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message"]
    #[inline(always)]
    #[must_use]
    pub fn rf1n(&mut self) -> RF1N_W<IR_SPEC, 4> {
        RF1N_W::new(self)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    #[must_use]
    pub fn rf1w(&mut self) -> RF1W_W<IR_SPEC, 5> {
        RF1W_W::new(self)
    }
    #[doc = "Bit 6 - Rx FIFO 1 FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn rf1f(&mut self) -> RF1F_W<IR_SPEC, 6> {
        RF1F_W::new(self)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf1l(&mut self) -> RF1L_W<IR_SPEC, 7> {
        RF1L_W::new(self)
    }
    #[doc = "Bit 8 - High Priority Message"]
    #[inline(always)]
    #[must_use]
    pub fn hpm(&mut self) -> HPM_W<IR_SPEC, 8> {
        HPM_W::new(self)
    }
    #[doc = "Bit 9 - Timestamp Completed"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<IR_SPEC, 9> {
        TC_W::new(self)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<IR_SPEC, 10> {
        TCF_W::new(self)
    }
    #[doc = "Bit 11 - Tx FIFO Empty"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<IR_SPEC, 11> {
        TFE_W::new(self)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry"]
    #[inline(always)]
    #[must_use]
    pub fn tefn(&mut self) -> TEFN_W<IR_SPEC, 12> {
        TEFN_W::new(self)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached"]
    #[inline(always)]
    #[must_use]
    pub fn tefw(&mut self) -> TEFW_W<IR_SPEC, 13> {
        TEFW_W::new(self)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn teff(&mut self) -> TEFF_W<IR_SPEC, 14> {
        TEFF_W::new(self)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost"]
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TEFL_W<IR_SPEC, 15> {
        TEFL_W::new(self)
    }
    #[doc = "Bit 16 - Timestamp Wraparound"]
    #[inline(always)]
    #[must_use]
    pub fn tsw(&mut self) -> TSW_W<IR_SPEC, 16> {
        TSW_W::new(self)
    }
    #[doc = "Bit 17 - Message RAM Access Failure"]
    #[inline(always)]
    #[must_use]
    pub fn mraf(&mut self) -> MRAF_W<IR_SPEC, 17> {
        MRAF_W::new(self)
    }
    #[doc = "Bit 18 - Timeout Occurred"]
    #[inline(always)]
    #[must_use]
    pub fn too(&mut self) -> TOO_W<IR_SPEC, 18> {
        TOO_W::new(self)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn drx(&mut self) -> DRX_W<IR_SPEC, 19> {
        DRX_W::new(self)
    }
    #[doc = "Bit 20 - Bit Error Corrected"]
    #[inline(always)]
    #[must_use]
    pub fn bec(&mut self) -> BEC_W<IR_SPEC, 20> {
        BEC_W::new(self)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected"]
    #[inline(always)]
    #[must_use]
    pub fn beu(&mut self) -> BEU_W<IR_SPEC, 21> {
        BEU_W::new(self)
    }
    #[doc = "Bit 22 - Error Logging Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn elo(&mut self) -> ELO_W<IR_SPEC, 22> {
        ELO_W::new(self)
    }
    #[doc = "Bit 23 - Error Passive"]
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EP_W<IR_SPEC, 23> {
        EP_W::new(self)
    }
    #[doc = "Bit 24 - Warning Status"]
    #[inline(always)]
    #[must_use]
    pub fn ew(&mut self) -> EW_W<IR_SPEC, 24> {
        EW_W::new(self)
    }
    #[doc = "Bit 25 - Bus_Off Status"]
    #[inline(always)]
    #[must_use]
    pub fn bo(&mut self) -> BO_W<IR_SPEC, 25> {
        BO_W::new(self)
    }
    #[doc = "Bit 26 - Watchdog Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wdi(&mut self) -> WDI_W<IR_SPEC, 26> {
        WDI_W::new(self)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase"]
    #[inline(always)]
    #[must_use]
    pub fn pea(&mut self) -> PEA_W<IR_SPEC, 27> {
        PEA_W::new(self)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase"]
    #[inline(always)]
    #[must_use]
    pub fn ped(&mut self) -> PED_W<IR_SPEC, 28> {
        PED_W::new(self)
    }
    #[doc = "Bit 29 - Access to Reserved Address"]
    #[inline(always)]
    #[must_use]
    pub fn ara(&mut self) -> ARA_W<IR_SPEC, 29> {
        ARA_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ir::W`](W) writer structure"]
impl crate::Writable for IR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
