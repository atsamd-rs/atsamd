#[doc = "Register `CHIPID_EXID` reader"]
pub struct R(crate::R<CHIPID_EXID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIPID_EXID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIPID_EXID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIPID_EXID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXID` reader - Chip ID Extension"]
pub struct EXID_R(crate::FieldReader<u32, u32>);
impl EXID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EXID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Chip ID Extension"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new(self.bits as u32)
    }
}
#[doc = "Chip ID Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chipid_exid](index.html) module"]
pub struct CHIPID_EXID_SPEC;
impl crate::RegisterSpec for CHIPID_EXID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chipid_exid::R](R) reader structure"]
impl crate::Readable for CHIPID_EXID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHIPID_EXID to value 0"]
impl crate::Resettable for CHIPID_EXID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
