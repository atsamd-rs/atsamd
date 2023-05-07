#[doc = "Register `SYNCBUSY` reader"]
pub struct R(crate::R<SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SWRST` reader - Swrst Busy"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` reader - Enable Busy"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CTRLB` reader - Ctrlb Busy"]
pub type CTRLB_R = crate::BitReader<bool>;
#[doc = "Field `STATUS` reader - Status Busy"]
pub type STATUS_R = crate::BitReader<bool>;
#[doc = "Field `COUNT` reader - Count Busy"]
pub type COUNT_R = crate::BitReader<bool>;
#[doc = "Field `PATT` reader - Pattern Busy"]
pub type PATT_R = crate::BitReader<bool>;
#[doc = "Field `WAVE` reader - Wave Busy"]
pub type WAVE_R = crate::BitReader<bool>;
#[doc = "Field `PER` reader - Period busy"]
pub type PER_R = crate::BitReader<bool>;
#[doc = "Field `CC0` reader - Compare Channel 0 Busy"]
pub type CC0_R = crate::BitReader<bool>;
#[doc = "Field `CC1` reader - Compare Channel 1 Busy"]
pub type CC1_R = crate::BitReader<bool>;
#[doc = "Field `CC2` reader - Compare Channel 2 Busy"]
pub type CC2_R = crate::BitReader<bool>;
#[doc = "Field `CC3` reader - Compare Channel 3 Busy"]
pub type CC3_R = crate::BitReader<bool>;
#[doc = "Field `PATTB` reader - Pattern Buffer Busy"]
pub type PATTB_R = crate::BitReader<bool>;
#[doc = "Field `WAVEB` reader - Wave Buffer Busy"]
pub type WAVEB_R = crate::BitReader<bool>;
#[doc = "Field `PERB` reader - Period Buffer Busy"]
pub type PERB_R = crate::BitReader<bool>;
#[doc = "Field `CCB0` reader - Compare Channel Buffer 0 Busy"]
pub type CCB0_R = crate::BitReader<bool>;
#[doc = "Field `CCB1` reader - Compare Channel Buffer 1 Busy"]
pub type CCB1_R = crate::BitReader<bool>;
#[doc = "Field `CCB2` reader - Compare Channel Buffer 2 Busy"]
pub type CCB2_R = crate::BitReader<bool>;
#[doc = "Field `CCB3` reader - Compare Channel Buffer 3 Busy"]
pub type CCB3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Swrst Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ctrlb Busy"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CTRLB_R {
        CTRLB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status Busy"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Count Busy"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pattern Busy"]
    #[inline(always)]
    pub fn patt(&self) -> PATT_R {
        PATT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wave Busy"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Period busy"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare Channel 0 Busy"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare Channel 1 Busy"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Compare Channel 2 Busy"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Compare Channel 3 Busy"]
    #[inline(always)]
    pub fn cc3(&self) -> CC3_R {
        CC3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Pattern Buffer Busy"]
    #[inline(always)]
    pub fn pattb(&self) -> PATTB_R {
        PATTB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wave Buffer Busy"]
    #[inline(always)]
    pub fn waveb(&self) -> WAVEB_R {
        WAVEB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Period Buffer Busy"]
    #[inline(always)]
    pub fn perb(&self) -> PERB_R {
        PERB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Compare Channel Buffer 0 Busy"]
    #[inline(always)]
    pub fn ccb0(&self) -> CCB0_R {
        CCB0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Compare Channel Buffer 1 Busy"]
    #[inline(always)]
    pub fn ccb1(&self) -> CCB1_R {
        CCB1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Compare Channel Buffer 2 Busy"]
    #[inline(always)]
    pub fn ccb2(&self) -> CCB2_R {
        CCB2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Compare Channel Buffer 3 Busy"]
    #[inline(always)]
    pub fn ccb3(&self) -> CCB3_R {
        CCB3_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
