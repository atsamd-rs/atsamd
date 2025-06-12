#[doc = "Register `CID2` reader"]
pub type R = crate::R<Cid2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`cid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cid2Spec;
impl crate::RegisterSpec for Cid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid2::R`](R) reader structure"]
impl crate::Readable for Cid2Spec {}
#[doc = "`reset()` method sets CID2 to value 0"]
impl crate::Resettable for Cid2Spec {}
