#[doc = "Register `ENTRY1` reader"]
pub struct R(crate::R<ENTRY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENTRY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENTRY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENTRY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "CoreSight ROM Table Entry 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [entry1](index.html) module"]
pub struct ENTRY1_SPEC;
impl crate::RegisterSpec for ENTRY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [entry1::R](R) reader structure"]
impl crate::Readable for ENTRY1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ENTRY1 to value 0x3002"]
impl crate::Resettable for ENTRY1_SPEC {
    const RESET_VALUE: Self::Ux = 0x3002;
}
