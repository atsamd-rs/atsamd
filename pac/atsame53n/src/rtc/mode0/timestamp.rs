#[doc = "Register `TIMESTAMP` reader"]
pub type R = crate::R<TIMESTAMP_SPEC>;
#[doc = "Field `COUNT` reader - Count Timestamp Value"]
pub type COUNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Count Timestamp Value"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
#[doc = "MODE0 Timestamp\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestamp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMESTAMP_SPEC;
impl crate::RegisterSpec for TIMESTAMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp::R`](R) reader structure"]
impl crate::Readable for TIMESTAMP_SPEC {}
#[doc = "`reset()` method sets TIMESTAMP to value 0"]
impl crate::Resettable for TIMESTAMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
