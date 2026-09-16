#[doc = "Register `TIMESTAMP` reader"]
pub type R = crate::R<TimestampSpec>;
#[doc = "Field `COUNT` reader - Count Timestamp Value"]
pub type CountR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Count Timestamp Value"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MODE1 Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimestampSpec;
impl crate::RegisterSpec for TimestampSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp::R`](R) reader structure"]
impl crate::Readable for TimestampSpec {}
#[doc = "`reset()` method sets TIMESTAMP to value 0"]
impl crate::Resettable for TimestampSpec {
    const RESET_VALUE: u32 = 0;
}
