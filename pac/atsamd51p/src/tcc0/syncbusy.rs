#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSY_SPEC>;
#[doc = "Field `SWRST` reader - Swrst Busy"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `ENABLE` reader - Enable Busy"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `CTRLB` reader - Ctrlb Busy"]
pub type CTRLB_R = crate::BitReader;
#[doc = "Field `STATUS` reader - Status Busy"]
pub type STATUS_R = crate::BitReader;
#[doc = "Field `COUNT` reader - Count Busy"]
pub type COUNT_R = crate::BitReader;
#[doc = "Field `PATT` reader - Pattern Busy"]
pub type PATT_R = crate::BitReader;
#[doc = "Field `WAVE` reader - Wave Busy"]
pub type WAVE_R = crate::BitReader;
#[doc = "Field `PER` reader - Period Busy"]
pub type PER_R = crate::BitReader;
#[doc = "Field `CC0` reader - Compare Channel 0 Busy"]
pub type CC0_R = crate::BitReader;
#[doc = "Field `CC1` reader - Compare Channel 1 Busy"]
pub type CC1_R = crate::BitReader;
#[doc = "Field `CC2` reader - Compare Channel 2 Busy"]
pub type CC2_R = crate::BitReader;
#[doc = "Field `CC3` reader - Compare Channel 3 Busy"]
pub type CC3_R = crate::BitReader;
#[doc = "Field `CC4` reader - Compare Channel 4 Busy"]
pub type CC4_R = crate::BitReader;
#[doc = "Field `CC5` reader - Compare Channel 5 Busy"]
pub type CC5_R = crate::BitReader;
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
    #[doc = "Bit 7 - Period Busy"]
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
    #[doc = "Bit 12 - Compare Channel 4 Busy"]
    #[inline(always)]
    pub fn cc4(&self) -> CC4_R {
        CC4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare Channel 5 Busy"]
    #[inline(always)]
    pub fn cc5(&self) -> CC5_R {
        CC5_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Synchronization Busy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
