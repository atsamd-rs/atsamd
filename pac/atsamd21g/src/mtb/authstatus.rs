#[doc = "Register `AUTHSTATUS` reader"]
pub type R = crate::R<AuthstatusSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "MTB Authentication Status\n\nYou can [`read`](crate::Reg::read) this register and get [`authstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuthstatusSpec;
impl crate::RegisterSpec for AuthstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`authstatus::R`](R) reader structure"]
impl crate::Readable for AuthstatusSpec {}
#[doc = "`reset()` method sets AUTHSTATUS to value 0"]
impl crate::Resettable for AuthstatusSpec {
    const RESET_VALUE: u32 = 0;
}
