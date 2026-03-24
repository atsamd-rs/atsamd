#[doc = "Register `SYNCFR` reader"]
pub type R = crate::R<SyncfrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Synchronization Frequency Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncfrSpec;
impl crate::RegisterSpec for SyncfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncfr::R`](R) reader structure"]
impl crate::Readable for SyncfrSpec {}
#[doc = "`reset()` method sets SYNCFR to value 0x0400"]
impl crate::Resettable for SyncfrSpec {
    const RESET_VALUE: u32 = 0x0400;
}
