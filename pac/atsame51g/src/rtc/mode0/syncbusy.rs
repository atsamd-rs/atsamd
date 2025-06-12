#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - Software Reset Busy"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `ENABLE` reader - Enable Bit Busy"]
pub type EnableR = crate::BitReader;
#[doc = "Field `FREQCORR` reader - FREQCORR Register Busy"]
pub type FreqcorrR = crate::BitReader;
#[doc = "Field `COUNT` reader - COUNT Register Busy"]
pub type CountR = crate::BitReader;
#[doc = "Field `COMP0` reader - COMP 0 Register Busy"]
pub type Comp0R = crate::BitReader;
#[doc = "Field `COMP1` reader - COMP 1 Register Busy"]
pub type Comp1R = crate::BitReader;
#[doc = "Field `COUNTSYNC` reader - Count Synchronization Enable Bit Busy"]
pub type CountsyncR = crate::BitReader;
#[doc = "Field `GP0` reader - General Purpose 0 Register Busy"]
pub type Gp0R = crate::BitReader;
#[doc = "Field `GP1` reader - General Purpose 1 Register Busy"]
pub type Gp1R = crate::BitReader;
#[doc = "Field `GP2` reader - General Purpose 2 Register Busy"]
pub type Gp2R = crate::BitReader;
#[doc = "Field `GP3` reader - General Purpose 3 Register Busy"]
pub type Gp3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software Reset Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Bit Busy"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FREQCORR Register Busy"]
    #[inline(always)]
    pub fn freqcorr(&self) -> FreqcorrR {
        FreqcorrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COUNT Register Busy"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - COMP 0 Register Busy"]
    #[inline(always)]
    pub fn comp0(&self) -> Comp0R {
        Comp0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - COMP 1 Register Busy"]
    #[inline(always)]
    pub fn comp1(&self) -> Comp1R {
        Comp1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 15 - Count Synchronization Enable Bit Busy"]
    #[inline(always)]
    pub fn countsync(&self) -> CountsyncR {
        CountsyncR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - General Purpose 0 Register Busy"]
    #[inline(always)]
    pub fn gp0(&self) -> Gp0R {
        Gp0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - General Purpose 1 Register Busy"]
    #[inline(always)]
    pub fn gp1(&self) -> Gp1R {
        Gp1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - General Purpose 2 Register Busy"]
    #[inline(always)]
    pub fn gp2(&self) -> Gp2R {
        Gp2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - General Purpose 3 Register Busy"]
    #[inline(always)]
    pub fn gp3(&self) -> Gp3R {
        Gp3R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "MODE0 Synchronization Busy Status\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
