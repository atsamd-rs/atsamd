#[doc = "Register `RESULT` reader"]
pub type R = crate::R<ResultSpec>;
#[doc = "Field `RESULT` reader - Result Conversion Value"]
pub type ResultR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Result Conversion Value"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new(self.bits)
    }
}
#[doc = "Result Conversion Value\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultSpec;
impl crate::RegisterSpec for ResultSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`result::R`](R) reader structure"]
impl crate::Readable for ResultSpec {}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for ResultSpec {}
