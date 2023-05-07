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
#[doc = "Field `SWRST` reader - Software Reset Bit Busy"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` reader - Enable Bit Busy"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `FREQCORR` reader - FREQCORR Register Busy"]
pub type FREQCORR_R = crate::BitReader<bool>;
#[doc = "Field `CLOCK` reader - CLOCK Register Busy"]
pub type CLOCK_R = crate::BitReader<bool>;
#[doc = "Field `ALARM0` reader - ALARM 0 Register Busy"]
pub type ALARM0_R = crate::BitReader<bool>;
#[doc = "Field `ALARM1` reader - ALARM 1 Register Busy"]
pub type ALARM1_R = crate::BitReader<bool>;
#[doc = "Field `MASK0` reader - MASK 0 Register Busy"]
pub type MASK0_R = crate::BitReader<bool>;
#[doc = "Field `MASK1` reader - MASK 1 Register Busy"]
pub type MASK1_R = crate::BitReader<bool>;
#[doc = "Field `CLOCKSYNC` reader - Clock Synchronization Enable Bit Busy"]
pub type CLOCKSYNC_R = crate::BitReader<bool>;
#[doc = "Field `GP0` reader - General Purpose 0 Register Busy"]
pub type GP0_R = crate::BitReader<bool>;
#[doc = "Field `GP1` reader - General Purpose 1 Register Busy"]
pub type GP1_R = crate::BitReader<bool>;
#[doc = "Field `GP2` reader - General Purpose 2 Register Busy"]
pub type GP2_R = crate::BitReader<bool>;
#[doc = "Field `GP3` reader - General Purpose 3 Register Busy"]
pub type GP3_R = crate::BitReader<bool>;
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
    #[doc = "Bit 3 - CLOCK Register Busy"]
    #[inline(always)]
    pub fn clock(&self) -> CLOCK_R {
        CLOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - ALARM 0 Register Busy"]
    #[inline(always)]
    pub fn alarm0(&self) -> ALARM0_R {
        ALARM0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ALARM 1 Register Busy"]
    #[inline(always)]
    pub fn alarm1(&self) -> ALARM1_R {
        ALARM1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - MASK 0 Register Busy"]
    #[inline(always)]
    pub fn mask0(&self) -> MASK0_R {
        MASK0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MASK 1 Register Busy"]
    #[inline(always)]
    pub fn mask1(&self) -> MASK1_R {
        MASK1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock Synchronization Enable Bit Busy"]
    #[inline(always)]
    pub fn clocksync(&self) -> CLOCKSYNC_R {
        CLOCKSYNC_R::new(((self.bits >> 15) & 1) != 0)
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
#[doc = "MODE2 Synchronization Busy Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
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
