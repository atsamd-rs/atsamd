#[doc = "Register `PID7` reader"]
pub type R = crate::R<Pid7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Peripheral Identification 7\n\nYou can [`read`](crate::Reg::read) this register and get [`pid7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid7Spec;
impl crate::RegisterSpec for Pid7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid7::R`](R) reader structure"]
impl crate::Readable for Pid7Spec {}
#[doc = "`reset()` method sets PID7 to value 0"]
impl crate::Resettable for Pid7Spec {
    const RESET_VALUE: u32 = 0;
}
