#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `SYNCBUSY` reader - Synchronization Busy Status"]
pub type SyncbusyR = crate::BitReader;
impl R {
    #[doc = "Bit 7 - Synchronization Busy Status"]
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
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u8 = 0;
}
