#[doc = "Register `MSR` reader"]
pub type R = crate::R<MSR_SPEC>;
#[doc = "Field `EVENT_CNT` reader - Monitor Event Counter"]
pub type EVENT_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Monitor Event Counter"]
    #[inline(always)]
    pub fn event_cnt(&self) -> EVENT_CNT_R {
        EVENT_CNT_R::new(self.bits)
    }
}
#[doc = "Cache Monitor Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MSR_SPEC {}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
