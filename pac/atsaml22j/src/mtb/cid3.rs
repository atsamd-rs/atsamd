#[doc = "Register `CID3` reader"]
pub struct R(crate::R<CID3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CID3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CID3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CID3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Component Identification 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid3](index.html) module"]
pub struct CID3_SPEC;
impl crate::RegisterSpec for CID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cid3::R](R) reader structure"]
impl crate::Readable for CID3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CID3 to value 0"]
impl crate::Resettable for CID3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
