#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - Software Reset Bit Busy"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `ENABLE` reader - Enable Bit Busy"]
pub type EnableR = crate::BitReader;
#[doc = "Field `FREQCORR` reader - FREQCORR Register Busy"]
pub type FreqcorrR = crate::BitReader;
#[doc = "Field `CLOCK` reader - CLOCK Register Busy"]
pub type ClockR = crate::BitReader;
#[doc = "Field `ALARM0` reader - ALARM 0 Register Busy"]
pub type Alarm0R = crate::BitReader;
#[doc = "Field `ALARM1` reader - ALARM 1 Register Busy"]
pub type Alarm1R = crate::BitReader;
#[doc = "Field `MASK0` reader - MASK 0 Register Busy"]
pub type Mask0R = crate::BitReader;
#[doc = "Field `MASK1` reader - MASK 1 Register Busy"]
pub type Mask1R = crate::BitReader;
#[doc = "Field `CLOCKSYNC` reader - Clock Synchronization Enable Bit Busy"]
pub type ClocksyncR = crate::BitReader;
#[doc = "Field `GP0` reader - General Purpose 0 Register Busy"]
pub type Gp0R = crate::BitReader;
#[doc = "Field `GP1` reader - General Purpose 1 Register Busy"]
pub type Gp1R = crate::BitReader;
#[doc = "Field `GP2` reader - General Purpose 2 Register Busy"]
pub type Gp2R = crate::BitReader;
#[doc = "Field `GP3` reader - General Purpose 3 Register Busy"]
pub type Gp3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software Reset Bit Busy"]
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
    #[doc = "Bit 3 - CLOCK Register Busy"]
    #[inline(always)]
    pub fn clock(&self) -> ClockR {
        ClockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - ALARM 0 Register Busy"]
    #[inline(always)]
    pub fn alarm0(&self) -> Alarm0R {
        Alarm0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ALARM 1 Register Busy"]
    #[inline(always)]
    pub fn alarm1(&self) -> Alarm1R {
        Alarm1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - MASK 0 Register Busy"]
    #[inline(always)]
    pub fn mask0(&self) -> Mask0R {
        Mask0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MASK 1 Register Busy"]
    #[inline(always)]
    pub fn mask1(&self) -> Mask1R {
        Mask1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock Synchronization Enable Bit Busy"]
    #[inline(always)]
    pub fn clocksync(&self) -> ClocksyncR {
        ClocksyncR::new(((self.bits >> 15) & 1) != 0)
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
#[doc = "MODE2 Synchronization Busy Status\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {
    const RESET_VALUE: u32 = 0;
}
