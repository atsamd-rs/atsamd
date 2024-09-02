#[doc = "Register `RESULT[%s]` reader"]
pub type R = crate::R<ResultSpec>;
#[doc = "Field `RESULT` reader - Filter Result"]
pub type ResultR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Filter Result"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new(self.bits)
    }
}
#[doc = "Filter Result\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultSpec;
impl crate::RegisterSpec for ResultSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`result::R`](R) reader structure"]
impl crate::Readable for ResultSpec {}
#[doc = "`reset()` method sets RESULT[%s]
to value 0"]
impl crate::Resettable for ResultSpec {
    const RESET_VALUE: u16 = 0;
}
