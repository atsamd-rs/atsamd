#[doc = "Register `MSR` reader"]
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EVENT_CNT` reader - Monitor Event Counter"]
pub type EVENT_CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Monitor Event Counter"]
    #[inline(always)]
    pub fn event_cnt(&self) -> EVENT_CNT_R {
        EVENT_CNT_R::new(self.bits)
    }
}
#[doc = "Cache Monitor Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](index.html) module"]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msr::R](R) reader structure"]
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
