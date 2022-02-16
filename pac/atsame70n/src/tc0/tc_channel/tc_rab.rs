#[doc = "Register `TC_RAB` reader"]
pub struct R(crate::R<TC_RAB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TC_RAB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TC_RAB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TC_RAB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RAB` reader - Register A or Register B"]
pub struct RAB_R(crate::FieldReader<u32, u32>);
impl RAB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RAB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Register A or Register B"]
    #[inline(always)]
    pub fn rab(&self) -> RAB_R {
        RAB_R::new(self.bits as u32)
    }
}
#[doc = "Register AB (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_rab](index.html) module"]
pub struct TC_RAB_SPEC;
impl crate::RegisterSpec for TC_RAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tc_rab::R](R) reader structure"]
impl crate::Readable for TC_RAB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TC_RAB to value 0"]
impl crate::Resettable for TC_RAB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
