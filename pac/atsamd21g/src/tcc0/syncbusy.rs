#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - Swrst Busy"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `ENABLE` reader - Enable Busy"]
pub type EnableR = crate::BitReader;
#[doc = "Field `CTRLB` reader - Ctrlb Busy"]
pub type CtrlbR = crate::BitReader;
#[doc = "Field `STATUS` reader - Status Busy"]
pub type StatusR = crate::BitReader;
#[doc = "Field `COUNT` reader - Count Busy"]
pub type CountR = crate::BitReader;
#[doc = "Field `PATT` reader - Pattern Busy"]
pub type PattR = crate::BitReader;
#[doc = "Field `WAVE` reader - Wave Busy"]
pub type WaveR = crate::BitReader;
#[doc = "Field `PER` reader - Period busy"]
pub type PerR = crate::BitReader;
#[doc = "Field `CC0` reader - Compare Channel 0 Busy"]
pub type Cc0R = crate::BitReader;
#[doc = "Field `CC1` reader - Compare Channel 1 Busy"]
pub type Cc1R = crate::BitReader;
#[doc = "Field `CC2` reader - Compare Channel 2 Busy"]
pub type Cc2R = crate::BitReader;
#[doc = "Field `CC3` reader - Compare Channel 3 Busy"]
pub type Cc3R = crate::BitReader;
#[doc = "Field `PATTB` reader - Pattern Buffer Busy"]
pub type PattbR = crate::BitReader;
#[doc = "Field `WAVEB` reader - Wave Buffer Busy"]
pub type WavebR = crate::BitReader;
#[doc = "Field `PERB` reader - Period Buffer Busy"]
pub type PerbR = crate::BitReader;
#[doc = "Field `CCB0` reader - Compare Channel Buffer 0 Busy"]
pub type Ccb0R = crate::BitReader;
#[doc = "Field `CCB1` reader - Compare Channel Buffer 1 Busy"]
pub type Ccb1R = crate::BitReader;
#[doc = "Field `CCB2` reader - Compare Channel Buffer 2 Busy"]
pub type Ccb2R = crate::BitReader;
#[doc = "Field `CCB3` reader - Compare Channel Buffer 3 Busy"]
pub type Ccb3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Swrst Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Busy"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ctrlb Busy"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CtrlbR {
        CtrlbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status Busy"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Count Busy"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pattern Busy"]
    #[inline(always)]
    pub fn patt(&self) -> PattR {
        PattR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wave Busy"]
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Period busy"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare Channel 0 Busy"]
    #[inline(always)]
    pub fn cc0(&self) -> Cc0R {
        Cc0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare Channel 1 Busy"]
    #[inline(always)]
    pub fn cc1(&self) -> Cc1R {
        Cc1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Compare Channel 2 Busy"]
    #[inline(always)]
    pub fn cc2(&self) -> Cc2R {
        Cc2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Compare Channel 3 Busy"]
    #[inline(always)]
    pub fn cc3(&self) -> Cc3R {
        Cc3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Pattern Buffer Busy"]
    #[inline(always)]
    pub fn pattb(&self) -> PattbR {
        PattbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wave Buffer Busy"]
    #[inline(always)]
    pub fn waveb(&self) -> WavebR {
        WavebR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Period Buffer Busy"]
    #[inline(always)]
    pub fn perb(&self) -> PerbR {
        PerbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Compare Channel Buffer 0 Busy"]
    #[inline(always)]
    pub fn ccb0(&self) -> Ccb0R {
        Ccb0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Compare Channel Buffer 1 Busy"]
    #[inline(always)]
    pub fn ccb1(&self) -> Ccb1R {
        Ccb1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Compare Channel Buffer 2 Busy"]
    #[inline(always)]
    pub fn ccb2(&self) -> Ccb2R {
        Ccb2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Compare Channel Buffer 3 Busy"]
    #[inline(always)]
    pub fn ccb3(&self) -> Ccb3R {
        Ccb3R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
