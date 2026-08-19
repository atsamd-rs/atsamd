#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - swrst"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `ENABLE` reader - enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `CTRLB` reader - CTRLB"]
pub type CtrlbR = crate::BitReader;
#[doc = "Field `STATUS` reader - STATUS"]
pub type StatusR = crate::BitReader;
#[doc = "Field `COUNT` reader - Counter"]
pub type CountR = crate::BitReader;
#[doc = "Field `PER` reader - Period"]
pub type PerR = crate::BitReader;
#[doc = "Field `CC0` reader - Compare Channel 0"]
pub type Cc0R = crate::BitReader;
#[doc = "Field `CC1` reader - Compare Channel 1"]
pub type Cc1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - swrst"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTRLB"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CtrlbR {
        CtrlbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STATUS"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counter"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Period"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare Channel 0"]
    #[inline(always)]
    pub fn cc0(&self) -> Cc0R {
        Cc0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare Channel 1"]
    #[inline(always)]
    pub fn cc1(&self) -> Cc1R {
        Cc1R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Synchronization Status\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
