#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `ENABLE` reader - Enable Synchronization Busy"]
pub type EnableR = crate::BitReader;
#[doc = "Field `WEN` reader - Window Enable Synchronization Busy"]
pub type WenR = crate::BitReader;
#[doc = "Field `ALWAYSON` reader - Always-On Synchronization Busy"]
pub type AlwaysonR = crate::BitReader;
#[doc = "Field `CLEAR` reader - Clear Synchronization Busy"]
pub type ClearR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Enable Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Enable Synchronization Busy"]
    #[inline(always)]
    pub fn wen(&self) -> WenR {
        WenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Always-On Synchronization Busy"]
    #[inline(always)]
    pub fn alwayson(&self) -> AlwaysonR {
        AlwaysonR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear Synchronization Busy"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new(((self.bits >> 4) & 1) != 0)
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
