#[doc = "Register `RESULT` reader"]
pub type R = crate::R<RESULT_SPEC>;
#[doc = "Field `RESULT` reader - Result Conversion Value"]
pub type RESULT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Result Conversion Value"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(self.bits)
    }
}
#[doc = "Result Conversion Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESULT_SPEC;
impl crate::RegisterSpec for RESULT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`result::R`](R) reader structure"]
impl crate::Readable for RESULT_SPEC {}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
