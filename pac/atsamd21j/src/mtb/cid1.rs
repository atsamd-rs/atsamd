#[doc = "Register `CID1` reader"]
pub type R = crate::R<CID1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CID1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "CoreSight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CID1_SPEC;
impl crate::RegisterSpec for CID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid1::R`](R) reader structure"]
impl crate::Readable for CID1_SPEC {}
#[doc = "`reset()` method sets CID1 to value 0"]
impl crate::Resettable for CID1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
