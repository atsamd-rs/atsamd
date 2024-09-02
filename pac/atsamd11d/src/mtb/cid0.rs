#[doc = "Register `CID0` reader"]
pub type R = crate::R<Cid0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`cid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cid0Spec;
impl crate::RegisterSpec for Cid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid0::R`](R) reader structure"]
impl crate::Readable for Cid0Spec {}
#[doc = "`reset()` method sets CID0 to value 0"]
impl crate::Resettable for Cid0Spec {
    const RESET_VALUE: u32 = 0;
}
