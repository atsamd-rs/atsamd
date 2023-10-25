#[doc = "Register `DTF` reader"]
pub type R = crate::R<DTF_SPEC>;
#[doc = "Field `DEFT` reader - Deferred Transmission"]
pub type DEFT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:17 - Deferred Transmission"]
    #[inline(always)]
    pub fn deft(&self) -> DEFT_R {
        DEFT_R::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Deferred Transmission Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTF_SPEC;
impl crate::RegisterSpec for DTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtf::R`](R) reader structure"]
impl crate::Readable for DTF_SPEC {}
#[doc = "`reset()` method sets DTF to value 0"]
impl crate::Resettable for DTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
