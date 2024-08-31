#[doc = "Register `CID1` reader"]
pub type R = crate::R<Cid1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`cid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cid1Spec;
impl crate::RegisterSpec for Cid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid1::R`](R) reader structure"]
impl crate::Readable for Cid1Spec {}
#[doc = "`reset()` method sets CID1 to value 0"]
impl crate::Resettable for Cid1Spec {
    const RESET_VALUE: u32 = 0;
}
