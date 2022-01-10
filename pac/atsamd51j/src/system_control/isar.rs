#[doc = "Register `ISAR[%s]` reader"]
pub struct R(crate::R<ISAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Instruction Set Attributes Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isar](index.html) module"]
pub struct ISAR_SPEC;
impl crate::RegisterSpec for ISAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isar::R](R) reader structure"]
impl crate::Readable for ISAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISAR[%s]
to value 0"]
impl crate::Resettable for ISAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
