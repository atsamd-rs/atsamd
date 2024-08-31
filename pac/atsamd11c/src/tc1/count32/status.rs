#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `STOP` reader - Stop"]
pub type StopR = crate::BitReader;
#[doc = "Field `SLAVE` reader - Slave"]
pub type SlaveR = crate::BitReader;
#[doc = "Field `SYNCBUSY` reader - Synchronization Busy"]
pub type SyncbusyR = crate::BitReader;
impl R {
    #[doc = "Bit 3 - Stop"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave"]
    #[inline(always)]
    pub fn slave(&self) -> SlaveR {
        SlaveR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronization Busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SyncbusyR {
        SyncbusyR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x08"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u8 = 0x08;
}
