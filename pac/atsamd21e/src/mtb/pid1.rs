#[doc = "Register `PID1` reader"]
pub type R = crate::R<Pid1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`pid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid1Spec;
impl crate::RegisterSpec for Pid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid1::R`](R) reader structure"]
impl crate::Readable for Pid1Spec {}
#[doc = "`reset()` method sets PID1 to value 0"]
impl crate::Resettable for Pid1Spec {}
