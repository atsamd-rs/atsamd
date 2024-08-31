#[doc = "Register `TRIGGER` reader"]
pub type R = crate::R<TriggerSpec>;
#[doc = "Field `TRIGGER` reader - "]
pub type TriggerR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn trigger(&self) -> TriggerR {
        TriggerR::new((self.bits & 1) != 0)
    }
}
#[doc = "TRIGGER\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TriggerSpec;
impl crate::RegisterSpec for TriggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigger::R`](R) reader structure"]
impl crate::Readable for TriggerSpec {}
#[doc = "`reset()` method sets TRIGGER to value 0"]
impl crate::Resettable for TriggerSpec {
    const RESET_VALUE: u32 = 0;
}
