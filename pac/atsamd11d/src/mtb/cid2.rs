#[doc = "Register `CID2` reader"]
pub struct R(crate::R<CID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "CoreSight\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid2](index.html) module"]
pub struct CID2_SPEC;
impl crate::RegisterSpec for CID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cid2::R](R) reader structure"]
impl crate::Readable for CID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CID2 to value 0"]
impl crate::Resettable for CID2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
