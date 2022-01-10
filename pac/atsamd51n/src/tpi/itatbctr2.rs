#[doc = "Register `ITATBCTR2` reader"]
pub struct R(crate::R<ITATBCTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITATBCTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITATBCTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITATBCTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ATREADY` reader - "]
pub struct ATREADY_R(crate::FieldReader<bool, bool>);
impl ATREADY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ATREADY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATREADY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn atready(&self) -> ATREADY_R {
        ATREADY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "ITATBCTR2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itatbctr2](index.html) module"]
pub struct ITATBCTR2_SPEC;
impl crate::RegisterSpec for ITATBCTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itatbctr2::R](R) reader structure"]
impl crate::Readable for ITATBCTR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITATBCTR2 to value 0"]
impl crate::Resettable for ITATBCTR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
