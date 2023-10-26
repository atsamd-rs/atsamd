#[doc = "Register `TSCV` reader"]
pub type R = crate::R<TSCV_SPEC>;
#[doc = "Field `TSC` reader - Timestamp Counter"]
pub type TSC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp Counter"]
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timestamp Counter Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscv::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSCV_SPEC;
impl crate::RegisterSpec for TSCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscv::R`](R) reader structure"]
impl crate::Readable for TSCV_SPEC {}
#[doc = "`reset()` method sets TSCV to value 0"]
impl crate::Resettable for TSCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
