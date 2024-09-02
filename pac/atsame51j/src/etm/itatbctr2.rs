#[doc = "Register `ITATBCTR2` reader"]
pub type R = crate::R<Itatbctr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ETM Integration Test ATB Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itatbctr2Spec;
impl crate::RegisterSpec for Itatbctr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itatbctr2::R`](R) reader structure"]
impl crate::Readable for Itatbctr2Spec {}
#[doc = "`reset()` method sets ITATBCTR2 to value 0"]
impl crate::Resettable for Itatbctr2Spec {
    const RESET_VALUE: u32 = 0;
}
