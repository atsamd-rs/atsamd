#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - Software Reset Synchronization Busy"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `ENABLE` reader - Enable Synchronization Busy"]
pub type EnableR = crate::BitReader;
#[doc = "Field `WINCTRL` reader - WINCTRL Synchronization Busy"]
pub type WinctrlR = crate::BitReader;
#[doc = "Field `COMPCTRL0` reader - COMPCTRL 0 Synchronization Busy"]
pub type Compctrl0R = crate::BitReader;
#[doc = "Field `COMPCTRL1` reader - COMPCTRL 1 Synchronization Busy"]
pub type Compctrl1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software Reset Synchronization Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WINCTRL Synchronization Busy"]
    #[inline(always)]
    pub fn winctrl(&self) -> WinctrlR {
        WinctrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COMPCTRL 0 Synchronization Busy"]
    #[inline(always)]
    pub fn compctrl0(&self) -> Compctrl0R {
        Compctrl0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - COMPCTRL 1 Synchronization Busy"]
    #[inline(always)]
    pub fn compctrl1(&self) -> Compctrl1R {
        Compctrl1R::new(((self.bits >> 4) & 1) != 0)
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
impl crate::Resettable for SyncbusySpec {
    const RESET_VALUE: u32 = 0;
}
