#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Field `VALUE` reader - Measurement Value"]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Measurement Value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Count Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`reset()` method sets VALUE to value 0"]
impl crate::Resettable for ValueSpec {
    const RESET_VALUE: u32 = 0;
}
