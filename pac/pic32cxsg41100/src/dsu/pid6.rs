#[doc = "Register `PID6` reader"]
pub type R = crate::R<Pid6Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Peripheral Identification 6\n\nYou can [`read`](crate::Reg::read) this register and get [`pid6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid6Spec;
impl crate::RegisterSpec for Pid6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid6::R`](R) reader structure"]
impl crate::Readable for Pid6Spec {}
#[doc = "`reset()` method sets PID6 to value 0"]
impl crate::Resettable for Pid6Spec {
    const RESET_VALUE: u32 = 0;
}
