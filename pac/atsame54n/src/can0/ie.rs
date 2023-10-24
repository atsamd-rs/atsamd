#[doc = "Register `IE` reader"]
pub type R = crate::R<IE_SPEC>;
#[doc = "Register `IE` writer"]
pub type W = crate::W<IE_SPEC>;
#[doc = "Field `RF0NE` reader - Rx FIFO 0 New Message Interrupt Enable"]
pub type RF0NE_R = crate::BitReader;
#[doc = "Field `RF0NE` writer - Rx FIFO 0 New Message Interrupt Enable"]
pub type RF0NE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF0WE` reader - Rx FIFO 0 Watermark Reached Interrupt Enable"]
pub type RF0WE_R = crate::BitReader;
#[doc = "Field `RF0WE` writer - Rx FIFO 0 Watermark Reached Interrupt Enable"]
pub type RF0WE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF0FE` reader - Rx FIFO 0 Full Interrupt Enable"]
pub type RF0FE_R = crate::BitReader;
#[doc = "Field `RF0FE` writer - Rx FIFO 0 Full Interrupt Enable"]
pub type RF0FE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF0LE` reader - Rx FIFO 0 Message Lost Interrupt Enable"]
pub type RF0LE_R = crate::BitReader;
#[doc = "Field `RF0LE` writer - Rx FIFO 0 Message Lost Interrupt Enable"]
pub type RF0LE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF1NE` reader - Rx FIFO 1 New Message Interrupt Enable"]
pub type RF1NE_R = crate::BitReader;
#[doc = "Field `RF1NE` writer - Rx FIFO 1 New Message Interrupt Enable"]
pub type RF1NE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF1WE` reader - Rx FIFO 1 Watermark Reached Interrupt Enable"]
pub type RF1WE_R = crate::BitReader;
#[doc = "Field `RF1WE` writer - Rx FIFO 1 Watermark Reached Interrupt Enable"]
pub type RF1WE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF1FE` reader - Rx FIFO 1 FIFO Full Interrupt Enable"]
pub type RF1FE_R = crate::BitReader;
#[doc = "Field `RF1FE` writer - Rx FIFO 1 FIFO Full Interrupt Enable"]
pub type RF1FE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF1LE` reader - Rx FIFO 1 Message Lost Interrupt Enable"]
pub type RF1LE_R = crate::BitReader;
#[doc = "Field `RF1LE` writer - Rx FIFO 1 Message Lost Interrupt Enable"]
pub type RF1LE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPME` reader - High Priority Message Interrupt Enable"]
pub type HPME_R = crate::BitReader;
#[doc = "Field `HPME` writer - High Priority Message Interrupt Enable"]
pub type HPME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCE` reader - Timestamp Completed Interrupt Enable"]
pub type TCE_R = crate::BitReader;
#[doc = "Field `TCE` writer - Timestamp Completed Interrupt Enable"]
pub type TCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCFE` reader - Transmission Cancellation Finished Interrupt Enable"]
pub type TCFE_R = crate::BitReader;
#[doc = "Field `TCFE` writer - Transmission Cancellation Finished Interrupt Enable"]
pub type TCFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFEE` reader - Tx FIFO Empty Interrupt Enable"]
pub type TFEE_R = crate::BitReader;
#[doc = "Field `TFEE` writer - Tx FIFO Empty Interrupt Enable"]
pub type TFEE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEFNE` reader - Tx Event FIFO New Entry Interrupt Enable"]
pub type TEFNE_R = crate::BitReader;
#[doc = "Field `TEFNE` writer - Tx Event FIFO New Entry Interrupt Enable"]
pub type TEFNE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEFWE` reader - Tx Event FIFO Watermark Reached Interrupt Enable"]
pub type TEFWE_R = crate::BitReader;
#[doc = "Field `TEFWE` writer - Tx Event FIFO Watermark Reached Interrupt Enable"]
pub type TEFWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEFFE` reader - Tx Event FIFO Full Interrupt Enable"]
pub type TEFFE_R = crate::BitReader;
#[doc = "Field `TEFFE` writer - Tx Event FIFO Full Interrupt Enable"]
pub type TEFFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEFLE` reader - Tx Event FIFO Element Lost Interrupt Enable"]
pub type TEFLE_R = crate::BitReader;
#[doc = "Field `TEFLE` writer - Tx Event FIFO Element Lost Interrupt Enable"]
pub type TEFLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSWE` reader - Timestamp Wraparound Interrupt Enable"]
pub type TSWE_R = crate::BitReader;
#[doc = "Field `TSWE` writer - Timestamp Wraparound Interrupt Enable"]
pub type TSWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MRAFE` reader - Message RAM Access Failure Interrupt Enable"]
pub type MRAFE_R = crate::BitReader;
#[doc = "Field `MRAFE` writer - Message RAM Access Failure Interrupt Enable"]
pub type MRAFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TOOE` reader - Timeout Occurred Interrupt Enable"]
pub type TOOE_R = crate::BitReader;
#[doc = "Field `TOOE` writer - Timeout Occurred Interrupt Enable"]
pub type TOOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRXE` reader - Message stored to Dedicated Rx Buffer Interrupt Enable"]
pub type DRXE_R = crate::BitReader;
#[doc = "Field `DRXE` writer - Message stored to Dedicated Rx Buffer Interrupt Enable"]
pub type DRXE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BECE` reader - Bit Error Corrected Interrupt Enable"]
pub type BECE_R = crate::BitReader;
#[doc = "Field `BECE` writer - Bit Error Corrected Interrupt Enable"]
pub type BECE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BEUE` reader - Bit Error Uncorrected Interrupt Enable"]
pub type BEUE_R = crate::BitReader;
#[doc = "Field `BEUE` writer - Bit Error Uncorrected Interrupt Enable"]
pub type BEUE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ELOE` reader - Error Logging Overflow Interrupt Enable"]
pub type ELOE_R = crate::BitReader;
#[doc = "Field `ELOE` writer - Error Logging Overflow Interrupt Enable"]
pub type ELOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPE` reader - Error Passive Interrupt Enable"]
pub type EPE_R = crate::BitReader;
#[doc = "Field `EPE` writer - Error Passive Interrupt Enable"]
pub type EPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EWE` reader - Warning Status Interrupt Enable"]
pub type EWE_R = crate::BitReader;
#[doc = "Field `EWE` writer - Warning Status Interrupt Enable"]
pub type EWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOE` reader - Bus_Off Status Interrupt Enable"]
pub type BOE_R = crate::BitReader;
#[doc = "Field `BOE` writer - Bus_Off Status Interrupt Enable"]
pub type BOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDIE` reader - Watchdog Interrupt Interrupt Enable"]
pub type WDIE_R = crate::BitReader;
#[doc = "Field `WDIE` writer - Watchdog Interrupt Interrupt Enable"]
pub type WDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEAE` reader - Protocol Error in Arbitration Phase Enable"]
pub type PEAE_R = crate::BitReader;
#[doc = "Field `PEAE` writer - Protocol Error in Arbitration Phase Enable"]
pub type PEAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEDE` reader - Protocol Error in Data Phase Enable"]
pub type PEDE_R = crate::BitReader;
#[doc = "Field `PEDE` writer - Protocol Error in Data Phase Enable"]
pub type PEDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ARAE` reader - Access to Reserved Address Enable"]
pub type ARAE_R = crate::BitReader;
#[doc = "Field `ARAE` writer - Access to Reserved Address Enable"]
pub type ARAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Interrupt Enable"]
    #[inline(always)]
    pub fn rf0ne(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub fn rf0we(&self) -> RF0WE_R {
        RF0WE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Interrupt Enable"]
    #[inline(always)]
    pub fn rf0fe(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Interrupt Enable"]
    #[inline(always)]
    pub fn rf0le(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Interrupt Enable"]
    #[inline(always)]
    pub fn rf1ne(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub fn rf1we(&self) -> RF1WE_R {
        RF1WE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 FIFO Full Interrupt Enable"]
    #[inline(always)]
    pub fn rf1fe(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Interrupt Enable"]
    #[inline(always)]
    pub fn rf1le(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message Interrupt Enable"]
    #[inline(always)]
    pub fn hpme(&self) -> HPME_R {
        HPME_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Completed Interrupt Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tfee(&self) -> TFEE_R {
        TFEE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Interrupt Enable"]
    #[inline(always)]
    pub fn tefne(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub fn tefwe(&self) -> TEFWE_R {
        TEFWE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Interrupt Enable"]
    #[inline(always)]
    pub fn teffe(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Interrupt Enable"]
    #[inline(always)]
    pub fn tefle(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Interrupt Enable"]
    #[inline(always)]
    pub fn tswe(&self) -> TSWE_R {
        TSWE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Interrupt Enable"]
    #[inline(always)]
    pub fn mrafe(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred Interrupt Enable"]
    #[inline(always)]
    pub fn tooe(&self) -> TOOE_R {
        TOOE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn drxe(&self) -> DRXE_R {
        DRXE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit Error Corrected Interrupt Enable"]
    #[inline(always)]
    pub fn bece(&self) -> BECE_R {
        BECE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Interrupt Enable"]
    #[inline(always)]
    pub fn beue(&self) -> BEUE_R {
        BEUE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn eloe(&self) -> ELOE_R {
        ELOE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive Interrupt Enable"]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status Interrupt Enable"]
    #[inline(always)]
    pub fn ewe(&self) -> EWE_R {
        EWE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status Interrupt Enable"]
    #[inline(always)]
    pub fn boe(&self) -> BOE_R {
        BOE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Interrupt Enable"]
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Enable"]
    #[inline(always)]
    pub fn peae(&self) -> PEAE_R {
        PEAE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Enable"]
    #[inline(always)]
    pub fn pede(&self) -> PEDE_R {
        PEDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address Enable"]
    #[inline(always)]
    pub fn arae(&self) -> ARAE_R {
        ARAE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0ne(&mut self) -> RF0NE_W<IE_SPEC, 0> {
        RF0NE_W::new(self)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0we(&mut self) -> RF0WE_W<IE_SPEC, 1> {
        RF0WE_W::new(self)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0fe(&mut self) -> RF0FE_W<IE_SPEC, 2> {
        RF0FE_W::new(self)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0le(&mut self) -> RF0LE_W<IE_SPEC, 3> {
        RF0LE_W::new(self)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1ne(&mut self) -> RF1NE_W<IE_SPEC, 4> {
        RF1NE_W::new(self)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1we(&mut self) -> RF1WE_W<IE_SPEC, 5> {
        RF1WE_W::new(self)
    }
    #[doc = "Bit 6 - Rx FIFO 1 FIFO Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1fe(&mut self) -> RF1FE_W<IE_SPEC, 6> {
        RF1FE_W::new(self)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1le(&mut self) -> RF1LE_W<IE_SPEC, 7> {
        RF1LE_W::new(self)
    }
    #[doc = "Bit 8 - High Priority Message Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpme(&mut self) -> HPME_W<IE_SPEC, 8> {
        HPME_W::new(self)
    }
    #[doc = "Bit 9 - Timestamp Completed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TCE_W<IE_SPEC, 9> {
        TCE_W::new(self)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcfe(&mut self) -> TCFE_W<IE_SPEC, 10> {
        TCFE_W::new(self)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfee(&mut self) -> TFEE_W<IE_SPEC, 11> {
        TFEE_W::new(self)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tefne(&mut self) -> TEFNE_W<IE_SPEC, 12> {
        TEFNE_W::new(self)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tefwe(&mut self) -> TEFWE_W<IE_SPEC, 13> {
        TEFWE_W::new(self)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn teffe(&mut self) -> TEFFE_W<IE_SPEC, 14> {
        TEFFE_W::new(self)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tefle(&mut self) -> TEFLE_W<IE_SPEC, 15> {
        TEFLE_W::new(self)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tswe(&mut self) -> TSWE_W<IE_SPEC, 16> {
        TSWE_W::new(self)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mrafe(&mut self) -> MRAFE_W<IE_SPEC, 17> {
        MRAFE_W::new(self)
    }
    #[doc = "Bit 18 - Timeout Occurred Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tooe(&mut self) -> TOOE_W<IE_SPEC, 18> {
        TOOE_W::new(self)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn drxe(&mut self) -> DRXE_W<IE_SPEC, 19> {
        DRXE_W::new(self)
    }
    #[doc = "Bit 20 - Bit Error Corrected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bece(&mut self) -> BECE_W<IE_SPEC, 20> {
        BECE_W::new(self)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn beue(&mut self) -> BEUE_W<IE_SPEC, 21> {
        BEUE_W::new(self)
    }
    #[doc = "Bit 22 - Error Logging Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eloe(&mut self) -> ELOE_W<IE_SPEC, 22> {
        ELOE_W::new(self)
    }
    #[doc = "Bit 23 - Error Passive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epe(&mut self) -> EPE_W<IE_SPEC, 23> {
        EPE_W::new(self)
    }
    #[doc = "Bit 24 - Warning Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ewe(&mut self) -> EWE_W<IE_SPEC, 24> {
        EWE_W::new(self)
    }
    #[doc = "Bit 25 - Bus_Off Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn boe(&mut self) -> BOE_W<IE_SPEC, 25> {
        BOE_W::new(self)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdie(&mut self) -> WDIE_W<IE_SPEC, 26> {
        WDIE_W::new(self)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Enable"]
    #[inline(always)]
    #[must_use]
    pub fn peae(&mut self) -> PEAE_W<IE_SPEC, 27> {
        PEAE_W::new(self)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pede(&mut self) -> PEDE_W<IE_SPEC, 28> {
        PEDE_W::new(self)
    }
    #[doc = "Bit 29 - Access to Reserved Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arae(&mut self) -> ARAE_W<IE_SPEC, 29> {
        ARAE_W::new(self)
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
#[doc = "Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
