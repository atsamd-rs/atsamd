#[doc = "Register `RR[%s]` reader"]
pub type R = crate::R<RR_SPEC>;
#[doc = "Field `CMDRESP` reader - Command Response"]
pub type CMDRESP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response"]
    #[inline(always)]
    pub fn cmdresp(&self) -> CMDRESP_R {
        CMDRESP_R::new(self.bits)
    }
}
#[doc = "Response\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RR_SPEC;
impl crate::RegisterSpec for RR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rr::R`](R) reader structure"]
impl crate::Readable for RR_SPEC {}
#[doc = "`reset()` method sets RR[%s]
to value 0"]
impl crate::Resettable for RR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
