#[doc = "Register `PID4` reader"]
pub type R = crate::R<Pid4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`pid4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid4Spec;
impl crate::RegisterSpec for Pid4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid4::R`](R) reader structure"]
impl crate::Readable for Pid4Spec {}
#[doc = "`reset()` method sets PID4 to value 0"]
impl crate::Resettable for Pid4Spec {}
