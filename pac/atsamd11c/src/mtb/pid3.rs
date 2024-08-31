#[doc = "Register `PID3` reader"]
pub type R = crate::R<Pid3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`pid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid3Spec;
impl crate::RegisterSpec for Pid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid3::R`](R) reader structure"]
impl crate::Readable for Pid3Spec {}
#[doc = "`reset()` method sets PID3 to value 0"]
impl crate::Resettable for Pid3Spec {
    const RESET_VALUE: u32 = 0;
}
