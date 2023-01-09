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
#[doc = "Field `SWRST` reader - Software Reset Busy"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` reader - Enable Bit Busy"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `FREQCORR` reader - FREQCORR Register Busy"]
pub type FREQCORR_R = crate::BitReader<bool>;
#[doc = "Field `COUNT` reader - COUNT Register Busy"]
pub type COUNT_R = crate::BitReader<bool>;
#[doc = "Field `COMP0` reader - COMP 0 Register Busy"]
pub type COMP0_R = crate::BitReader<bool>;
#[doc = "Field `COUNTSYNC` reader - Count Synchronization Enable Bit Busy"]
pub type COUNTSYNC_R = crate::BitReader<bool>;
#[doc = "Field `GP0` reader - General Purpose 0 Register Busy"]
pub type GP0_R = crate::BitReader<bool>;
#[doc = "Field `GP1` reader - General Purpose 1 Register Busy"]
pub type GP1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Software Reset Busy"]
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
    #[doc = "Bit 5 - COMP 0 Register Busy"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new(((self.bits >> 5) & 1) != 0)
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
}
#[doc = "MODE0 Synchronization Busy Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
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
