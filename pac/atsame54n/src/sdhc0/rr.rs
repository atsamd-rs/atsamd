#[doc = "Register `RR[%s]` reader"]
pub struct R(crate::R<RR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDRESP` reader - Command Response"]
pub struct CMDRESP_R(crate::FieldReader<u32, u32>);
impl CMDRESP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CMDRESP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDRESP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Command Response"]
    #[inline(always)]
    pub fn cmdresp(&self) -> CMDRESP_R {
        CMDRESP_R::new(self.bits as u32)
    }
}
#[doc = "Response\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rr](index.html) module"]
pub struct RR_SPEC;
impl crate::RegisterSpec for RR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rr::R](R) reader structure"]
impl crate::Readable for RR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RR[%s]
to value 0"]
impl crate::Resettable for RR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
