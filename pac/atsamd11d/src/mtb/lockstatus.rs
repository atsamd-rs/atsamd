#[doc = "Register `LOCKSTATUS` reader"]
pub type R = crate::R<LockstatusSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "MTB Lock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`lockstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockstatusSpec;
impl crate::RegisterSpec for LockstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lockstatus::R`](R) reader structure"]
impl crate::Readable for LockstatusSpec {}
#[doc = "`reset()` method sets LOCKSTATUS to value 0"]
impl crate::Resettable for LockstatusSpec {
    const RESET_VALUE: u32 = 0;
}
