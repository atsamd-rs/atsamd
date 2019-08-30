#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTRLB`"]
pub type CTRLB_R = crate::R<bool, bool>;
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PATT`"]
pub type PATT_R = crate::R<bool, bool>;
#[doc = "Reader of field `WAVE`"]
pub type WAVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PER`"]
pub type PER_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC0`"]
pub type CC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC1`"]
pub type CC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC2`"]
pub type CC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC3`"]
pub type CC3_R = crate::R<bool, bool>;
#[doc = "Reader of field `PATTB`"]
pub type PATTB_R = crate::R<bool, bool>;
#[doc = "Reader of field `WAVEB`"]
pub type WAVEB_R = crate::R<bool, bool>;
#[doc = "Reader of field `PERB`"]
pub type PERB_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCB0`"]
pub type CCB0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCB1`"]
pub type CCB1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCB2`"]
pub type CCB2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCB3`"]
pub type CCB3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Swrst Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Ctrlb Busy"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CTRLB_R {
        CTRLB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status Busy"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Count Busy"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pattern Busy"]
    #[inline(always)]
    pub fn patt(&self) -> PATT_R {
        PATT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wave Busy"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Period busy"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Compare Channel 0 Busy"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Compare Channel 1 Busy"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Compare Channel 2 Busy"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Compare Channel 3 Busy"]
    #[inline(always)]
    pub fn cc3(&self) -> CC3_R {
        CC3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pattern Buffer Busy"]
    #[inline(always)]
    pub fn pattb(&self) -> PATTB_R {
        PATTB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Wave Buffer Busy"]
    #[inline(always)]
    pub fn waveb(&self) -> WAVEB_R {
        WAVEB_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Period Buffer Busy"]
    #[inline(always)]
    pub fn perb(&self) -> PERB_R {
        PERB_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Compare Channel Buffer 0 Busy"]
    #[inline(always)]
    pub fn ccb0(&self) -> CCB0_R {
        CCB0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Compare Channel Buffer 1 Busy"]
    #[inline(always)]
    pub fn ccb1(&self) -> CCB1_R {
        CCB1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Compare Channel Buffer 2 Busy"]
    #[inline(always)]
    pub fn ccb2(&self) -> CCB2_R {
        CCB2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Compare Channel Buffer 3 Busy"]
    #[inline(always)]
    pub fn ccb3(&self) -> CCB3_R {
        CCB3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
