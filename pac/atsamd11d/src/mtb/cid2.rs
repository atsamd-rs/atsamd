#[doc = "Register `CID2` reader"]
pub type R = crate::R<CID2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CID2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CID2_SPEC;
impl crate::RegisterSpec for CID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid2::R`](R) reader structure"]
impl crate::Readable for CID2_SPEC {}
#[doc = "`reset()` method sets CID2 to value 0"]
impl crate::Resettable for CID2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
