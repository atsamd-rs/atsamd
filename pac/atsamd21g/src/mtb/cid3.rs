#[doc = "Register `CID3` reader"]
pub type R = crate::R<Cid3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "CoreSight\n\nYou can [`read`](crate::Reg::read) this register and get [`cid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cid3Spec;
impl crate::RegisterSpec for Cid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid3::R`](R) reader structure"]
impl crate::Readable for Cid3Spec {}
#[doc = "`reset()` method sets CID3 to value 0"]
impl crate::Resettable for Cid3Spec {
    const RESET_VALUE: u32 = 0;
}
