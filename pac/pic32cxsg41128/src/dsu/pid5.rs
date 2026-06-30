#[doc = "Register `PID5` reader"]
pub type R = crate::R<Pid5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Peripheral Identification 5\n\nYou can [`read`](crate::Reg::read) this register and get [`pid5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid5Spec;
impl crate::RegisterSpec for Pid5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid5::R`](R) reader structure"]
impl crate::Readable for Pid5Spec {}
#[doc = "`reset()` method sets PID5 to value 0"]
impl crate::Resettable for Pid5Spec {
    const RESET_VALUE: u32 = 0;
}
