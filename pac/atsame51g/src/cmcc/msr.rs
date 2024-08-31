#[doc = "Register `MSR` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Field `EVENT_CNT` reader - Monitor Event Counter"]
pub type EventCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Monitor Event Counter"]
    #[inline(always)]
    pub fn event_cnt(&self) -> EventCntR {
        EventCntR::new(self.bits)
    }
}
#[doc = "Cache Monitor Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MsrSpec {
    const RESET_VALUE: u32 = 0;
}
