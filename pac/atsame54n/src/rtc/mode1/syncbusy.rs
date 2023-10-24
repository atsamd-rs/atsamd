#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSY_SPEC>;
#[doc = "Field `SWRST` reader - Software Reset Bit Busy"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `ENABLE` reader - Enable Bit Busy"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `FREQCORR` reader - FREQCORR Register Busy"]
pub type FREQCORR_R = crate::BitReader;
#[doc = "Field `COUNT` reader - COUNT Register Busy"]
pub type COUNT_R = crate::BitReader;
#[doc = "Field `PER` reader - PER Register Busy"]
pub type PER_R = crate::BitReader;
#[doc = "Field `COMP0` reader - COMP 0 Register Busy"]
pub type COMP0_R = crate::BitReader;
#[doc = "Field `COMP1` reader - COMP 1 Register Busy"]
pub type COMP1_R = crate::BitReader;
#[doc = "Field `COMP2` reader - COMP 2 Register Busy"]
pub type COMP2_R = crate::BitReader;
#[doc = "Field `COMP3` reader - COMP 3 Register Busy"]
pub type COMP3_R = crate::BitReader;
#[doc = "Field `COUNTSYNC` reader - Count Synchronization Enable Bit Busy"]
pub type COUNTSYNC_R = crate::BitReader;
#[doc = "Field `GP0` reader - General Purpose 0 Register Busy"]
pub type GP0_R = crate::BitReader;
#[doc = "Field `GP1` reader - General Purpose 1 Register Busy"]
pub type GP1_R = crate::BitReader;
#[doc = "Field `GP2` reader - General Purpose 2 Register Busy"]
pub type GP2_R = crate::BitReader;
#[doc = "Field `GP3` reader - General Purpose 3 Register Busy"]
pub type GP3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software Reset Bit Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Bit Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FREQCORR Register Busy"]
    #[inline(always)]
    pub fn freqcorr(&self) -> FREQCORR_R {
        FREQCORR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COUNT Register Busy"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PER Register Busy"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COMP 0 Register Busy"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - COMP 1 Register Busy"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - COMP 2 Register Busy"]
    #[inline(always)]
    pub fn comp2(&self) -> COMP2_R {
        COMP2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - COMP 3 Register Busy"]
    #[inline(always)]
    pub fn comp3(&self) -> COMP3_R {
        COMP3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - Count Synchronization Enable Bit Busy"]
    #[inline(always)]
    pub fn countsync(&self) -> COUNTSYNC_R {
        COUNTSYNC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - General Purpose 0 Register Busy"]
    #[inline(always)]
    pub fn gp0(&self) -> GP0_R {
        GP0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - General Purpose 1 Register Busy"]
    #[inline(always)]
    pub fn gp1(&self) -> GP1_R {
        GP1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - General Purpose 2 Register Busy"]
    #[inline(always)]
    pub fn gp2(&self) -> GP2_R {
        GP2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - General Purpose 3 Register Busy"]
    #[inline(always)]
    pub fn gp3(&self) -> GP3_R {
        GP3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "MODE1 Synchronization Busy Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
